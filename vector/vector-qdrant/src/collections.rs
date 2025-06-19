
use crate::QdrantComponent;
use golem_vector::golem::vector::collections::{
    CollectionInfo, Guest as CollectionGuest, VectorError,
};

impl CollectionGuest for QdrantComponent {
    fn collection_exists(name: String) -> Result<bool, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "collection_exists is not supported by Qdrant".to_string(),
        ))
    }
    fn delete_collection(name: String) -> Result<(), VectorError> {
        Err(VectorError::UnsupportedFeature(
            "delete_collection is not supported by Qdrant".to_string(),
        ))
    }
    fn get_collection(name: String) -> Result<CollectionInfo, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "get_collection is not supported by Qdrant".to_string(),
        ))
    }
    fn list_collections() -> Result<Vec<CollectionInfo>, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "list_collections is not supported by Qdrant".to_string(),
        ))
    }
    fn update_collection(
        name: String,
        description: Option<String>,
        metadata: Option<golem_vector::golem::vector::collections::Metadata>,
    ) -> Result<CollectionInfo, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "update_collection is not supported by Qdrant".to_string(),
        ))
    }
    fn upsert_collection(
        name: String,
        description: Option<String>,
        dimension: u32,
        metric: golem_vector::golem::vector::collections::DistanceMetric,
        index_config: Option<golem_vector::golem::vector::collections::IndexConfig>,
        metadata: Option<golem_vector::golem::vector::collections::Metadata>,
    ) -> Result<CollectionInfo, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "upsert_collection is not supported by Qdrant".to_string(),
        ))
    }
}
