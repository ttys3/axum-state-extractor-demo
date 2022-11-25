//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-hello-world
//! ```

use axum::extract::State;
use axum::response::IntoResponse;
use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // this will not work:
    // let inner_router: Router<InnerState> = Router::new()

    // this works (auto infer):
    // let inner_router = Router::new()
    //     .route("/inner", get(inner_handler))
    //     .with_state(InnerState {});

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/outer", get(outer_handler))
        .merge(get_sub_router())
        .with_state(OuterState {});

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// just return `-> Router` will not work
// work: -> Router<OuterState>     , it works!
// not work: -> Router<InnerState> , the trait `From<Router<InnerState>>` is not implemented for `Router<OuterState, _>`
// not work: -> Router<()>         , the trait `From<Router>` is not implemented for `Router<OuterState, _>`
// not work: -> Router             , the trait `From<Router>` is not implemented for `Router<OuterState, _>`
fn get_sub_router<T>() -> Router<T> {
    Router::new()
        .route("/inner", get(inner_handler))
        .with_state(InnerState {})
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[derive(Clone)]
struct InnerState {}

#[derive(Clone)]
struct OuterState {}

async fn inner_handler(_state: State<InnerState>) -> impl IntoResponse {
    "inner"
}

async fn outer_handler(_state: State<OuterState>) -> impl IntoResponse {
    "outer"
}

#[cfg(test)]
mod tests {
    use crate::{get_sub_router, handler, outer_handler, OuterState};
    use axum::routing::get;
    use axum::Router;

    #[test]
    fn test_new_router_with_state_without_used() {
        // without the `Router<OuterState>` type annotation, it will not work and result in:
        // error[E0282]: type annotations needed for `Router<S2>`
        let _app: Router<OuterState> = Router::new()
            .route("/", get(handler))
            .route("/outer", get(outer_handler))
            .merge(get_sub_router())
            .with_state(OuterState {});
    }
}
