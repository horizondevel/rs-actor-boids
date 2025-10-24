pub trait Actor<T> {
    fn handle_message(&self, msg: T)
    -> impl std::future::Future<Output = crate::Result<()>> + Send;
}
