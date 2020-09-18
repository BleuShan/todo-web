use crate::{
    persistence::prelude::*,
    prelude::*,
    Configuration,
};
use async_std::task::{
    Context as TaskContext,
    Poll,
};
use sqlx::{
    migrate,
    migrate::MigrateError,
};

pub static MIGRATOR: migrate::Migrator = migrate!("./migrations");

#[derive(Debug)]
pub struct Store {
    pool: PgPool,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            pool: Configuration::load().database().into(),
        }
    }
}

impl Actor for Store {
    type Context = Context<Self>;
}

impl Supervised for Store {}

impl SystemService for Store {
    #[instrument(name = "Store:service_started", skip(self, ctx))]
    fn service_started(&mut self, ctx: &mut Context<Self>) {
        info!("updating database.");
        ctx.wait(StoreMigration::default())
    }
}

#[pin_project]
#[derive(Default)]
#[repr(transparent)]
struct StoreMigration {
    #[pin]
    inner: Option<LocalBoxFuture<'static, Result<(), MigrateError>>>,
}

impl ActorFuture for StoreMigration {
    type Output = ();

    type Actor = Store;

    #[instrument(name = "StoreMigration", skip(self, store, ctx, task))]
    fn poll(
        mut self: Pin<&mut Self>,
        store: &mut Self::Actor,
        ctx: &mut <Self::Actor as Actor>::Context,
        task: &mut TaskContext<'_>,
    ) -> Poll<Self::Output> {
        if self.inner.is_none() {
            let pool = store.pool.clone();
            self.inner.replace(
                async move { MIGRATOR.run(&pool).instrument(info_span!("query")).await }
                    .boxed_local(),
            );
        }

        let this = self.project();

        if let Some(mut value) = this.inner.as_pin_mut() {
            match value.poll_unpin(task) {
                Poll::Ready(Ok(_)) => {
                    info!("update completed.");
                    Poll::Ready(())
                }
                Poll::Ready(Err(error)) => {
                    error!("{}", error);
                    ctx.stop();
                    Poll::Ready(())
                }

                Poll::Pending => Poll::Pending,
            }
        } else {
            unreachable!()
        }
    }
}
