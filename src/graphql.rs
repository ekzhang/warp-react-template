use async_graphql::*;
use sqlx::postgres::PgPool;
use uuid::Uuid;

#[derive(Enum, sqlx::Type, Copy, Clone, Eq, PartialEq)]
#[sqlx(rename = "episode", rename_all = "lowercase")]
enum Episode {
    #[sqlx(rename = "new hope")]
    NewHope,
    Empire,
    Jedi,
}

/// A humanoid creature in the Star Wars universe
#[derive(SimpleObject, Clone)]
struct Human {
    id: Uuid,
    name: String,
    appears_in: Option<Episode>,
    home_planet: String,
}

/// A humanoid creature in the Star Wars universe
#[derive(InputObject)]
struct NewHuman {
    name: String,
    appears_in: Option<Episode>,
    home_planet: String,
}

pub struct Query {
    pool: PgPool,
}

impl Query {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[Object]
impl Query {
    async fn api_version(&self) -> &str {
        "0.1"
    }

    async fn human(&self, id: Uuid) -> Result<Human> {
        sqlx::query_as_unchecked!(Human, "SELECT * FROM humans WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    async fn humans(&self) -> Result<Vec<Human>> {
        sqlx::query_as_unchecked!(Human, "SELECT * FROM humans")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }
}

pub struct Mutation {
    pool: PgPool,
}

impl Mutation {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[Object]
impl Mutation {
    async fn create_human(&self, new_human: NewHuman) -> Result<Human> {
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
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.into())
    }
}

pub type SchemaRoot = Schema<Query, Mutation, EmptySubscription>;

pub fn schema(pool: PgPool) -> SchemaRoot {
    Schema::build(
        Query::new(pool.clone()),
        Mutation::new(pool.clone()),
        EmptySubscription,
    )
    .finish()
}
