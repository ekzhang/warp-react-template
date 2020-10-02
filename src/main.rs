use hyper::server::{conn::AddrIncoming, Builder, Server};
use juniper_warp::{make_graphql_filter, playground_filter};
use listenfd::ListenFd;
use sqlx::postgres::PgPool;
use warp::{Filter, Rejection, Reply};

mod graphql;

fn make_server(port: u16) -> hyper::Result<Builder<AddrIncoming>> {
    let mut listenfd = ListenFd::from_env();
    if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        Server::from_tcp(l)
    } else {
        Ok(Server::bind(&([0, 0, 0, 0], port).into()))
    }
}

fn routes(pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // Static files, from create-react-app build output
    let files = warp::fs::dir("app/build").or(warp::fs::file("app/build/index.html"));

    let state = graphql::Context::new(pool);
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    let port = std::env::var("PORT")
        .unwrap_or("3535".to_string())
        .parse()?;

    let svc = warp::service(routes(pool));
    let make_svc = hyper::service::make_service_fn(|_| {
        let svc = svc.clone();
        async move { Ok::<_, std::convert::Infallible>(svc) }
    });

    println!("Server listening on http://localhost:{}", port);
    make_server(port)?.serve(make_svc).await?;
    Ok(())
}
