use crate::QdrantComponent;
use golem_vector::golem::vector::search::{Guest as SearchGuest, SearchResult};

impl SearchGuest for QdrantComponent {
    fn batch_search(
        collection: String,
        queries: Vec<golem_vector::golem::vector::search::SearchQuery>,
        limit: u32,
        filter: Option<golem_vector::golem::vector::search::FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
        search_params: Option<Vec<(String, String)>>,
    ) -> Result<Vec<Vec<SearchResult>>, golem_vector::golem::vector::search::VectorError> {
        Err(
            golem_vector::golem::vector::search::VectorError::UnsupportedFeature(
                "batch_search is not supported by Qdrant".to_string(),
            ),
        )
    }
    fn find_similar(
        collection: String,
        vector: golem_vector::golem::vector::search::VectorData,
        limit: u32,
        namespace: Option<String>,
    ) -> Result<Vec<SearchResult>, golem_vector::golem::vector::search::VectorError> {
        Err(
            golem_vector::golem::vector::search::VectorError::UnsupportedFeature(
                "find_similar is not supported by Qdrant".to_string(),
            ),
        )
    }
    fn search_vectors(
        collection: String,
        query: golem_vector::golem::vector::search::SearchQuery,
        limit: u32,
        filter: Option<golem_vector::golem::vector::search::FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
        min_score: Option<f32>,
        max_distance: Option<f32>,
        search_params: Option<Vec<(String, String)>>,
    ) -> Result<Vec<SearchResult>, golem_vector::golem::vector::search::VectorError> {
        Err(
            golem_vector::golem::vector::search::VectorError::UnsupportedFeature(
                "search_vectors is not supported by Qdrant".to_string(),
            ),
        )
    }
}
