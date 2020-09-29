use juniper_warp::{make_graphql_filter, playground_filter};
use warp::{Filter, Rejection, Reply};

use crate::graphql;

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // Static files, from create-react-app build output
    let files = warp::fs::dir("app/build").or(warp::fs::file("app/build/index.html"));

    let state = graphql::Context::default();
    let context = warp::any().map(move || state.clone());

    // GraphQL endpoints
    let graphql = {
        let query = warp::path("query")
            .and(warp::path::end())
            .and(warp::post())
            .and(make_graphql_filter(graphql::schema(), context.boxed()));

        let playground = warp::path("playground")
            .and(warp::path::end())
            .and(playground_filter(
                "/graphql/query",
                Some("/graphql/subscriptions"),
            ));

        warp::path("graphql").and(query.or(playground))
    };

    graphql.or(files).with(warp::trace::request())
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    use super::routes;

    fn graphql_payload(query: &str) -> serde_json::Value {
        serde_json::json!({
            "operationName": null,
            "query": query,
            "variables": {}
        })
    }

    #[tokio::test]
    async fn test_hello() {
        let api = routes();
        let resp = request()
            .method("POST")
            .path("/graphql/query")
            .json(&graphql_payload(
                r#"
                query {
                    human(id: "93aef7bf-0dd2-4790-8f29-9987b0b21c81") {
                        name
                    }
                }
                "#,
            ))
            .reply(&api)
            .await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body(), "{\"data\":{\"human\":{\"name\":\"Eric\"}}}");
    }
}
