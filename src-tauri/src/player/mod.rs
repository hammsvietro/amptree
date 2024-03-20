use crate::track::Track;

pub enum PlayerCommand {
    Play(Track),
    Resume,
    Pause,
    Seek(usize),
}
