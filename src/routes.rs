use warp::{Filter, Rejection, Reply};

pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // Static files, from create-react-app build output
    let files = warp::fs::dir("app/build").or(warp::fs::file("app/build/index.html"));

    // Actual API endpoints
    let api = {
        let hello = warp::path!("hello").map(hello);
        let echo = warp::path!("echo" / String).map(echo);
        warp::path("api").and(hello.or(echo))
    };

    api.or(files).with(warp::trace::request())
}

fn hello() -> impl Reply {
    "Hello world!"
}

fn echo(message: String) -> impl Reply {
    message
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    use super::routes;

    #[tokio::test]
    async fn test_hello() {
        let api = routes();
        let resp = request().method("GET").path("/api/hello").reply(&api).await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body(), "Hello world!");
    }

    #[tokio::test]
    async fn test_echo() {
        let api = routes();
        let resp = request()
            .method("GET")
            .path("/api/echo/foo")
            .reply(&api)
            .await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body(), "foo");
    }
}
