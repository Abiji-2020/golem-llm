use crate::QdrantComponent;
use golem_vector::golem::vector::namespaces::{
    Guest as NamespaceGuest, NamespaceInfo, VectorError,
};

impl NamespaceGuest for QdrantComponent {
    fn delete_namespace(collection: String, namespace: String) -> Result<(), VectorError> {
        Err(VectorError::UnsupportedFeature(
            "delete_namespace is not supported by Qdrant".to_string(),
        ))
    }
    fn get_namespace(collection: String, namespace: String) -> Result<NamespaceInfo, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "get_namespace is not supported by Qdrant".to_string(),
        ))
    }
    fn list_namespaces(collection: String) -> Result<Vec<NamespaceInfo>, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "list_namespaces is not supported by Qdrant".to_string(),
        ))
    }
    fn namespace_exists(collection: String, namespace: String) -> Result<bool, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "namespace_exists is not supported by Qdrant".to_string(),
        ))
    }
    fn upsert_namespace(
        collection: String,
        namespace: String,
        metadata: Option<golem_vector::golem::vector::namespaces::Metadata>,
    ) -> Result<NamespaceInfo, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "upsert_namespace is not supported by Qdrant".to_string(),
        ))
    }
}
