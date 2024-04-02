use serde::Deserialize;

#[cfg(not(debug_assertions))]
use std::fmt;

#[derive(Default, Deserialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct User {
    pub username: String,
    pub password: String,
    pub ip: Option<String>,
    pub if_name: Option<String>,
}

impl User {
    pub fn new(username: String, password: String, ip: String) -> Self {
        Self {
            username,
            password,
            ip: Some(ip),
            if_name: None,
        }
    }

    pub fn new_with_if_name(username: String, password: String, if_name: String) -> Self {
        Self {
            username,
            password,
            ip: None,
            if_name: Some(if_name),
        }
    }
}

#[cfg(not(debug_assertions))]
impl fmt::Debug for User {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let temp = format!("__MASKED__,length: {}", self.password.len());
        let password_shadow = match self.password.is_empty() {
            true => &self.password,
            false => &temp
        };
        fmt.debug_struct("User")
            .field("username", &self.username)
            .field("password", &password_shadow)
            .field("ip", &self.ip)
            .field("if_name", &self.if_name)
            .finish()
    }
}