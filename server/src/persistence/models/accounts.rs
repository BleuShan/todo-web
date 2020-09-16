use crate::{
    persistence::{
        prelude::*,
        query_as,
        PgPool,
        Postgres,
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
    pub async fn new<'c, ExecutorType, HandleType>(
        executor: ExecutorType,
        handle: HandleType,
    ) -> SQLResult<Self>
    where
        ExecutorType: Executor<'c, Database = Postgres>,
        String: From<HandleType>,
    {
        query_as!(
            Account,
            r#"INSERT INTO accounts (handle) VALUES ($1) RETURNING *"#,
            String::from(handle)
        )
        .fetch_one(executor)
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
    use once_cell::sync::OnceCell;
    use std::env;

    static CURRENT_POOL: OnceCell<PgPool> = OnceCell::new();

    fn test_pool<'a>() -> Result<&'a PgPool> {
        CURRENT_POOL.get_or_try_init(|| {
            dotenv::dotenv().ok();
            PgPool::connect_lazy(env::var("DATABASE_TEST_URL")?.as_str()).map_err(|e| e.into())
        })
    }

    #[actix_web::rt::test]
    async fn new_account() {
        let pool = test_pool().expect("Couldn't instantiate test pool");
        let mut transaction = pool.begin().await.expect("Fail to begin transaction");
        let account = Account::new(&mut transaction, "handle")
            .await
            .expect("Account");

        assert_eq!(account.handle, "handle");

        transaction
            .rollback()
            .await
            .expect("Failed to cleanup test")
    }
}
