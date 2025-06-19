use crate::QdrantComponent;
use golem_vector::golem::vector::connection::{Guest as ConnectionGuest, VectorError};

impl ConnectionGuest for QdrantComponent {
    fn connect(
        endpoint: String,
        credentials: Option<golem_vector::golem::vector::connection::Credentials>,
        timeout_ms: Option<u32>,
        options: Option<golem_vector::golem::vector::connection::Metadata>,
    ) -> Result<(), VectorError> {
        Err(VectorError::UnsupportedFeature(
            "connect is not supported by Qdrant".to_string(),
        ))
    }
    fn disconnect() -> Result<(), VectorError> {
        Err(VectorError::UnsupportedFeature(
            "disconnect is not supported by Qdrant".to_string(),
        ))
    }
    fn get_connection_status()
    -> Result<golem_vector::golem::vector::connection::ConnectionStatus, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "get_connection_status is not supported by Qdrant".to_string(),
        ))
    }
    fn test_connection(
        endpoint: String,
        credentials: Option<golem_vector::golem::vector::connection::Credentials>,
        timeout_ms: Option<u32>,
        options: Option<golem_vector::golem::vector::connection::Metadata>,
    ) -> Result<bool, VectorError> {
        Err(VectorError::UnsupportedFeature(
            "test_connection is not supported by Qdrant".to_string(),
        ))
    }
}
