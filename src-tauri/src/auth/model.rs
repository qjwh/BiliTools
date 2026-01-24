use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthSession {
    pub cookies: CookieJar,
    pub expires_at: Option<SystemTime>,
    pub last_verified_at: Option<SystemTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CookieJar {
    inner: Vec<Cookie>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
}

impl CookieJar {
    pub fn get(&self, name: &str) -> Option<&str> {
        self.inner
            .iter()
            .find(|c| c.name == name)
            .map(|c| c.value.as_str())
    }

    pub fn set<T>(&mut self, name: T, value: T)
    where
        T: Into<String>,
    {
        let name = name.into();
        let value = value.into();

        if let Some(cookie) = self.inner.iter_mut().find(|c| c.name == name) {
            cookie.value = value;
        } else {
            self.inner.push(Cookie { name, value });
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.inner.iter().any(|c| c.name == name)
    }

    pub fn to_cookie_header(&self) -> String {
        self.inner
            .iter()
            .map(|c| format!("{}={}", c.name, c.value))
            .collect::<Vec<_>>()
            .join("; ")
    }
}
