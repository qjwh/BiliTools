use serde::{Deserialize, Serialize};
use tauri::State;

use super::Error;

use crate::{auth::AuthManager, http::{Body, HttpContext, Method}};

#[derive(Debug, Deserialize)]
struct SmsCaptchaResponse {
    code: i32,
    message: String,
    data: Option<SmsCaptchaResponseData>
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct SmsCaptchaResponseData {
    token: String,
    geetest: SmsCaptchaResponseGeetest,
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct SmsCaptchaResponseGeetest {
    gt: String,
    challenge: String,
}

pub async fn sms_login_prepare(
    auth: &State<'_, AuthManager>,
    http: &State<'_, HttpContext>,
) -> Result<SmsCaptchaResponseData, Error> {
    let resp = http
        .request(
            Method::GET,
            "https://passport.bilibili.com/x/passport-login/captcha",
            None,
            Some(auth.cookies()),
            None
        )
        .await?;

    let resp_json: SmsCaptchaResponse = serde_json::from_slice(&resp.bytes)?;

    if let Some(data) = resp_json.data {
        Ok(data)
    } else {
        Err(Error::PrepareSmsLoginFailed(resp_json.message, resp_json.code))
    }
}

#[derive(Debug, Deserialize)]
struct SmsSendCodeResponse {
    code: i32,
    message: String,
    data: Option<SmsSendCodeResponseData>
}

#[derive(Debug, Serialize, Deserialize, specta::Type)]
pub struct SmsSendCodeResponseData {
    captcha_key: String,
}

pub async fn sms_login_send_code(
    auth: &State<'_, AuthManager>,
    http: &State<'_, HttpContext>,
    cid: &str,
    tel: &str,
    token: &str,
    gt: &str,
    challenge: &str,
) -> Result<SmsSendCodeResponseData, Error> {
    let resp = http
        .request(
            Method::POST,
            "https://passport.bilibili.com/x/passport-login/web/sms/send",
            None,
            Some(auth.cookies()),
            Some(Body::Form(&[
                ("cid", cid),
                ("tel", tel),
                ("token", token),
                ("gt", gt),
                ("challenge", challenge),
            ]))
        )
        .await?;

    let resp_json: SmsSendCodeResponse = serde_json::from_slice(&resp.bytes)?;

    if let Some(data) = resp_json.data {
        Ok(data)
    } else {
        Err(Error::SmsLoginSendCodeFailed(resp_json.message, resp_json.code))
    }
}

#[derive(Debug, Deserialize)]
struct SmsLoginResponse {
    code: i32,
    message: String,
    data: Option<SmsLoginResponseData>
}

#[derive(Debug, Deserialize)]
struct SmsLoginResponseData {
    status: i32,
    message: String,
    // refresh_token: String
}

pub async fn sms_login(
    auth: &State<'_, AuthManager>,
    http: &State<'_, HttpContext>,
    cid: &str,
    tel: &str,
    code: &str,
    captcha_key: &str,
) -> Result<(), Error> {
    let resp = http
        .request(
            Method::POST,
            "https://passport.bilibili.com/x/passport-login/web/login/sms",
            None,
            Some(auth.cookies()),
            Some(Body::Form(&[
                ("cid", cid),
                ("tel", tel),
                ("code", code),
                ("captcha_key", captcha_key),
            ])),
        )
        .await?;

    let resp_json: SmsLoginResponse = serde_json::from_slice(&resp.bytes)?;

    if let Some(data) = resp_json.data {
        Ok(match data.status {
            0 => {
                auth.with_session(|s| {
                    for cookie in resp.cookies {
                        s.cookies.set(cookie.name, cookie.value);
                    }
                });
                ()
            }
            _ => return Err(Error::SmsLoginFailed(data.message, data.status))
        })
    } else {
        Err(Error::SmsLoginFailed(resp_json.message, resp_json.code))
    }
}
