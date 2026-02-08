use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct State(Arc<StateInner>);

#[derive(Debug)]
struct StateInner {}

impl Default for State {
    fn default() -> Self {
        State(Arc::new(StateInner {}))
    }
}
