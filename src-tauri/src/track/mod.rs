use std::{collections::VecDeque, fs::File, path::Path};

use serde::Serialize;

use symphonia::core::{
    audio::SampleBuffer,
    codecs::{Decoder, DecoderOptions},
    formats::{FormatOptions, FormatReader, SeekMode},
    io::MediaSourceStream,
    meta::MetadataOptions,
    probe::Hint,
    units::{Time, TimeBase},
};

const MINIMUM_FRAMES_IN_BUFFER_COUNT: usize = 1028;

#[derive(Debug)]
pub struct Track {
    path: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrackStatus {
    pub duration_seconds: usize,
    pub played_time: usize,
}

pub struct TrackHandle {
    pub channel_count: usize,
    pub sample_rate: u32,
    volume: f64,
    time: u64,
    samples: Vec<VecDeque<f64>>,
    reader: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    time_base: TimeBase,
    frames_count: u64,
    track_id: u32,
}

impl TrackHandle {
    pub fn new(
        reader: Box<dyn FormatReader>,
        decoder: Box<dyn Decoder>,
        volume: f64,
        track_id: u32,
        sample_rate: u32,
        channel_count: usize,
        time_base: TimeBase,
        frames_count: u64,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            channel_count,
            sample_rate,
            reader,
            volume,
            decoder,
            time_base,
            frames_count,
            track_id,
            time: 0,
            samples: vec![VecDeque::new(); channel_count],
        })
    }

    pub fn get_sample_buffer(&mut self) -> anyhow::Result<Vec<f64>> {
        if self.needs_to_fetch_more_samples() {
            self.fetch_samples()?;
        }
        let mut buf = Vec::new();
        for channel_buffer in self.samples.iter_mut() {
            if let Some(sample) = channel_buffer.pop_front() {
                buf.push(sample * (self.volume));
            }
        }
        Ok(buf)
    }

    pub fn update_volume(&mut self, volume: f64) {
        self.volume = volume;
    }

    pub fn get_volume(&self) -> f64 {
        self.volume
    }

    pub fn seek(&mut self, seconds: usize) -> anyhow::Result<()> {
        let mut time = self.get_duration();
        time.seconds = seconds as u64;
        // TODO: update self.time here
        self.time = self.time_base.calc_timestamp(time.clone());
        self.reader.seek(
            SeekMode::Accurate,
            symphonia::core::formats::SeekTo::Time {
                time,
                track_id: None,
            },
        )?;
        Ok(())
    }

    pub fn has_finished(&self) -> bool {
        self.time >= self.frames_count
    }

    pub fn increment_time(&mut self) {
        self.time += 1;
    }

    pub fn get_status(&self) -> TrackStatus {
        TrackStatus {
            duration_seconds: self.get_duration().seconds as usize,
            played_time: self.get_played_time().seconds as usize,
        }
    }

    pub fn get_percentage(&self) -> f64 {
        self.time as f64 / self.frames_count as f64
    }

    fn get_played_time(&self) -> Time {
        self.time_base.calc_time(self.time)
    }

    fn get_duration(&self) -> Time {
        self.time_base.calc_time(self.frames_count)
    }

    fn needs_to_fetch_more_samples(&self) -> bool {
        self.samples[0].len() < MINIMUM_FRAMES_IN_BUFFER_COUNT && !self.has_finished()
    }

    fn fetch_samples(&mut self) -> anyhow::Result<()> {
        while let Ok(packet) = self.reader.next_packet() {
            if packet.track_id() != self.track_id {
                continue;
            }

            match self.decoder.decode(&packet) {
                Ok(audio_buf) => {
                    let spec = *audio_buf.spec();
                    let duration = audio_buf.capacity() as u64;
                    let mut sample_buf = SampleBuffer::<f64>::new(duration, spec);

                    sample_buf.copy_interleaved_ref(audio_buf);

                    for channel_idx in 0..self.channel_count {
                        let mut channel_buf: VecDeque<f64> = sample_buf
                            .samples()
                            .chunks(self.channel_count)
                            .map(|chunk| chunk.get(channel_idx).unwrap_or(&0f64))
                            .copied()
                            .collect();

                        self.samples[channel_idx].append(&mut channel_buf);
                    }
                    break;
                }
                Err(symphonia::core::errors::Error::DecodeError(_)) => (),
                Err(_) => break,
            }
        }
        Ok(())
    }
}

impl Track {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn get_track_handle(&self, volume: f64) -> anyhow::Result<TrackHandle> {
        let file = Box::new(File::open(Path::new(&self.path)).unwrap());

        let mss = MediaSourceStream::new(file, Default::default());

        let hint = Hint::new();

        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();
        let decoder_opts: DecoderOptions = Default::default();

        let mut probed =
            symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;

        if let Some(metadata_revision) = probed.format.metadata().current() {
            println!("{:?}", metadata_revision.tags());
        } else {
            println!("no tags =(");
        }

        let format = probed.format;

        let track = format.default_track().unwrap();

        let decoder = symphonia::default::get_codecs().make(&track.codec_params, &decoder_opts)?;

        let track_id = track.id;

        let channels = track.codec_params.channels.unwrap();
        let sample_rate = track.codec_params.sample_rate.unwrap();
        let time_base = track.codec_params.time_base.unwrap();
        let frames_count = track.codec_params.n_frames.unwrap();

        let track_data = TrackHandle::new(
            format,
            decoder,
            volume,
            track_id,
            sample_rate,
            channels.count(),
            time_base,
            frames_count,
        )?;

        Ok(track_data)
    }
}
