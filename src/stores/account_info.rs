use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Store, Properties)]
pub struct AccountInfo {
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub user_id: Option<u32>,
    pub settings: (),
}

impl Default for AccountInfo{
    fn default() -> Self {
        Self { user_name: Some("Garrett".to_owned()), password: None, user_id: None, settings: () }
    }
}
