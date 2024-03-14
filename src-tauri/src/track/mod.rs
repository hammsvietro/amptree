use std::{
    fs::File,
    path::Path,
    sync::{Arc, Mutex},
};

use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};
use symphonia::core::{
    audio::SampleBuffer, codecs::DecoderOptions, formats::FormatOptions, io::MediaSourceStream,
    meta::MetadataOptions, probe::Hint,
};

pub struct Track {
    path: String,
}

pub struct TrackData {
    pub samples: Vec<Vec<f64>>,
    pub channel_count: usize,
    pub sample_rate: u32,
    pub time: usize,
}

impl TrackData {
    pub fn new(samples: Vec<Vec<f64>>, channel_count: usize, sample_rate: u32) -> Self {
        Self {
            samples,
            channel_count,
            sample_rate,
            time: 0,
        }
    }

    pub fn get_sample_vec(&self) -> Vec<f64> {
        self.samples
            .iter()
            .map(|sample_channel| sample_channel.get(self.time).cloned().unwrap_or(0f64))
            .collect()
    }

    pub fn skip_to(&mut self, seconds: usize) {
        self.time = seconds * (self.sample_rate as usize);
    }

    pub fn resample(&mut self, sample_rate: u32) -> anyhow::Result<()> {
        if self.sample_rate == sample_rate {
            return Ok(());
        }
        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };
        let mut resampler = SincFixedIn::<f64>::new(
            sample_rate as f64 / self.sample_rate as f64,
            2.0,
            params,
            self.samples.len(),
            1,
        )?;

        let out = resampler.process(&self.samples, None)?;
        self.samples = out;

        Ok(())
    }
}

pub type TrackDataHandle = Arc<Mutex<TrackData>>;

impl Track {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn get_data(&self) -> anyhow::Result<TrackDataHandle> {
        let file = Box::new(File::open(Path::new(&self.path)).unwrap());

        let mss = MediaSourceStream::new(file, Default::default());

        let hint = Hint::new();

        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();
        let decoder_opts: DecoderOptions = Default::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)
            .unwrap();

        let mut format = probed.format;

        let track = format.default_track().unwrap();

        let mut decoder =
            symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts)?;

        let track_id = track.id;

        let mut sample_buf = None;
        let channels = track.codec_params.channels.unwrap();
        let sample_rate = track.codec_params.sample_rate.unwrap();

        let mut buffer = vec![vec![]; channels.count()];

        loop {
            let Ok(packet) = format.next_packet() else {
                break;
            };

            // If the packet does not belong to the selected track, skip it.
            if packet.track_id() != track_id {
                continue;
            }

            // Decode the packet into audio samples, ignoring any decode errors.
            match decoder.decode(&packet) {
                Ok(audio_buf) => {
                    // The decoded audio samples may now be accessed via the audio buffer if per-channel
                    // slices of samples in their native decoded format is desired. Use-cases where
                    // the samples need to be accessed in an interleaved order or converted into
                    // another sample format, or a byte buffer is required, are covered by copying the
                    // audio buffer into a sample buffer or raw sample buffer, respectively. In the
                    // example below, we will copy the audio buffer into a sample buffer in an
                    // interleaved order while also converting to a f32 sample format.

                    // If this is the *first* decoded packet, create a sample buffer matching the
                    // decoded audio buffer format.
                    if sample_buf.is_none() {
                        // Get the audio buffer specification.
                        let spec = *audio_buf.spec();

                        // Get the capacity of the decoded buffer. Note: This is capacity, not length!
                        let duration = audio_buf.capacity() as u64;

                        // Create the f32 sample buffer.
                        sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
                    }

                    // Copy the decoded audio buffer into the sample buffer in an interleaved format.
                    if let Some(buf) = &mut sample_buf {
                        buf.copy_interleaved_ref(audio_buf);
                        for (idx, _channel) in channels.iter().enumerate() {
                            let channel_buf: Vec<f64> = buf
                                .samples()
                                .chunks(channels.count())
                                .map(|chunk| chunk.get(idx).unwrap_or(&0f32))
                                .copied()
                                .map(|x| x as f64)
                                .collect();

                            buffer[idx].extend_from_slice(&channel_buf);
                        }
                    }
                }
                Err(symphonia::core::errors::Error::DecodeError(_)) => (),
                Err(_) => break,
            }
        }
        Ok(Arc::new(Mutex::new(TrackData::new(
            buffer,
            channels.count(),
            sample_rate,
        ))))
    }
}
