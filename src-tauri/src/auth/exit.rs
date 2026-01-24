use serde::Deserialize;
use tauri::State;

use crate::{
    auth::AuthManager,
    http::{HttpContext, Method},
};

use super::Error;

#[derive(Debug, Deserialize)]
struct ExitLoginResponse {
    code: i32,
    message: String,
}

pub async fn exit_login(
    auth: &State<'_, AuthManager>,
    http: &State<'_, HttpContext>,
) -> Result<(), Error> {
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

    let resp_json: ExitLoginResponse = serde_json::from_slice(&resp.bytes)?;

    if resp_json.code != 0 {
        return Err(Error::ExitLoginFailed(resp_json.message, resp_json.code));
    }
    Ok(())
}
