//! Data backends are to implement `trait Datastore`.
//! To avoid blocking, backends are to be async.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait Datastore {
    async fn get(&self, key: &str) -> Option<Vec<u8>>;
    async fn set(&self, key: &str, value: [u8]);
    async fn delete(&self, key: &str);

    async fn get_collections(&self) -> Vec<Collection>;
    //async fn get_collections_iter(&self) -> Vec<Collection>;
    async fn get_images_in_collection(&self, collection: String) -> Vec<Image>;
    //async fn get_images_in_collection_iter(&self) -> Vec<Image>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Collection {
    pub folder: String,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    pub path: String,
    pub collection: String,
    pub size: (u32, u32),
    pub tags: Vec<String>,
    pub description: String,
}
