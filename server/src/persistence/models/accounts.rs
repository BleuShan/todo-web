use crate::{
    persistence::prelude::*,
    prelude::*,
};
use chrono::{
    DateTime,
    Utc,
};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Account {
    id: Uuid,
    handle: String,
    first_name: Option<String>,
    last_name: Option<String>,
    created_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct AccountEmail {
    id: Uuid,
    account_id: Uuid,
    email: String,
    created_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct AccountPassword {
    id: Uuid,
    account_id: Uuid,
    password_hash: String,
    created_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}
