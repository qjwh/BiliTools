use serde::Serialize;
use tauri::{AppHandle, Manager, ipc::Channel};

use crate::error::{AppError, AppResult};

// specta does not generate types for Rust results, so we need to simulate it ourselves.
// ensure that the type generated is compatible with Result.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(tag = "status")]
pub enum InitResult {
    #[serde(rename = "ok")]
    Ok { data: () },
    #[serde(rename = "error")]
    Err { error: AppError },
}

impl From<AppResult<()>> for InitResult {
    fn from(value: AppResult<()>) -> Self {
        match value {
            Ok(_) => Self::Ok { data: () },
            Err(e) => Self::Err { error: e },
        }
    }
}

#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
pub enum InitEvent {
    Result(InitResult),
}

#[derive(Default)]
struct InitInner {
    events: Vec<InitEvent>,
    channel: Option<Channel<InitEvent>>,
}

pub struct InitState {
    inner: std::sync::Mutex<InitInner>,
}

impl InitState {
    pub fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }

    pub fn result(&self, result: AppResult<()>) {
        let event = InitEvent::Result(result.into());

        let channel = {
            let mut inner =
                self.inner.lock().unwrap_or_else(|e| e.into_inner());
            inner.events.push(event.clone());
            inner.channel.clone()
        };

        if let Some(ch) = channel {
            let _ = ch.send(event);
        }
    }

    fn start_streaming(&self, channel: Channel<InitEvent>) {
        let events = {
            let mut inner =
                self.inner.lock().unwrap_or_else(|e| e.into_inner());
            inner.channel = Some(channel.clone());
            inner.events.clone()
        };

        for e in events {
            let _ = channel.send(e);
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn init(app: AppHandle, on_event: Channel<InitEvent>) -> AppResult<()> {
    let init = app.state::<InitState>();
    init.start_streaming(on_event);
    Ok(())
}
