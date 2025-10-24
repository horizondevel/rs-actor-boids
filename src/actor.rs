use std::fmt::Debug;

use tracing::trace;

pub async fn run_actor<T: Actor>(mut actor: T) -> crate::Result<()> {
    while let Some(msg) = actor.recv().await {
        trace!("Received::{:?}", msg);
        actor.handle_message(msg).await?;
    }
    Ok(())
}

pub trait Actor {
    type Message: Debug + Send + Sync + 'static;

    fn recv(&mut self) -> impl std::future::Future<Output = Option<Self::Message>> + Send {
        async { None }
    }

    fn handle_message(
        &mut self,
        msg: Self::Message,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send;
}
