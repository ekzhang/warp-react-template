use juniper::{EmptySubscription, FieldResult};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(juniper::GraphQLEnum, Clone)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

/// A humanoid creature in the Star Wars universe
#[derive(juniper::GraphQLObject, Clone)]
struct Human {
    id: Uuid,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

/// A humanoid creature in the Star Wars universe
#[derive(juniper::GraphQLInputObject)]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(Default, Clone)]
pub struct Context {
    db: Arc<RwLock<Vec<Human>>>,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    fn human(_context: &Context, id: Uuid) -> FieldResult<Human> {
        let h = Human {
            id,
            name: "Eric".to_string(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_string(),
        };
        Ok(h)
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    fn createHuman(context: &Context, new_human: NewHuman) -> FieldResult<Human> {
        let human = Human {
            id: Uuid::new_v4(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        };
        context.db.write().unwrap().push(human.clone());
        Ok(human)
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
