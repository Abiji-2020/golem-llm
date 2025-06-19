use golem_vector::durability::{DurableVector, ExtendedGuest};

pub mod analytics;
pub mod client;
pub mod collections;
pub mod connection;
pub mod namespaces;
pub mod search;
pub mod search_extended;
pub mod vectors;

pub struct QdrantComponent;

impl ExtendedGuest for QdrantComponent {}

type DurableQdrantComponent = DurableVector<QdrantComponent>;

golem_vector::export_vector!(
    DurableQdrantComponent with_types_in golem_vector
);
