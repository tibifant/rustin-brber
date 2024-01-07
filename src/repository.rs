use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

pub trait Identifiable {
    fn id(&self) -> String;
}

#[async_trait]
pub trait AsyncRepository<Item: Identifiable> {
    /// Adds an item to the repository. Returns an error if the item already exists.
    async fn add(&self, item: Item) -> Result<(), String>;

    /// Retrieves all items from the repository. Returns an empty vector if there are no items.
    async fn get_all(&self) -> Result<Vec<Item>, String>;

    /// Retrieves an item by its ID. Returns None if the item does not exist.
    async fn get(&self, id: &str) -> Result<Option<Item>, String>;

    /// Saves an item to the repository (commonly used for both creating and updating items).
    async fn save(&self, item: Item) -> Result<(), String>;

    /// Updates an existing item by its ID. Returns an error if the item does not exist.
    async fn update(&self, item: Item) -> Result<(), String>;

    /// Deletes an item from the repository by its ID. Returns an error if the item does not exist.
    async fn delete(&self, id: &str) -> Result<(), String>;

    /// Deletes all items from the repository. Returns an error if the operation fails.
    async fn delete_all(&self) -> Result<(), String>;
}

#[derive(Clone)]
pub struct InMemoryRepository<Item> {
    store: Arc<RwLock<HashMap<String, Item>>>,
}

impl<Item> InMemoryRepository<Item> {
    pub fn new() -> Self {
        InMemoryRepository {
            store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl<Item: Send + Sync + Clone + 'static + Identifiable> AsyncRepository<Item>
    for InMemoryRepository<Item>
{
    async fn add(&self, item: Item) -> Result<(), String> {
        let mut store = self.store.write().await;
        store.insert(item.id(), item);
        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Item>, String> {
        let store = self.store.read().await;
        Ok(store.values().cloned().collect())
    }

    async fn get(&self, id: &str) -> Result<Option<Item>, String> {
        let store = self.store.read().await;
        Ok(store.get(id).cloned())
    }

    async fn save(&self, item: Item) -> Result<(), String> {
        let mut store = self.store.write().await;
        store.insert(item.id(), item);
        Ok(())
    }

    async fn update(&self, item: Item) -> Result<(), String> {
        let mut store = self.store.write().await;
        if store.contains_key(&item.id()) {
            store.insert(item.id(), item);
            Ok(())
        } else {
            Err(format!("Item with id {} not found", item.id()))
        }
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        let mut store = self.store.write().await;
        store.remove(id);
        Ok(())
    }

    async fn delete_all(&self) -> Result<(), String> {
        let mut store = self.store.write().await;
        store.clear();
        Ok(())
    }
}
