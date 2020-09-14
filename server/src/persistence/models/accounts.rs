use crate::{
    persistence::{
        prelude::*,
        query_as,
        PgPool,
        SQLResult,
    },
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

impl Account {
    pub async fn new(pool: &PgPool, handle: String) -> SQLResult<Self> {
        query_as!(
            Account,
            r#"INSERT INTO accounts (handle) VALUES ($1) RETURNING *"#,
            handle
        )
        .fetch_one(pool)
        .await
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[actix_web::rt::test]
    async fn test() {}
}
