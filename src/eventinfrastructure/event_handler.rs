use async_trait::async_trait;

#[async_trait]
pub trait EventHandler<E> {
    async fn handle(&self, event: E);
}
