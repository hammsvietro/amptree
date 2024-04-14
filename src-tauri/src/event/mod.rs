use tauri::{AppHandle, Manager};

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
        self.emit_all(event, data)?;
        Ok(())
    }
}
