use golem_vector::golem::vector::analytics::{
    CollectionStats, FieldStats, FlatMetadataValue, Guest as AnalyticGuest, VectorError,
};

use crate::QdrantComponent;

impl AnalyticGuest for QdrantComponent {
    fn get_collection_stats(
        collection: String,
        namespace: Option<String>,
    ) -> Result<CollectionStats, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "get_collection_stats is not supported by Qdrant".to_string(),
        ))
    }
    fn get_field_distribution(
        collection: String,
        field: String,
        limit: Option<u32>,
        namespace: Option<String>,
    ) -> Result<Vec<(FlatMetadataValue, u64)>, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "get_field_distribution is not supported by Qdrant".to_string(),
        ))
    }
    fn get_field_stats(
        collection: String,
        field: String,
        namespace: Option<String>,
    ) -> Result<FieldStats, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "get_field_stats is not supported by Qdrant".to_string(),
        ))
    }
}
