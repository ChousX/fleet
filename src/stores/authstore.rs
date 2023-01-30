use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Debug, PartialEq, Clone, Default, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Store, Default, PartialEq, Clone)]
pub struct AuthStore {
    pub user: Option<User>,
    pub token: Option<String>,
}
