use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_warp::graphql_subscription;
use hyper::server::{conn::AddrIncoming, Builder, Server};
use listenfd::ListenFd;
use sqlx::postgres::PgPool;
use std::convert::Infallible;
use warp::{http::Response, Filter, Rejection, Reply};

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

    // GraphQL endpoints
    let schema = graphql::schema(pool);
    let graphql = {
        let query = warp::post()
            .and(async_graphql_warp::graphql(schema.clone()))
            .and_then(
                |(schema, request): (graphql::SchemaRoot, async_graphql::Request)| async move {
                    let resp = schema.execute(request).await;
                    Ok::<async_graphql_warp::Response, Infallible>(resp.into())
                },
            );
        let playground = warp::get().map(|| {
            Response::builder()
                .header("Content-Type", "text/html")
                .body(playground_source(
                    GraphQLPlaygroundConfig::new("/graphql")
                        .subscription_endpoint("/graphql/subscribe"),
                ))
        });
        let subscriptions = graphql_subscription(schema);
        warp::path("graphql").and(
            warp::path::end()
                .and(query.or(playground))
                .or(warp::path("subscribe")
                    .and(warp::path::end())
                    .and(subscriptions)),
        )
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
        async move { Ok::<_, Infallible>(svc) }
    });

    println!("Server listening on http://localhost:{}", port);
    make_server(port)?.serve(make_svc).await?;
    Ok(())
}
