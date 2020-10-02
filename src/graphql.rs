use juniper::{EmptySubscription, FieldResult};
use sqlx::postgres::PgPool;
use uuid::Uuid;

#[derive(juniper::GraphQLEnum, sqlx::Type, Copy, Clone, Debug)]
#[sqlx(rename = "episode", rename_all = "lowercase")]
enum Episode {
    #[sqlx(rename = "new hope")]
    NewHope,
    Empire,
    Jedi,
}

/// A humanoid creature in the Star Wars universe
#[derive(juniper::GraphQLObject, Clone)]
struct Human {
    id: Uuid,
    name: String,
    appears_in: Option<Episode>,
    home_planet: String,
}

/// A humanoid creature in the Star Wars universe
#[derive(juniper::GraphQLInputObject)]
struct NewHuman {
    name: String,
    appears_in: Option<Episode>,
    home_planet: String,
}

#[derive(Clone)]
pub struct Context {
    pool: PgPool,
}

impl Context {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    fn api_version() -> &str {
        "0.1"
    }

    async fn human(context: &Context, id: Uuid) -> FieldResult<Human> {
        sqlx::query_as_unchecked!(Human, "SELECT * FROM humans WHERE id = $1", id)
            .fetch_one(&context.pool)
            .await
            .map_err(|e| e.into())
    }

    async fn humans(context: &Context) -> FieldResult<Vec<Human>> {
        sqlx::query_as_unchecked!(Human, "SELECT * FROM humans")
            .fetch_all(&context.pool)
            .await
            .map_err(|e| e.into())
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn create_human(context: &Context, new_human: NewHuman) -> FieldResult<Human> {
        sqlx::query_as_unchecked!(
            Human,
            "
INSERT INTO humans (name, appears_in, home_planet)
    VALUES ($1, $2, $3)
RETURNING *
",
            new_human.name,
            new_human.appears_in,
            new_human.home_planet,
        )
        .fetch_one(&context.pool)
        .await
        .map_err(|e| e.into())
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
