use crate::{event::rpc, shared::unix_now};
use serde::Deserialize;
use tauri::{AppHandle, State};

use super::{AuthManager, Error};
use crate::http::{HttpContext, Method};

pub async fn ensure_base(
    auth: &State<'_, AuthManager>,
    http: &State<'_, HttpContext>,
) -> Result<(), Error> {
    if auth.with_session(|s| s.cookies.contains("buvid3")) {
        return Ok(());
    }

    let resp = http
        .request(Method::GET, "https://www.bilibili.com", None, None, None)
        .await?;

    auth.with_session(|s| {
        for cookie in resp.cookies {
            s.cookies.set(cookie.name, cookie.value);
        }
    });

    Ok(())
}

#[derive(Debug, Deserialize)]
struct BuvidResponse {
    code: i32,
    message: String,
    data: Option<BuvidResponseData>,
}

#[derive(Debug, Deserialize)]
struct BuvidResponseData {
    b_3: String,
    b_4: String,
}

pub async fn ensure_buvid(
    auth: &State<'_, AuthManager>,
    http: &State<'_, HttpContext>,
) -> Result<(), Error> {
    if auth.with_session(|s| {
        s.cookies.contains("buvid3") && s.cookies.contains("buvid4")
    }) {
        return Ok(());
    }

    let resp = http
        .request(
            Method::GET,
            "https://api.bilibili.com/x/frontend/finger/spi",
            None,
            Some(auth.cookies()),
            None,
        )
        .await?;

    let resp_json: BuvidResponse = serde_json::from_slice(&resp.bytes)?;

    if let Some(data) = resp_json.data {
        auth.with_session(|s| {
            if !s.cookies.contains("buvid3") {
                s.cookies.set("buvid3", &data.b_3)
            }
            if !s.cookies.contains("buvid4") {
                s.cookies.set("buvid4", &data.b_4)
            }
        })
    } else {
        return Err(Error::GetBuvid4Failed(resp_json.message, resp_json.code));
    }
    Ok(())
}

pub async fn ensure_uuid(
    app: &AppHandle,
    auth: &State<'_, AuthManager>,
) -> Result<(), Error> {
    if auth.with_session(|s| s.cookies.contains("_uuid")) {
        return Ok(());
    }

    let uuid: String = rpc::request(
        app,
        rpc::FrontendRequestPayload::BiliUuid {
            now: (unix_now().as_millis() % 100_000).to_string(),
        },
    )
    .await?;

    auth.with_session(|s| {
        s.cookies.set("_uuid", &uuid);
    });

    Ok(())
}

#[derive(Debug, Deserialize)]
struct BiliTicketResponse {
    code: i32,
    message: String,
    data: Option<BiliTicketResponseData>,
}

#[derive(Debug, Deserialize)]
struct BiliTicketResponseData {
    ticket: String,
}

pub async fn ensure_bili_ticket(
    app: &AppHandle,
    auth: &State<'_, AuthManager>,
    http: &State<'_, HttpContext>,
) -> Result<(), Error> {
    if auth.with_session(|s| s.cookies.contains("bili_ticket")) {
        return Ok(());
    }

    let csrf = auth.csrf_token().unwrap_or_default();

    let e = unix_now().as_secs();
    let t = "YhxToH[2q";

    let hexsign: String = rpc::request(
        app,
        rpc::FrontendRequestPayload::BiliTicketHexSign {
            message: format!("ts{e}"),
            key: t.into(),
        },
    )
    .await?;

    let resp = http.request(
        Method::POST,
        "https://api.bilibili.com/bapis/bilibili.api.ticket.v1.Ticket/GenWebTicket",
        Some(&[
            ("key_id", "ec02"),
            ("hexsign", &hexsign),
            ("context[ts]", &e.to_string()),
            ("csrf", &csrf),
        ]),
        Some(auth.cookies()),
        None,
    ).await?;

    let resp_json: BiliTicketResponse = serde_json::from_slice(&resp.bytes)?;

    if let Some(data) = resp_json.data {
        auth.with_session(|s| s.cookies.set("bili_ticket", &data.ticket));
    } else {
        return Err(Error::GenWebTicketFailed(
            resp_json.message,
            resp_json.code,
        ));
    }

    Ok(())
}
