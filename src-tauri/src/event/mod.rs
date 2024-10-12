use tauri::{AppHandle, Emitter};

pub trait EventEmitter {
    fn emit_event<T>(&self, event: &str, data: T) -> anyhow::Result<()>
    where
        T: serde::Serialize + Clone;
}

impl EventEmitter for AppHandle {
    fn emit_event<T>(&self, event: &str, data: T) -> anyhow::Result<()>
    where
        T: serde::Serialize + Clone,
    {
        self.emit(event, data)?;
        Ok(())
    }
}
