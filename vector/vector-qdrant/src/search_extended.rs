use golem_vector::golem::vector::search_extended::Guest as SearchExtendedGuest;

use crate::QdrantComponent;

impl SearchExtendedGuest for QdrantComponent {
    fn discover_vectors(
        collection: String,
        context_pairs: Vec<golem_vector::golem::vector::search_extended::ContextPair>,
        limit: u32,
        filter: Option<golem_vector::golem::vector::search_extended::FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<
        Vec<golem_vector::golem::vector::search_extended::SearchResult>,
        golem_vector::golem::vector::search_extended::VectorError,
    > {
        Err(
            golem_vector::golem::vector::search_extended::VectorError::UnsupportedFeature(
                "discover_vectors is not supported by Qdrant".to_string(),
            ),
        )
    }
    fn recommend_vectors(
        collection: String,
        positive: Vec<golem_vector::golem::vector::search_extended::RecommendationExample>,
        negative: Option<Vec<golem_vector::golem::vector::search_extended::RecommendationExample>>,
        limit: u32,
        filter: Option<golem_vector::golem::vector::search_extended::FilterExpression>,
        namespace: Option<String>,
        strategy: Option<golem_vector::golem::vector::search_extended::RecommendationStrategy>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<
        Vec<golem_vector::golem::vector::search_extended::SearchResult>,
        golem_vector::golem::vector::search_extended::VectorError,
    > {
        Err(
            golem_vector::golem::vector::search_extended::VectorError::UnsupportedFeature(
                "recommend_vectors is not supported by Qdrant".to_string(),
            ),
        )
    }
    fn search_groups(
        collection: String,
        query: golem_vector::golem::vector::search_extended::SearchQuery,
        group_by: String,
        group_size: u32,
        max_groups: u32,
        filter: Option<golem_vector::golem::vector::search_extended::FilterExpression>,
        namespace: Option<String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<
        Vec<golem_vector::golem::vector::search_extended::GroupedSearchResult>,
        golem_vector::golem::vector::search_extended::VectorError,
    > {
        Err(
            golem_vector::golem::vector::search_extended::VectorError::UnsupportedFeature(
                "search_groups is not supported by Qdrant".to_string(),
            ),
        )
    }
    fn search_range(
        collection: String,
        vector: golem_vector::golem::vector::search_extended::VectorData,
        min_distance: Option<f32>,
        max_distance: f32,
        filter: Option<golem_vector::golem::vector::search_extended::FilterExpression>,
        namespace: Option<String>,
        limit: Option<u32>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    ) -> Result<
        Vec<golem_vector::golem::vector::search_extended::SearchResult>,
        golem_vector::golem::vector::search_extended::VectorError,
    > {
        Err(
            golem_vector::golem::vector::search_extended::VectorError::UnsupportedFeature(
                "search_range is not supported by Qdrant".to_string(),
            ),
        )
    }
    fn search_text(
        collection: String,
        query_text: String,
        limit: u32,
        filter: Option<golem_vector::golem::vector::search_extended::FilterExpression>,
        namespace: Option<String>,
    ) -> Result<
        Vec<golem_vector::golem::vector::search_extended::SearchResult>,
        golem_vector::golem::vector::search_extended::VectorError,
    > {
        Err(
            golem_vector::golem::vector::search_extended::VectorError::UnsupportedFeature(
                "search_text is not supported by Qdrant".to_string(),
            ),
        )
    }
}
