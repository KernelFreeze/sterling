use std::sync::Arc;

use typed_builder::TypedBuilder;

#[derive(Debug, Clone)]
pub struct State(Arc<StateInner>);

#[derive(Debug, TypedBuilder)]
struct StateInner {}

impl State {
    pub fn new() -> Self {
        let inner = StateInner::builder().build();
        State(Arc::new(inner))
    }
}
