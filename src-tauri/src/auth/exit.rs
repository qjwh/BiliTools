use serde::Deserialize;
use tauri::State;

use crate::{
    auth::AuthManager, error::AppError, http::{HttpContext, Method}
};

#[derive(Debug, thiserror::Error)]
pub enum ExitError {
    #[error("exit login http failed")]
    Http(#[from] crate::http::Error),

    #[error("exit login failed: {0} ({1})")]
    ExitFailed(String, i32),
}

impl From<ExitError> for AppError {
    fn from(value: ExitError) -> Self {
        let (source, reason) = match value {
            ExitError::Http(r) => return r.into(),
            ExitError::ExitFailed(_, _) => ("auth.exit", "exit_login_failed"),
        };
        Self {
            source,
            reason,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ExitLoginResponse {
    code: i32,
    message: String,
}

pub async fn exit_login(
    auth: &State<'_, AuthManager>,
    http: &State<'_, HttpContext>,
) -> Result<(), ExitError> {
    let Some(csrf) = auth.csrf_token() else {
        return Ok(());
    };

    let resp = http
        .request(
            Method::POST,
            "https://passport.bilibili.com/login/exit/v2",
            Some(&[("biliCSRF", &csrf)]),
            Some(auth.cookies()),
            None,
        )
        .await?;

    let resp_json: ExitLoginResponse = resp.json()?;

    if resp_json.code != 0 {
        return Err(ExitError::ExitFailed(resp_json.message, resp_json.code));
    }
    Ok(())
}
