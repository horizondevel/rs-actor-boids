pub async fn run_actor<M, T: Actor<M>>(mut actor: T) -> crate::Result<()> {
    while let Some(msg) = actor.recv().await {
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
