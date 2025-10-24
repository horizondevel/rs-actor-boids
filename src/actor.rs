use std::fmt::Debug;

use tracing::trace;

pub async fn run_actor<M: Debug, T: Actor<M>>(mut actor: T) -> crate::Result<()> {
    while let Some(msg) = actor.recv().await {
        trace!("Received::{:?}", msg);
        actor.handle_message(msg).await?;
    }
    Ok(())
}

pub trait Actor<T> {
    fn recv(&mut self) -> impl std::future::Future<Output = Option<T>> + Send {
        async { None }
    }
    fn handle_message(
        &mut self,
        msg: T,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send;
}
