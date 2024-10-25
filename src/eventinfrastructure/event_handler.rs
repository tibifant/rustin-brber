use async_trait::async_trait;

#[async_trait]
pub trait EventHandler<E> {
    fn handle(&mut self, event: E);
}
