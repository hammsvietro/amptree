use std::{collections::VecDeque, fmt::Debug, fs::File, path::Path};

use symphonia::{
    core::{
        audio::SampleBuffer,
        codecs::{Decoder, DecoderOptions},
        formats::{FormatOptions, FormatReader, SeekMode},
        io::MediaSourceStream,
        meta::{MetadataOptions, StandardTagKey},
        probe::{Hint, Probe, ProbeResult},
        units::{Time, TimeBase},
    },
    default::get_probe,
};

#[derive(Debug)]
pub struct AudioFile {
    path: String,
}

const MINIMUM_FRAMES_IN_BUFFER_COUNT: usize = 1028;

#[derive(Debug, serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct AudioPlaybackStatus {
    pub percentage: f64,
    pub total_duration_secs: f64,
    pub played_secs: f64,
    pub volume: f64,
}

#[derive(Debug, serde::Serialize, Clone, Default)]
pub(super) struct AudioMetadata {
    pub artist: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub year: Option<usize>,
    pub genre: Option<String>,
    pub track_number: Option<usize>,
}

pub(super) struct AudioPlaybackMetadata {
    pub channel_count: usize,
    pub sample_rate: u32,
    time_base: TimeBase,
    frames_count: u64,
    track_id: u32,
}

impl AudioPlaybackMetadata {
    pub fn new(
        channel_count: usize,
        sample_rate: u32,
        time_base: TimeBase,
        frames_count: u64,
        track_id: u32,
    ) -> Self {
        Self {
            channel_count,
            sample_rate,
            time_base,
            frames_count,
            track_id,
        }
    }
}

pub(super) struct AudioHandle {
    pub(super) track_metadata: AudioPlaybackMetadata,
    volume: f64,
    time: u64,
    samples: Vec<VecDeque<f64>>,
    reader: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
}

impl AudioHandle {
    pub(super) fn new(
        reader: Box<dyn FormatReader>,
        decoder: Box<dyn Decoder>,
        track_metadata: AudioPlaybackMetadata,
        volume: f64,
    ) -> Self {
        let channel_count = track_metadata.channel_count;
        Self {
            track_metadata,
            reader,
            volume,
            decoder,
            time: 0,
            samples: vec![VecDeque::new(); channel_count],
        }
    }

    pub(super) fn get_sample_buffer(&mut self) -> anyhow::Result<Vec<f64>> {
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

    pub(super) fn update_volume(&mut self, volume: f64) {
        self.volume = volume;
    }

    pub(super) fn get_status(&self) -> AudioPlaybackStatus {
        AudioPlaybackStatus {
            percentage: self.get_percentage(),
            total_duration_secs: self.get_duration().seconds as f64,
            played_secs: self.get_played_time().seconds as f64,
            volume: self.volume,
        }
    }

    pub(super) fn seek(&mut self, seconds: usize) -> anyhow::Result<()> {
        let mut time = self.get_duration();
        time.seconds = seconds as u64;
        self.time = self.track_metadata.time_base.calc_timestamp(time);
        self.reader.seek(
            SeekMode::Accurate,
            symphonia::core::formats::SeekTo::Time {
                time,
                track_id: None,
            },
        )?;
        Ok(())
    }

    pub(super) fn has_finished(&self) -> bool {
        self.time >= self.track_metadata.frames_count
    }

    pub(super) fn increment_time(&mut self) {
        self.time += 1;
    }

    pub(super) fn get_percentage(&self) -> f64 {
        self.time as f64 / self.track_metadata.frames_count as f64
    }

    fn get_played_time(&self) -> Time {
        self.track_metadata.time_base.calc_time(self.time)
    }

    pub fn get_duration(&self) -> Time {
        self.track_metadata
            .time_base
            .calc_time(self.track_metadata.frames_count)
    }

    fn needs_to_fetch_more_samples(&self) -> bool {
        self.samples[0].len() < MINIMUM_FRAMES_IN_BUFFER_COUNT && !self.has_finished()
    }

    fn fetch_samples(&mut self) -> anyhow::Result<()> {
        while let Ok(packet) = self.reader.next_packet() {
            if packet.track_id() != self.track_metadata.track_id {
                continue;
            }

            match self.decoder.decode(&packet) {
                Ok(audio_buf) => {
                    let spec = *audio_buf.spec();
                    let duration = audio_buf.capacity() as u64;
                    let mut sample_buf = SampleBuffer::<f64>::new(duration, spec);

                    sample_buf.copy_interleaved_ref(audio_buf);

                    for channel_idx in 0..self.track_metadata.channel_count {
                        let mut channel_buf: VecDeque<f64> = sample_buf
                            .samples()
                            .chunks(self.track_metadata.channel_count)
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

pub(super) trait AudioSource {
    /// Returns the metadata of the audio
    ///
    /// Use the `AudioHandle` to fetch samples, seek, playback status, etc...
    fn get_handle(&self, volume: f64) -> anyhow::Result<AudioHandle>;

    /// Returns the metadata of the audio
    fn get_metadata(&self) -> anyhow::Result<AudioMetadata>;
}
impl AudioSource for AudioFile {
    fn get_metadata(&self) -> anyhow::Result<AudioMetadata> {
        let mut probe_result = self.probe()?;
        let mut metadata = AudioMetadata::default();

        if let Some(mut metadata_revision) = probe_result.metadata.get() {
            if let Some(metadata_revision) = metadata_revision.skip_to_latest() {
                for tag in metadata_revision.tags() {
                    match tag.std_key {
                        Some(StandardTagKey::Artist) => {
                            metadata.artist = Some(tag.value.to_string())
                        }
                        Some(StandardTagKey::TrackTitle) => {
                            metadata.title = Some(tag.value.to_string())
                        }
                        Some(StandardTagKey::Album) => metadata.album = Some(tag.value.to_string()),
                        Some(StandardTagKey::Date) => {
                            metadata.year = tag.value.to_string().parse().ok()
                        }
                        Some(StandardTagKey::Genre) => metadata.genre = Some(tag.value.to_string()),
                        Some(StandardTagKey::TrackNumber) => {
                            metadata.track_number = tag.value.to_string().parse().ok()
                        }
                        _ => (),
                    }
                }
            } else {
                println!("no latest metadata revision =(");
            }
        } else {
            println!("no global metadata tags =(");
        }

        if let Some(metadata_revision) = probe_result.format.metadata().current() {
            println!("{:?}", metadata_revision.tags());
        } else {
            println!("no format metadata tags =(");
        }

        Ok(AudioMetadata {
            artist: None,
            title: None,
            album: None,
            year: None,
            genre: None,
            track_number: None,
        })
    }

    fn get_handle(&self, volume: f64) -> anyhow::Result<AudioHandle> {
        let probe_result = self.probe()?;

        let format = probe_result.format;

        let track = format.default_track().unwrap();

        let decoder =
            symphonia::default::get_codecs().make(&track.codec_params, &Default::default())?;

        let track_id = track.id;

        let channels = track.codec_params.channels.unwrap();
        let sample_rate = track.codec_params.sample_rate.unwrap();
        let time_base = track.codec_params.time_base.unwrap();
        let frames_count = track.codec_params.n_frames.unwrap();

        let track_information = AudioPlaybackMetadata::new(
            channels.count(),
            sample_rate,
            time_base,
            frames_count,
            track_id,
        );
        Ok(AudioHandle::new(format, decoder, track_information, volume))
    }
}

impl AudioFile {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    fn probe(&self) -> anyhow::Result<ProbeResult> {
        let file = Box::new(File::open(Path::new(&self.path))?);

        let mss = MediaSourceStream::new(file, Default::default());

        let hint = Hint::new();

        let format_opts: FormatOptions = Default::default();
        let metadata_opts: MetadataOptions = Default::default();

        Ok(get_probe().format(&hint, mss, &format_opts, &metadata_opts)?)
    }
}
