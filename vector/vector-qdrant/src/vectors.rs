use golem_vector::golem::vector::vectors::{
    BatchResult, FilterExpression, Guest as VectorGuest, Id, ListResponse, Metadata, VectorData,
    VectorError, VectorRecord,
};

use crate::QdrantComponent;

impl VectorGuest for QdrantComponent {
    fn count_vectors(
        collection: String,
        filter: Option<FilterExpression>,
        namespace: Option<String>,
    ) -> Result<u64, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "count_vectors is not supported by Qdrant".to_string(),
        ))
    }
    fn delete_by_filter(
        collection: String,
        filter: FilterExpression,
        namespace: Option<String>,
    ) -> Result<u32, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "delete_by_filter is not supported by Qdrant".to_string(),
        ))
    }
    fn delete_namespace(collection: String, namespace: String) -> Result<u32, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "delete_namespace is not supported by Qdrant".to_string(),
        ))
    }
    fn delete_vectors(
        collection: String,
        ids: Vec<Id>,
        namespace: Option<String>,
    ) -> Result<u32, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "delete_vectors is not supported by Qdrant".to_string(),
        ))
    }
    fn get_vector(
        collection: String,
        id: Id,
        namespace: Option<String>,
    ) -> Result<Option<VectorRecord>, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "get_vector is not supported by Qdrant".to_string(),
        ))
    }
    fn get_vectors(
        collection: String,
        ids: Vec<Id>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<Vec<VectorRecord>, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "get_vectors is not supported by Qdrant".to_string(),
        ))
    }
    fn list_vectors(
        collection: String,
        namespace: Option<String>,
        filter: Option<FilterExpression>,
        limit: Option<u32>,
        cursor: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<ListResponse, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "list_vectors is not supported by Qdrant".to_string(),
        ))
    }
    fn update_vector(
        collection: String,
        id: Id,
        vector: Option<VectorData>,
        metadata: Option<Metadata>,
        namespace: Option<String>,
        merge_metadata: Option<bool>,
    ) -> Result<(), VectorError> {
        Err(VectorError::UnsupportedFeature(
            "update_vector is not supported by Qdrant".to_string(),
        ))
    }
    fn upsert_vector(
        collection: String,
        id: Id,
        vector: VectorData,
        metadata: Option<Metadata>,
        namespace: Option<String>,
    ) -> Result<(), VectorError> {
        Err(VectorError::UnsupportedFeature(
            "upsert_vector is not supported by Qdrant".to_string(),
        ))
    }
    fn upsert_vectors(
        collection: String,
        vectors: Vec<VectorRecord>,
        namespace: Option<String>,
    ) -> Result<BatchResult, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "upsert_vectors is not supported by Qdrant".to_string(),
        ))
    }
}
