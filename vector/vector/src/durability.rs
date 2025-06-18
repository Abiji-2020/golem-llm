use crate::golem::vector::{
    analytics::Guest as AnalyticGuest, collections::Guest as CollectionGuest,
    connection::Guest as ConnectionGuest, namespaces::Guest as NamespaceGuest,
    search::Guest as SearchGuest, search_extended::Guest as SearchExtendedGuest,
    vectors::Guest as VectorGuest,
};
use golem_rust::{FromValueAndType, IntoValue};
use std::marker::PhantomData;

#[derive(Debug, Clone, FromValueAndType, IntoValue)]
struct Unit;

// --- Durable wrappers ---

pub struct DurableVector<Impl> {
    _phantom: PhantomData<Impl>,
}

// - Guest Trait for providers

// must be implemented by the provider
pub trait ExtendedGuest:
    AnalyticGuest
    + CollectionGuest
    + ConnectionGuest
    + NamespaceGuest
    + SearchGuest
    + SearchExtendedGuest
    + VectorGuest
    + 'static
{
}
#[cfg(not(feature = "durability"))]
mod passthrough_impl {
    use super::ExtendedGuest;
    use crate::golem::vector::{
        analytics::Guest as AnalyticGuest, collections::Guest as CollectionGuest,
        connection::Guest as ConnectionGuest, namespaces::Guest as NamespaceGuest,
        search::Guest as SearchGuest, search_extended::Guest as SearchExtendedGuest,
        vectors::Guest as VectorGuest,
    };

    impl<Impl: ExtendedGuest> AnalyticGuest for Impl {
        fn get_collection_stats(
            collection: crate::_rt::String,
            namespace: Option<crate::_rt::String>,
        ) -> Result<
            crate::golem::vector::analytics::CollectionStats,
            crate::golem::vector::analytics::VectorError,
        > {
            Impl::get_collection_stats(collection, namespace)
        }
        fn get_field_distribution(
            collection: crate::_rt::String,
            field: crate::_rt::String,
            limit: Option<u32>,
            namespace: Option<crate::_rt::String>,
        ) -> Result<
            crate::_rt::Vec<(crate::golem::vector::analytics::FlatMetadataValue, u64)>,
            crate::golem::vector::analytics::VectorError,
        > {
            Impl::get_field_distribution(collection, field, limit, namespace)
        }
        fn get_field_stats(
            collection: crate::_rt::String,
            field: crate::_rt::String,
            namespace: Option<crate::_rt::String>,
        ) -> Result<
            crate::golem::vector::analytics::FieldStats,
            crate::golem::vector::analytics::VectorError,
        > {
            Impl::get_field_stats(collection, field, namespace)
        }
    }
    impl<Impl: ExtendedGuest> CollectionGuest for Impl {
        fn collection_exists(
            name: crate::_rt::String,
        ) -> Result<bool, crate::golem::vector::collections::VectorError> {
            Impl::collection_exists(name)
        }
        fn delete_collection(
            name: crate::_rt::String,
        ) -> Result<(), crate::golem::vector::collections::VectorError> {
            Impl::delete_collection(name)
        }
        fn get_collection(
            name: crate::_rt::String,
        ) -> Result<
            crate::golem::vector::collections::CollectionInfo,
            crate::golem::vector::collections::VectorError,
        > {
            Impl::get_collection(name)
        }
        fn list_collections() -> Result<
            crate::_rt::Vec<crate::golem::vector::collections::CollectionInfo>,
            crate::golem::vector::collections::VectorError,
        > {
            Impl::list_collections()
        }
        fn update_collection(
            name: crate::_rt::String,
            description: Option<crate::_rt::String>,
            metadata: Option<crate::golem::vector::collections::Metadata>,
        ) -> Result<
            crate::golem::vector::collections::CollectionInfo,
            crate::golem::vector::collections::VectorError,
        > {
            Impl::update_collection(name, description, metadata)
        }
        fn upsert_collection(
            name: crate::_rt::String,
            description: Option<crate::_rt::String>,
            dimension: u32,
            metric: crate::golem::vector::collections::DistanceMetric,
            index_config: Option<crate::golem::vector::collections::IndexConfig>,
            metadata: Option<crate::golem::vector::collections::Metadata>,
        ) -> Result<
            crate::golem::vector::collections::CollectionInfo,
            crate::golem::vector::collections::VectorError,
        > {
            Impl::upsert_collection(name, description, dimension, metric, index_config, metadata)
        }
    }
    impl<Impl: ExtendedGuest> ConnectionGuest for Impl {
        fn connect(
            endpoint: crate::_rt::String,
            credentials: Option<crate::golem::vector::connection::Credentials>,
            timeout_ms: Option<u32>,
            options: Option<crate::golem::vector::connection::Metadata>,
        ) -> Result<(), crate::golem::vector::connection::VectorError> {
            Impl::connect(endpoint, credentials, timeout_ms, options)
        }
        fn disconnect() -> Result<(), crate::golem::vector::connection::VectorError> {
            Impl::disconnect()
        }
        fn get_connection_status() -> Result<
            crate::golem::vector::connection::ConnectionStatus,
            crate::golem::vector::connection::VectorError,
        > {
            Impl::get_connection_status()
        }
        fn test_connection(
            endpoint: crate::_rt::String,
            credentials: Option<crate::golem::vector::connection::Credentials>,
            timeout_ms: Option<u32>,
            options: Option<crate::golem::vector::connection::Metadata>,
        ) -> Result<bool, crate::golem::vector::connection::VectorError> {
            Impl::test_connection(endpoint, credentials, timeout_ms, options)
        }
    }
    impl<Impl: ExtendedGuest> NamespaceGuest for Impl {
        fn delete_namespace(
            collection: crate::_rt::String,
            namespace: crate::_rt::String,
        ) -> Result<(), crate::golem::vector::namespaces::VectorError> {
            <Impl as NamespaceGuest>::delete_namespace(collection, namespace)
        }
        fn get_namespace(
            collection: crate::_rt::String,
            namespace: crate::_rt::String,
        ) -> Result<
            crate::golem::vector::namespaces::NamespaceInfo,
            crate::golem::vector::namespaces::VectorError,
        > {
            Impl::get_namespace(collection, namespace)
        }
        fn list_namespaces(
            collection: crate::_rt::String,
        ) -> Result<
            crate::_rt::Vec<crate::golem::vector::namespaces::NamespaceInfo>,
            crate::golem::vector::namespaces::VectorError,
        > {
            Impl::list_namespaces(collection)
        }
        fn namespace_exists(
            collection: crate::_rt::String,
            namespace: crate::_rt::String,
        ) -> Result<bool, crate::golem::vector::namespaces::VectorError> {
            Impl::namespace_exists(collection, namespace)
        }
        fn upsert_namespace(
            collection: crate::_rt::String,
            namespace: crate::_rt::String,
            metadata: Option<crate::golem::vector::namespaces::Metadata>,
        ) -> Result<
            crate::golem::vector::namespaces::NamespaceInfo,
            crate::golem::vector::namespaces::VectorError,
        > {
            Impl::upsert_namespace(collection, namespace, metadata)
        }
    }
    impl<Impl: ExtendedGuest> SearchGuest for Impl {
        fn batch_search(
            collection: crate::_rt::String,
            queries: crate::_rt::Vec<crate::golem::vector::search::SearchQuery>,
            limit: u32,
            filter: Option<crate::golem::vector::search::FilterExpression>,
            namespace: Option<crate::_rt::String>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
            search_params: Option<crate::_rt::Vec<(crate::_rt::String, crate::_rt::String)>>,
        ) -> Result<
            crate::_rt::Vec<crate::_rt::Vec<crate::golem::vector::search::SearchResult>>,
            crate::golem::vector::search::VectorError,
        > {
            Impl::batch_search(
                collection,
                queries,
                limit,
                filter,
                namespace,
                include_vectors,
                include_metadata,
                search_params,
            )
        }
        fn find_similar(
            collection: crate::_rt::String,
            vector: crate::golem::vector::search::VectorData,
            limit: u32,
            namespace: Option<crate::_rt::String>,
        ) -> Result<
            crate::_rt::Vec<crate::golem::vector::search::SearchResult>,
            crate::golem::vector::search::VectorError,
        > {
            Impl::find_similar(collection, vector, limit, namespace)
        }
        fn search_vectors(
            collection: crate::_rt::String,
            query: crate::golem::vector::search::SearchQuery,
            limit: u32,
            filter: Option<crate::golem::vector::search::FilterExpression>,
            namespace: Option<crate::_rt::String>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
            min_score: Option<f32>,
            max_distance: Option<f32>,
            search_params: Option<crate::_rt::Vec<(crate::_rt::String, crate::_rt::String)>>,
        ) -> Result<
            crate::_rt::Vec<crate::golem::vector::search::SearchResult>,
            crate::golem::vector::search::VectorError,
        > {
            Impl::search_vectors(
                collection,
                query,
                limit,
                filter,
                namespace,
                include_vectors,
                include_metadata,
                min_score,
                max_distance,
                search_params,
            )
        }
    }
    impl<Impl: ExtendedGuest> SearchExtendedGuest for Impl {
        fn discover_vectors(
            collection: crate::_rt::String,
            context_pairs: crate::_rt::Vec<crate::golem::vector::search_extended::ContextPair>,
            limit: u32,
            filter: Option<crate::golem::vector::search_extended::FilterExpression>,
            namespace: Option<crate::_rt::String>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
        ) -> Result<
            crate::_rt::Vec<crate::golem::vector::search_extended::SearchResult>,
            crate::golem::vector::search_extended::VectorError,
        > {
            Impl::discover_vectors(
                collection,
                context_pairs,
                limit,
                filter,
                namespace,
                include_vectors,
                include_metadata,
            )
        }
        fn recommend_vectors(
            collection: crate::_rt::String,
            positive: crate::_rt::Vec<crate::golem::vector::search_extended::RecommendationExample>,
            negative: Option<
                crate::_rt::Vec<crate::golem::vector::search_extended::RecommendationExample>,
            >,
            limit: u32,
            filter: Option<crate::golem::vector::search_extended::FilterExpression>,
            namespace: Option<crate::_rt::String>,
            strategy: Option<crate::golem::vector::search_extended::RecommendationStrategy>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
        ) -> Result<
            crate::_rt::Vec<crate::golem::vector::search_extended::SearchResult>,
            crate::golem::vector::search_extended::VectorError,
        > {
            Impl::recommend_vectors(
                collection,
                positive,
                negative,
                limit,
                filter,
                namespace,
                strategy,
                include_vectors,
                include_metadata,
            )
        }
        fn search_groups(
            collection: crate::_rt::String,
            query: crate::golem::vector::search_extended::SearchQuery,
            group_by: crate::_rt::String,
            group_size: u32,
            max_groups: u32,
            filter: Option<crate::golem::vector::search_extended::FilterExpression>,
            namespace: Option<crate::_rt::String>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
        ) -> Result<
            crate::_rt::Vec<crate::golem::vector::search_extended::GroupedSearchResult>,
            crate::golem::vector::search_extended::VectorError,
        > {
            Impl::search_groups(
                collection,
                query,
                group_by,
                group_size,
                max_groups,
                filter,
                namespace,
                include_vectors,
                include_metadata,
            )
        }
        fn search_range(
            collection: crate::_rt::String,
            vector: crate::golem::vector::search_extended::VectorData,
            min_distance: Option<f32>,
            max_distance: f32,
            filter: Option<crate::golem::vector::search_extended::FilterExpression>,
            namespace: Option<crate::_rt::String>,
            limit: Option<u32>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
        ) -> Result<
            crate::_rt::Vec<crate::golem::vector::search_extended::SearchResult>,
            crate::golem::vector::search_extended::VectorError,
        > {
            Impl::search_range(
                collection,
                vector,
                min_distance,
                max_distance,
                filter,
                namespace,
                limit,
                include_vectors,
                include_metadata,
            )
        }
        fn search_text(
            collection: crate::_rt::String,
            query_text: crate::_rt::String,
            limit: u32,
            filter: Option<crate::golem::vector::search_extended::FilterExpression>,
            namespace: Option<crate::_rt::String>,
        ) -> Result<
            crate::_rt::Vec<crate::golem::vector::search_extended::SearchResult>,
            crate::golem::vector::search_extended::VectorError,
        > {
            Impl::search_text(collection, query_text, limit, filter, namespace)
        }
    }
    impl<Impl: ExtendedGuest> VectorGuest for Impl {
        fn count_vectors(
            collection: crate::_rt::String,
            filter: Option<crate::golem::vector::vectors::FilterExpression>,
            namespace: Option<crate::_rt::String>,
        ) -> Result<u64, crate::golem::vector::vectors::VectorError> {
            Impl::count_vectors(collection, filter, namespace)
        }
        fn delete_by_filter(
            collection: crate::_rt::String,
            filter: crate::golem::vector::vectors::FilterExpression,
            namespace: Option<crate::_rt::String>,
        ) -> Result<u32, crate::golem::vector::vectors::VectorError> {
            Impl::delete_by_filter(collection, filter, namespace)
        }
        fn delete_namespace(
            collection: crate::_rt::String,
            namespace: crate::_rt::String,
        ) -> Result<u32, crate::golem::vector::vectors::VectorError> {
            <Impl as VectorGuest>::delete_namespace(collection, namespace)
        }
        fn delete_vectors(
            collection: crate::_rt::String,
            ids: crate::_rt::Vec<crate::golem::vector::vectors::Id>,
            namespace: Option<crate::_rt::String>,
        ) -> Result<u32, crate::golem::vector::vectors::VectorError> {
            Impl::delete_vectors(collection, ids, namespace)
        }
        fn get_vector(
            collection: crate::_rt::String,
            id: crate::golem::vector::vectors::Id,
            namespace: Option<crate::_rt::String>,
        ) -> Result<
            Option<crate::golem::vector::vectors::VectorRecord>,
            crate::golem::vector::vectors::VectorError,
        > {
            Impl::get_vector(collection, id, namespace)
        }
        fn get_vectors(
            collection: crate::_rt::String,
            ids: crate::_rt::Vec<crate::golem::vector::vectors::Id>,
            namespace: Option<crate::_rt::String>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
        ) -> Result<
            crate::_rt::Vec<crate::golem::vector::vectors::VectorRecord>,
            crate::golem::vector::vectors::VectorError,
        > {
            Impl::get_vectors(
                collection,
                ids,
                namespace,
                include_vectors,
                include_metadata,
            )
        }
        fn list_vectors(
            collection: crate::_rt::String,
            namespace: Option<crate::_rt::String>,
            filter: Option<crate::golem::vector::vectors::FilterExpression>,
            limit: Option<u32>,
            cursor: Option<crate::_rt::String>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
        ) -> Result<
            crate::golem::vector::vectors::ListResponse,
            crate::golem::vector::vectors::VectorError,
        > {
            Impl::list_vectors(
                collection,
                namespace,
                filter,
                limit,
                cursor,
                include_vectors,
                include_metadata,
            )
        }
        fn update_vector(
            collection: crate::_rt::String,
            id: crate::golem::vector::vectors::Id,
            vector: Option<crate::golem::vector::vectors::VectorData>,
            metadata: Option<crate::golem::vector::vectors::Metadata>,
            namespace: Option<crate::_rt::String>,
            merge_metadata: Option<bool>,
        ) -> Result<(), crate::golem::vector::vectors::VectorError> {
            Impl::update_vector(collection, id, vector, metadata, namespace, merge_metadata)
        }
        fn upsert_vector(
            collection: crate::_rt::String,
            id: crate::golem::vector::vectors::Id,
            vector: crate::golem::vector::vectors::VectorData,
            metadata: Option<crate::golem::vector::vectors::Metadata>,
            namespace: Option<crate::_rt::String>,
        ) -> Result<(), crate::golem::vector::vectors::VectorError> {
            Impl::upsert_vector(collection, id, vector, metadata, namespace)
        }
        fn upsert_vectors(
            collection: crate::_rt::String,
            vectors: crate::_rt::Vec<crate::golem::vector::vectors::VectorRecord>,
            namespace: Option<crate::_rt::String>,
        ) -> Result<
            crate::golem::vector::vectors::BatchResult,
            crate::golem::vector::vectors::VectorError,
        > {
            Impl::upsert_vectors(collection, vectors, namespace)
        }
    }
}

#[cfg(feature = "durability")]
mod durable_impl {
    use crate::durability::{DurableVector, ExtendedGuest};
    use crate::golem::vector::{
        analytics::{CollectionStats, FieldStats, Guest as AnalyticGuest},
        collections::{CollectionInfo, Guest as CollectionGuest, IndexConfig},
        connection::{ConnectionStatus, Credentials, Guest as ConnectionGuest},
        namespaces::{Guest as NamespaceGuest, NamespaceInfo},
        search::{Guest as SearchGuest, SearchQuery, SearchResult},
        search_extended::{
            ContextPair, GroupedSearchResult, Guest as SearchExtendedGuest, RecommendationExample,
            RecommendationStrategy,
        },
        vectors::{
            BatchResult, FilterExpression, Guest as VectorGuest, ListResponse, VectorData,
            VectorRecord,
        },
    };
    use golem_rust::bindings::golem::durability::durability::DurableFunctionType;
    use golem_rust::durability::Durability;
    use golem_rust::{FromValueAndType, IntoValue, PersistenceLevel, with_persistence_level};
    use std::fmt::{Display, Formatter};

    impl<Impl: ExtendedGuest> AnalyticGuest for DurableVector<Impl> {
        fn get_collection_stats(
            collection: crate::_rt::String,
            namespace: Option<crate::_rt::String>,
        ) -> Result<CollectionStats, crate::golem::vector::analytics::VectorError> {
            let durability = Durability::<
                Result<CollectionStats, crate::golem::vector::analytics::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "get_collection_stats",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::get_collection_stats(collection.clone(), namespace.clone())
                });
                durability.persist_infallible(
                    GenerateGetCollectionStatsParam {
                        collection,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn get_field_distribution(
            collection: crate::_rt::String,
            field: crate::_rt::String,
            limit: Option<u32>,
            namespace: Option<crate::_rt::String>,
        ) -> Result<
            crate::_rt::Vec<(crate::golem::vector::analytics::FlatMetadataValue, u64)>,
            crate::golem::vector::analytics::VectorError,
        > {
            let durability = Durability::<
                Result<
                    crate::_rt::Vec<(crate::golem::vector::analytics::FlatMetadataValue, u64)>,
                    crate::golem::vector::analytics::VectorError,
                >,
                UnusedError,
            >::new(
                "golem_vector",
                "get_field_distribution",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::get_field_distribution(
                        collection.clone(),
                        field.clone(),
                        limit.clone(),
                        namespace.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateGetFieldDistributionParam {
                        collection,
                        field,
                        limit,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn get_field_stats(
            collection: crate::_rt::String,
            field: crate::_rt::String,
            namespace: Option<crate::_rt::String>,
        ) -> Result<FieldStats, crate::golem::vector::analytics::VectorError> {
            let durability = Durability::<
                Result<FieldStats, crate::golem::vector::analytics::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "get_field_stats",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::get_field_stats(collection.clone(), field.clone(), namespace.clone())
                });
                durability.persist_infallible(
                    GenerateGetFieldStatsParam {
                        collection,
                        field,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateGetCollectionStatsParam {
        collection: crate::_rt::String,
        namespace: Option<crate::_rt::String>,
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateGetFieldDistributionParam {
        collection: crate::_rt::String,
        field: crate::_rt::String,
        limit: Option<u32>,
        namespace: Option<crate::_rt::String>,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateGetFieldStatsParam {
        collection: crate::_rt::String,
        field: crate::_rt::String,
        namespace: Option<crate::_rt::String>,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct UnusedError;
    impl Display for UnusedError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "This error is unused and only serves as a placeholder.")
        }
    }

    impl<Impl: ExtendedGuest> CollectionGuest for DurableVector<Impl> {
        fn collection_exists(
            name: crate::_rt::String,
        ) -> Result<bool, crate::golem::vector::collections::VectorError> {
            let durability = Durability::<
                Result<bool, crate::golem::vector::collections::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "collection_exists",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::collection_exists(name.clone())
                });
                durability
                    .persist_infallible(GenerateCollectionExistsParam { name }, result.clone())
            } else {
                durability.replay_infallible()
            }
        }
        fn delete_collection(
            name: crate::_rt::String,
        ) -> Result<(), crate::golem::vector::collections::VectorError> {
            let durability = Durability::<
                Result<(), crate::golem::vector::collections::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "delete_collection",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::delete_collection(name.clone())
                });
                durability
                    .persist_infallible(GenerateDeleteCollectionParam { name }, result.clone())
            } else {
                durability.replay_infallible()
            }
        }
        fn get_collection(
            name: crate::_rt::String,
        ) -> Result<CollectionInfo, crate::golem::vector::collections::VectorError> {
            let durability = Durability::<
                Result<CollectionInfo, crate::golem::vector::collections::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "get_collection",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::get_collection(name.clone())
                });
                durability.persist_infallible(GenerateGetCollectionParam { name }, result.clone())
            } else {
                durability.replay_infallible()
            }
        }
        fn list_collections()
        -> Result<crate::_rt::Vec<CollectionInfo>, crate::golem::vector::collections::VectorError>
        {
            let durability = Durability::<
                Result<
                    crate::_rt::Vec<CollectionInfo>,
                    crate::golem::vector::collections::VectorError,
                >,
                UnusedError,
            >::new(
                "golem_vector",
                "list_collections",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::list_collections()
                });
                durability.persist_infallible(Unit, result.clone())
            } else {
                durability.replay_infallible()
            }
        }
        fn update_collection(
            name: crate::_rt::String,
            description: Option<crate::_rt::String>,
            metadata: Option<crate::golem::vector::collections::Metadata>,
        ) -> Result<CollectionInfo, crate::golem::vector::collections::VectorError> {
            let durability = Durability::<
                Result<CollectionInfo, crate::golem::vector::collections::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "update_collection",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::update_collection(name.clone(), description.clone(), metadata.clone())
                });
                durability.persist_infallible(
                    GenerateUpdateCollectionParam {
                        name,
                        description,
                        metadata,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn upsert_collection(
            name: crate::_rt::String,
            description: Option<crate::_rt::String>,
            dimension: u32,
            metric: crate::golem::vector::collections::DistanceMetric,
            index_config: Option<IndexConfig>,
            metadata: Option<crate::golem::vector::collections::Metadata>,
        ) -> Result<CollectionInfo, crate::golem::vector::collections::VectorError> {
            let durability = Durability::<
                Result<CollectionInfo, crate::golem::vector::collections::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "upsert_collection",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::upsert_collection(
                        name.clone(),
                        description.clone(),
                        dimension.clone(),
                        metric.clone(),
                        index_config.clone(),
                        metadata.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateUpsertCollectionParam {
                        name,
                        description,
                        dimension,
                        metric,
                        index_config,
                        metadata,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateCollectionExistsParam {
        name: crate::_rt::String,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateDeleteCollectionParam {
        name: crate::_rt::String,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateGetCollectionParam {
        name: crate::_rt::String,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateUpdateCollectionParam {
        name: crate::_rt::String,
        description: Option<crate::_rt::String>,
        metadata: Option<crate::golem::vector::collections::Metadata>,
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateUpsertCollectionParam {
        name: crate::_rt::String,
        description: Option<crate::_rt::String>,
        dimension: u32,
        metric: crate::golem::vector::collections::DistanceMetric,
        index_config: Option<IndexConfig>,
        metadata: Option<crate::golem::vector::collections::Metadata>,
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct Unit;

    impl<Impl: ExtendedGuest> ConnectionGuest for DurableVector<Impl> {
        fn connect(
            endpoint: crate::_rt::String,
            credentials: Option<Credentials>,
            timeout_ms: Option<u32>,
            options: Option<crate::golem::vector::connection::Metadata>,
        ) -> Result<(), crate::golem::vector::connection::VectorError> {
            let durability = Durability::<
                Result<(), crate::golem::vector::connection::VectorError>,
                UnusedError,
            >::new(
                "golem_vector", "connect", DurableFunctionType::WriteRemote
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::connect(
                        endpoint.clone(),
                        credentials.clone(),
                        timeout_ms.clone(),
                        options.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateConnectParam {
                        endpoint,
                        credentials,
                        timeout_ms,
                        options,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }

        fn disconnect() -> Result<(), crate::golem::vector::connection::VectorError> {
            let durability = Durability::<
                Result<(), crate::golem::vector::connection::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "disconnect",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result =
                    with_persistence_level(PersistenceLevel::PersistNothing, || Impl::disconnect());
                durability.persist_infallible(Unit, result.clone())
            } else {
                durability.replay_infallible()
            }
        }
        fn get_connection_status()
        -> Result<ConnectionStatus, crate::golem::vector::connection::VectorError> {
            let durability = Durability::<
                Result<ConnectionStatus, crate::golem::vector::connection::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "get_connection_status",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::get_connection_status()
                });
                durability.persist_infallible(Unit, result.clone())
            } else {
                durability.replay_infallible()
            }
        }
        fn test_connection(
            endpoint: crate::_rt::String,
            credentials: Option<Credentials>,
            timeout_ms: Option<u32>,
            options: Option<crate::golem::vector::connection::Metadata>,
        ) -> Result<bool, crate::golem::vector::connection::VectorError> {
            let durability = Durability::<
                Result<bool, crate::golem::vector::connection::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "test_connection",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::test_connection(
                        endpoint.clone(),
                        credentials.clone(),
                        timeout_ms.clone(),
                        options.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateTestConnectionParam {
                        endpoint,
                        credentials,
                        timeout_ms,
                        options,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateConnectParam {
        endpoint: crate::_rt::String,
        credentials: Option<Credentials>,
        timeout_ms: Option<u32>,
        options: Option<crate::golem::vector::connection::Metadata>,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateTestConnectionParam {
        endpoint: crate::_rt::String,
        credentials: Option<Credentials>,
        timeout_ms: Option<u32>,
        options: Option<crate::golem::vector::connection::Metadata>,
    }

    impl<Impl: ExtendedGuest> NamespaceGuest for DurableVector<Impl> {
        fn delete_namespace(
            collection: crate::_rt::String,
            namespace: crate::_rt::String,
        ) -> Result<(), crate::golem::vector::namespaces::VectorError> {
            let durability = Durability::<
                Result<(), crate::golem::vector::namespaces::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "delete_namespace",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    <Impl as NamespaceGuest>::delete_namespace(
                        collection.clone(),
                        namespace.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateDeleteNamespaceParam {
                        collection,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn get_namespace(
            collection: crate::_rt::String,
            namespace: crate::_rt::String,
        ) -> Result<NamespaceInfo, crate::golem::vector::namespaces::VectorError> {
            let durability = Durability::<
                Result<NamespaceInfo, crate::golem::vector::namespaces::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "get_namespace",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    <Impl as NamespaceGuest>::get_namespace(collection.clone(), namespace.clone())
                });
                durability.persist_infallible(
                    GenerateGetNamespaceParam {
                        collection,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn list_namespaces(
            collection: crate::_rt::String,
        ) -> Result<crate::_rt::Vec<NamespaceInfo>, crate::golem::vector::namespaces::VectorError>
        {
            let durability = Durability::<
                Result<
                    crate::_rt::Vec<NamespaceInfo>,
                    crate::golem::vector::namespaces::VectorError,
                >,
                UnusedError,
            >::new(
                "golem_vector",
                "list_namespaces",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    <Impl as NamespaceGuest>::list_namespaces(collection.clone())
                });
                durability
                    .persist_infallible(GenerateListNamespacesParam { collection }, result.clone())
            } else {
                durability.replay_infallible()
            }
        }

        fn namespace_exists(
            collection: crate::_rt::String,
            namespace: crate::_rt::String,
        ) -> Result<bool, crate::golem::vector::namespaces::VectorError> {
            let durability = Durability::<
                Result<bool, crate::golem::vector::namespaces::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "namespace_exists",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    <Impl as NamespaceGuest>::namespace_exists(
                        collection.clone(),
                        namespace.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateNamespaceExistsParam {
                        collection,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }

        fn upsert_namespace(
            collection: crate::_rt::String,
            namespace: crate::_rt::String,
            metadata: Option<crate::golem::vector::namespaces::Metadata>,
        ) -> Result<NamespaceInfo, crate::golem::vector::namespaces::VectorError> {
            let durability = Durability::<
                Result<NamespaceInfo, crate::golem::vector::namespaces::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "upsert_namespace",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    <Impl as NamespaceGuest>::upsert_namespace(
                        collection.clone(),
                        namespace.clone(),
                        metadata.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateUpsertNamespaceParam {
                        collection,
                        namespace,
                        metadata,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateDeleteNamespaceParam {
        collection: crate::_rt::String,
        namespace: crate::_rt::String,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateGetNamespaceParam {
        collection: crate::_rt::String,
        namespace: crate::_rt::String,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateListNamespacesParam {
        collection: crate::_rt::String,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateNamespaceExistsParam {
        collection: crate::_rt::String,
        namespace: crate::_rt::String,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateUpsertNamespaceParam {
        collection: crate::_rt::String,
        namespace: crate::_rt::String,
        metadata: Option<crate::golem::vector::namespaces::Metadata>,
    }

    impl<Impl: ExtendedGuest> SearchGuest for DurableVector<Impl> {
        fn batch_search(
            collection: crate::_rt::String,
            queries: crate::_rt::Vec<SearchQuery>,
            limit: u32,
            filter: Option<crate::golem::vector::search::FilterExpression>,
            namespace: Option<crate::_rt::String>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
            search_params: Option<crate::_rt::Vec<(crate::_rt::String, crate::_rt::String)>>,
        ) -> Result<
            crate::_rt::Vec<crate::_rt::Vec<SearchResult>>,
            crate::golem::vector::search::VectorError,
        > {
            let durability = Durability::<
                Result<
                    crate::_rt::Vec<crate::_rt::Vec<SearchResult>>,
                    crate::golem::vector::search::VectorError,
                >,
                UnusedError,
            >::new(
                "golem_vector",
                "batch_search",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::batch_search(
                        collection.clone(),
                        queries.clone(),
                        limit,
                        filter.clone(),
                        namespace.clone(),
                        include_vectors.clone(),
                        include_metadata.clone(),
                        search_params.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateBatchSearchParam {
                        collection,
                        queries,
                        limit,
                        filter,
                        namespace,
                        include_vectors,
                        include_metadata,
                        search_params,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn find_similar(
            collection: crate::_rt::String,
            vector: crate::golem::vector::search::VectorData,
            limit: u32,
            namespace: Option<crate::_rt::String>,
        ) -> Result<crate::_rt::Vec<SearchResult>, crate::golem::vector::search::VectorError>
        {
            let durability = Durability::<
                Result<crate::_rt::Vec<SearchResult>, crate::golem::vector::search::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "find_similar",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::find_similar(collection.clone(), vector.clone(), limit, namespace.clone())
                });
                durability.persist_infallible(
                    GenerateFindSimilarParam {
                        collection,
                        vector,
                        limit,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn search_vectors(
            collection: crate::_rt::String,
            query: SearchQuery,
            limit: u32,
            filter: Option<crate::golem::vector::search::FilterExpression>,
            namespace: Option<crate::_rt::String>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
            min_score: Option<f32>,
            max_distance: Option<f32>,
            search_params: Option<crate::_rt::Vec<(crate::_rt::String, crate::_rt::String)>>,
        ) -> Result<crate::_rt::Vec<SearchResult>, crate::golem::vector::search::VectorError>
        {
            let durability = Durability::<
                Result<crate::_rt::Vec<SearchResult>, crate::golem::vector::search::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "search_vectors",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::search_vectors(
                        collection.clone(),
                        query.clone(),
                        limit,
                        filter.clone(),
                        namespace.clone(),
                        include_vectors.clone(),
                        include_metadata.clone(),
                        min_score,
                        max_distance,
                        search_params.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateSearchVectorsParam {
                        collection,
                        query,
                        limit,
                        filter,
                        namespace,
                        include_vectors,
                        include_metadata,
                        min_score,
                        max_distance,
                        search_params,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateBatchSearchParam {
        collection: crate::_rt::String,
        queries: crate::_rt::Vec<SearchQuery>,
        limit: u32,
        filter: Option<FilterExpression>,
        namespace: Option<crate::_rt::String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
        search_params: Option<crate::_rt::Vec<(crate::_rt::String, crate::_rt::String)>>,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateFindSimilarParam {
        collection: crate::_rt::String,
        vector: VectorData,
        limit: u32,
        namespace: Option<crate::_rt::String>,
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateSearchVectorsParam {
        collection: crate::_rt::String,
        query: SearchQuery,
        limit: u32,
        filter: Option<FilterExpression>,
        namespace: Option<crate::_rt::String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
        min_score: Option<f32>,
        max_distance: Option<f32>,
        search_params: Option<crate::_rt::Vec<(crate::_rt::String, crate::_rt::String)>>,
    }
    impl<Impl: ExtendedGuest> SearchExtendedGuest for DurableVector<Impl> {
        fn discover_vectors(
            collection: crate::_rt::String,
            context_pairs: crate::_rt::Vec<ContextPair>,
            limit: u32,
            filter: Option<crate::golem::vector::search_extended::FilterExpression>,
            namespace: Option<crate::_rt::String>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
        ) -> Result<
            crate::_rt::Vec<crate::golem::vector::search_extended::SearchResult>,
            crate::golem::vector::search_extended::VectorError,
        > {
            let durability = Durability::<
                Result<
                    crate::_rt::Vec<crate::golem::vector::search_extended::SearchResult>,
                    crate::golem::vector::search_extended::VectorError,
                >,
                UnusedError,
            >::new(
                "golem_vector",
                "discover_vectors",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::discover_vectors(
                        collection.clone(),
                        context_pairs.clone(),
                        limit,
                        filter.clone(),
                        namespace.clone(),
                        include_vectors.clone(),
                        include_metadata.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateDiscoverVectorsParam {
                        collection,
                        context_pairs,
                        limit,
                        filter,
                        namespace,
                        include_vectors,
                        include_metadata,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn recommend_vectors(
            collection: crate::_rt::String,
            positive: crate::_rt::Vec<RecommendationExample>,
            negative: Option<crate::_rt::Vec<RecommendationExample>>,
            limit: u32,
            filter: Option<crate::golem::vector::search_extended::FilterExpression>,
            namespace: Option<crate::_rt::String>,
            strategy: Option<RecommendationStrategy>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
        ) -> Result<
            crate::_rt::Vec<crate::golem::vector::search_extended::SearchResult>,
            crate::golem::vector::search_extended::VectorError,
        > {
            let durability = Durability::<
                Result<
                    crate::_rt::Vec<crate::golem::vector::search_extended::SearchResult>,
                    crate::golem::vector::search_extended::VectorError,
                >,
                UnusedError,
            >::new(
                "golem_vector",
                "recommend_vectors",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::recommend_vectors(
                        collection.clone(),
                        positive.clone(),
                        negative.clone(),
                        limit,
                        filter.clone(),
                        namespace.clone(),
                        strategy.clone(),
                        include_vectors.clone(),
                        include_metadata.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateRecommendVectorsParam {
                        collection,
                        positive,
                        negative,
                        limit,
                        filter,
                        namespace,
                        strategy,
                        include_vectors,
                        include_metadata,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn search_groups(
            collection: crate::_rt::String,
            query: crate::golem::vector::search_extended::SearchQuery,
            group_by: crate::_rt::String,
            group_size: u32,
            max_groups: u32,
            filter: Option<crate::golem::vector::search_extended::FilterExpression>,
            namespace: Option<crate::_rt::String>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
        ) -> Result<
            crate::_rt::Vec<GroupedSearchResult>,
            crate::golem::vector::search_extended::VectorError,
        > {
            let durability = Durability::<
                Result<
                    crate::_rt::Vec<GroupedSearchResult>,
                    crate::golem::vector::search_extended::VectorError,
                >,
                UnusedError,
            >::new(
                "golem_vector",
                "search_groups",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::search_groups(
                        collection.clone(),
                        query.clone(),
                        group_by.clone(),
                        group_size,
                        max_groups,
                        filter.clone(),
                        namespace.clone(),
                        include_vectors.clone(),
                        include_metadata.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateSearchGroupsParam {
                        collection,
                        query,
                        group_by,
                        group_size,
                        max_groups,
                        filter,
                        namespace,
                        include_vectors,
                        include_metadata,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }

        fn search_range(
            collection: crate::_rt::String,
            vector: crate::golem::vector::search_extended::VectorData,
            min_distance: Option<f32>,
            max_distance: f32,
            filter: Option<crate::golem::vector::search_extended::FilterExpression>,
            namespace: Option<crate::_rt::String>,
            limit: Option<u32>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
        ) -> Result<
            crate::_rt::Vec<crate::golem::vector::search_extended::SearchResult>,
            crate::golem::vector::search_extended::VectorError,
        > {
            let durability = Durability::<
                Result<
                    crate::_rt::Vec<crate::golem::vector::search_extended::SearchResult>,
                    crate::golem::vector::search_extended::VectorError,
                >,
                UnusedError,
            >::new(
                "golem_vector",
                "search_range",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::search_range(
                        collection.clone(),
                        vector.clone(),
                        min_distance,
                        max_distance,
                        filter.clone(),
                        namespace.clone(),
                        limit,
                        include_vectors.clone(),
                        include_metadata.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateSearchRangeParam {
                        collection,
                        vector,
                        min_distance,
                        max_distance,
                        filter,
                        namespace,
                        limit,
                        include_vectors,
                        include_metadata,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn search_text(
            collection: crate::_rt::String,
            query_text: crate::_rt::String,
            limit: u32,
            filter: Option<crate::golem::vector::search_extended::FilterExpression>,
            namespace: Option<crate::_rt::String>,
        ) -> Result<
            crate::_rt::Vec<crate::golem::vector::search_extended::SearchResult>,
            crate::golem::vector::search_extended::VectorError,
        > {
            let durability = Durability::<
                Result<
                    crate::_rt::Vec<crate::golem::vector::search_extended::SearchResult>,
                    crate::golem::vector::search_extended::VectorError,
                >,
                UnusedError,
            >::new(
                "golem_vector",
                "search_text",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::search_text(
                        collection.clone(),
                        query_text.clone(),
                        limit,
                        filter.clone(),
                        namespace.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateSearchTextParam {
                        collection,
                        query_text,
                        limit,
                        filter,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateDiscoverVectorsParam {
        collection: crate::_rt::String,
        context_pairs: crate::_rt::Vec<ContextPair>,
        limit: u32,
        filter: Option<FilterExpression>,
        namespace: Option<crate::_rt::String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateRecommendVectorsParam {
        collection: crate::_rt::String,
        positive: crate::_rt::Vec<RecommendationExample>,
        negative: Option<crate::_rt::Vec<RecommendationExample>>,
        limit: u32,
        filter: Option<FilterExpression>,
        namespace: Option<crate::_rt::String>,
        strategy: Option<RecommendationStrategy>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateSearchGroupsParam {
        collection: crate::_rt::String,
        query: SearchQuery,
        group_by: crate::_rt::String,
        group_size: u32,
        max_groups: u32,
        filter: Option<FilterExpression>,
        namespace: Option<crate::_rt::String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateSearchRangeParam {
        collection: crate::_rt::String,
        vector: VectorData,
        min_distance: Option<f32>,
        max_distance: f32,
        filter: Option<FilterExpression>,
        namespace: Option<crate::_rt::String>,
        limit: Option<u32>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateSearchTextParam {
        collection: crate::_rt::String,
        query_text: crate::_rt::String,
        limit: u32,
        filter: Option<FilterExpression>,
        namespace: Option<crate::_rt::String>,
    }

    impl<Impl: ExtendedGuest> VectorGuest for DurableVector<Impl> {
        fn count_vectors(
            collection: crate::_rt::String,
            filter: Option<FilterExpression>,
            namespace: Option<crate::_rt::String>,
        ) -> Result<u64, crate::golem::vector::vectors::VectorError> {
            let durability = Durability::<
                Result<u64, crate::golem::vector::vectors::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "count_vectors",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::count_vectors(collection.clone(), filter.clone(), namespace.clone())
                });
                durability.persist_infallible(
                    GenerateCountVectorsParam {
                        collection,
                        filter,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn delete_by_filter(
            collection: crate::_rt::String,
            filter: FilterExpression,
            namespace: Option<crate::_rt::String>,
        ) -> Result<u32, crate::golem::vector::vectors::VectorError> {
            let durability = Durability::<
                Result<u32, crate::golem::vector::vectors::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "delete_by_filter",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::delete_by_filter(collection.clone(), filter.clone(), namespace.clone())
                });
                durability.persist_infallible(
                    GenerateDeleteByFilterParam {
                        collection,
                        filter,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn delete_namespace(
            collection: crate::_rt::String,
            namespace: crate::_rt::String,
        ) -> Result<u32, crate::golem::vector::vectors::VectorError> {
            let durability = Durability::<
                Result<u32, crate::golem::vector::vectors::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "delete_namespace",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    <Impl as VectorGuest>::delete_namespace(collection.clone(), namespace.clone())
                });
                durability.persist_infallible(
                    GenerateDeleteNamespaceParam {
                        collection,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn delete_vectors(
            collection: crate::_rt::String,
            ids: crate::_rt::Vec<crate::golem::vector::vectors::Id>,
            namespace: Option<crate::_rt::String>,
        ) -> Result<u32, crate::golem::vector::vectors::VectorError> {
            let durability = Durability::<
                Result<u32, crate::golem::vector::vectors::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "delete_vectors",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::delete_vectors(collection.clone(), ids.clone(), namespace.clone())
                });
                durability.persist_infallible(
                    GenerateDeleteVectorsParam {
                        collection,
                        ids,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn get_vector(
            collection: crate::_rt::String,
            id: crate::golem::vector::vectors::Id,
            namespace: Option<crate::_rt::String>,
        ) -> Result<Option<VectorRecord>, crate::golem::vector::vectors::VectorError> {
            let durability = Durability::<
                Result<Option<VectorRecord>, crate::golem::vector::vectors::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "get_vector",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::get_vector(collection.clone(), id.clone(), namespace.clone())
                });
                durability.persist_infallible(
                    GenerateGetVectorParam {
                        collection,
                        id,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn get_vectors(
            collection: crate::_rt::String,
            ids: crate::_rt::Vec<crate::golem::vector::vectors::Id>,
            namespace: Option<crate::_rt::String>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
        ) -> Result<crate::_rt::Vec<VectorRecord>, crate::golem::vector::vectors::VectorError>
        {
            let durability = Durability::<
                Result<crate::_rt::Vec<VectorRecord>, crate::golem::vector::vectors::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "get_vectors",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::get_vectors(
                        collection.clone(),
                        ids.clone(),
                        namespace.clone(),
                        include_vectors.clone(),
                        include_metadata.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateGetVectorsParam {
                        collection,
                        ids,
                        namespace,
                        include_vectors,
                        include_metadata,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn list_vectors(
            collection: crate::_rt::String,
            namespace: Option<crate::_rt::String>,
            filter: Option<FilterExpression>,
            limit: Option<u32>,
            cursor: Option<crate::_rt::String>,
            include_vectors: Option<bool>,
            include_metadata: Option<bool>,
        ) -> Result<ListResponse, crate::golem::vector::vectors::VectorError> {
            let durability = Durability::<
                Result<ListResponse, crate::golem::vector::vectors::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "list_vectors",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::list_vectors(
                        collection.clone(),
                        namespace.clone(),
                        filter.clone(),
                        limit,
                        cursor.clone(),
                        include_vectors.clone(),
                        include_metadata.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateListVectorsParam {
                        collection,
                        namespace,
                        filter,
                        limit,
                        cursor,
                        include_vectors,
                        include_metadata,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn update_vector(
            collection: crate::_rt::String,
            id: crate::golem::vector::vectors::Id,
            vector: Option<VectorData>,
            metadata: Option<crate::golem::vector::vectors::Metadata>,
            namespace: Option<crate::_rt::String>,
            merge_metadata: Option<bool>,
        ) -> Result<(), crate::golem::vector::vectors::VectorError> {
            let durability = Durability::<
                Result<(), crate::golem::vector::vectors::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "update_vector",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::update_vector(
                        collection.clone(),
                        id.clone(),
                        vector.clone(),
                        metadata.clone(),
                        namespace.clone(),
                        merge_metadata.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateUpdateVectorParam {
                        collection,
                        id,
                        vector,
                        metadata,
                        namespace,
                        merge_metadata,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn upsert_vector(
            collection: crate::_rt::String,
            id: crate::golem::vector::vectors::Id,
            vector: VectorData,
            metadata: Option<crate::golem::vector::vectors::Metadata>,
            namespace: Option<crate::_rt::String>,
        ) -> Result<(), crate::golem::vector::vectors::VectorError> {
            let durability = Durability::<
                Result<(), crate::golem::vector::vectors::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "upsert_vector",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::upsert_vector(
                        collection.clone(),
                        id.clone(),
                        vector.clone(),
                        metadata.clone(),
                        namespace.clone(),
                    )
                });
                durability.persist_infallible(
                    GenerateUpsertVectorParam {
                        collection,
                        id,
                        vector,
                        metadata,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
        fn upsert_vectors(
            collection: crate::_rt::String,
            vectors: crate::_rt::Vec<VectorRecord>,
            namespace: Option<crate::_rt::String>,
        ) -> Result<BatchResult, crate::golem::vector::vectors::VectorError> {
            let durability = Durability::<
                Result<BatchResult, crate::golem::vector::vectors::VectorError>,
                UnusedError,
            >::new(
                "golem_vector",
                "upsert_vectors",
                DurableFunctionType::WriteRemote,
            );
            if durability.is_live() {
                let result = with_persistence_level(PersistenceLevel::PersistNothing, || {
                    Impl::upsert_vectors(collection.clone(), vectors.clone(), namespace.clone())
                });
                durability.persist_infallible(
                    GenerateUpsertVectorsParam {
                        collection,
                        vectors,
                        namespace,
                    },
                    result.clone(),
                )
            } else {
                durability.replay_infallible()
            }
        }
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateCountVectorsParam {
        collection: crate::_rt::String,
        filter: Option<FilterExpression>,
        namespace: Option<crate::_rt::String>,
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateDeleteByFilterParam {
        collection: crate::_rt::String,
        filter: FilterExpression,
        namespace: Option<crate::_rt::String>,
    }

    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateDeleteVectorsParam {
        collection: crate::_rt::String,
        ids: crate::_rt::Vec<crate::golem::vector::vectors::Id>,
        namespace: Option<crate::_rt::String>,
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateGetVectorParam {
        collection: crate::_rt::String,
        id: crate::golem::vector::vectors::Id,
        namespace: Option<crate::_rt::String>,
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateGetVectorsParam {
        collection: crate::_rt::String,
        ids: crate::_rt::Vec<crate::golem::vector::vectors::Id>,
        namespace: Option<crate::_rt::String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateListVectorsParam {
        collection: crate::_rt::String,
        namespace: Option<crate::_rt::String>,
        filter: Option<FilterExpression>,
        limit: Option<u32>,
        cursor: Option<crate::_rt::String>,
        include_vectors: Option<bool>,
        include_metadata: Option<bool>,
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateUpdateVectorParam {
        collection: crate::_rt::String,
        id: crate::golem::vector::vectors::Id,
        vector: Option<VectorData>,
        metadata: Option<crate::golem::vector::vectors::Metadata>,
        namespace: Option<crate::_rt::String>,
        merge_metadata: Option<bool>,
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateUpsertVectorParam {
        collection: crate::_rt::String,
        id: crate::golem::vector::vectors::Id,
        vector: VectorData,
        metadata: Option<crate::golem::vector::vectors::Metadata>,
        namespace: Option<crate::_rt::String>,
    }
    #[derive(Debug, Clone, PartialEq, IntoValue, FromValueAndType)]
    struct GenerateUpsertVectorsParam {
        collection: crate::_rt::String,
        vectors: crate::_rt::Vec<VectorRecord>,
        namespace: Option<crate::_rt::String>,
    }

    #[cfg(test)]
    mod tests {
        use crate::golem::vector::types::{
            BinaryVector, FilterCondition, FilterExpression, FilterOp, FilterOperator,
            FlatMetadataValue, VectorData, VectorRecord,
        };

        use crate::golem::vector::types::FilterNode;
        use golem_rust::value_and_type::{FromValueAndType, IntoValueAndType};
        use std::fmt::Debug;

        fn roundtrip_test<T: Debug + Clone + PartialEq + IntoValueAndType + FromValueAndType>(
            value: T,
        ) {
            let vnt = value.clone().into_value_and_type();
            let extracted = T::from_value_and_type(vnt).unwrap();
            assert_eq!(value, extracted);
        }

        #[test]
        fn filter_expression_roundtrip() {
            let filter = FilterExpression {
                nodes: vec![FilterNode {
                    id: 123,
                    op: FilterOp::And,
                    condition: Some(FilterCondition {
                        field: "test_field".to_string(),
                        operator: FilterOperator::Eq,
                        value: FlatMetadataValue::StringVal("test_value".to_string()),
                    }),
                    inputs: vec![1, 2, 3, 4],
                }],
                root_id: 123,
            };
            roundtrip_test(filter);
            let filter2 = FilterExpression {
                nodes: vec![FilterNode {
                    id: 123,
                    op: FilterOp::Or,
                    condition: Some(FilterCondition {
                        field: "test_field".to_string(),
                        operator: FilterOperator::Gt,
                        value: FlatMetadataValue::NumberVal(42.0),
                    }),
                    inputs: vec![1, 2],
                }],
                root_id: 123,
            };
            roundtrip_test(filter2);
            let filter3 = FilterExpression {
                nodes: vec![FilterNode {
                    id: 123,
                    op: FilterOp::Not,
                    condition: None,
                    inputs: vec![1, 2],
                }],
                root_id: 123,
            };
            roundtrip_test(filter3);
        }

        #[test]
        fn vector_record_roundtrip() {
            let vector_record = VectorRecord {
                id: "test_id".to_string(),
                vector: VectorData::Binary(BinaryVector {
                    data: vec![1, 2, 3, 4],
                    dimensions: 4,
                }),
                metadata: Some(vec![
                    (
                        "key1".to_string(),
                        FlatMetadataValue::StringVal("value1".to_string()),
                    ),
                    ("key2".to_string(), FlatMetadataValue::NumberVal(3.14)),
                ]),
            };
            roundtrip_test(vector_record);
        }
    }
}

// --- Structs we are using ---
