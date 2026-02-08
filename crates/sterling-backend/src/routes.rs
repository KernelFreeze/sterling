use crate::state::State;

pub type Router = axum::Router<State>;

pub fn router() -> Router {
    Router::new()
}
