use crate::{
    auth::AuthManager,
    http::{HttpContext, Method},
};
use serde::{Deserialize, Serialize};
use tauri::State;

use super::Error;

#[derive(Debug, Deserialize)]
struct ScanLoginPrepareResponse {
    code: i32,
    message: String,
    data: Option<ScanLoginPrepareResponseData>,
}

#[derive(Debug, Deserialize, Serialize, specta::Type)]
pub struct ScanLoginPrepareResponseData {
    url: String,
    qrcode_key: String,
}

#[derive(Debug, Deserialize)]
struct ScanLoginResponse {
    code: i32,
    message: String,
    data: Option<ScanLoginResponseData>,
}

#[derive(Debug, Deserialize)]
struct ScanLoginResponseData {
    // refresh_token: String,
    code: i32,
    message: String,
}

#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum ScanLoginStatus {
    /// 86101
    Waiting,
    /// 86090
    Scanned,
    /// 86038
    Expired,
    /// 0
    Success,
}

pub async fn scan_login_prepare(
    auth: &State<'_, AuthManager>,
    http: &State<'_, HttpContext>,
) -> Result<ScanLoginPrepareResponseData, Error> {
    let resp = http.request(
        Method::GET,
        "https://passport.bilibili.com/x/passport-login/web/qrcode/generate",
        None,
        Some(auth.cookies()),
        None,
    ).await?;

    let resp_json: ScanLoginPrepareResponse =
        serde_json::from_slice(&resp.bytes)?;

    if let Some(data) = resp_json.data {
        Ok(data)
    } else {
        Err(Error::PrepareScanLoginFailed(
            resp_json.message,
            resp_json.code,
        ))
    }
}

pub async fn scan_login(
    auth: &State<'_, AuthManager>,
    http: &State<'_, HttpContext>,
    qrcode_key: &str,
) -> Result<ScanLoginStatus, Error> {
    let resp = http
        .request(
            Method::GET,
            "https://passport.bilibili.com/x/passport-login/web/qrcode/poll",
            Some(&[("qrcode_key", qrcode_key)]),
            Some(auth.cookies()),
            None,
        )
        .await?;

    let resp_json: ScanLoginResponse = serde_json::from_slice(&resp.bytes)?;

    if let Some(data) = resp_json.data {
        Ok(match data.code {
            0 => {
                auth.with_session(|s| {
                    for cookie in resp.cookies {
                        s.cookies.set(cookie.name, cookie.value);
                    }
                });
                ScanLoginStatus::Success
            }
            86101 => ScanLoginStatus::Waiting,
            86090 => ScanLoginStatus::Scanned,
            86038 => ScanLoginStatus::Expired,
            _ => return Err(Error::ScanLoginFailed(data.message, data.code)),
        })
    } else {
        Err(Error::ScanLoginFailed(resp_json.message, resp_json.code))
    }
}
