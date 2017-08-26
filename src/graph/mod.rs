use juniper::Context;

pub struct Root {}
impl Context for Root {}

impl Root {
    pub fn new() -> Root {
        Root {}
    }
}

graphql_object!(Root: Root as "Query" |&self| {
    description: "The root query object of the schema"

    field ping() -> String {
        info!("QUÆSTIO ping");
        "pong".to_string()
    }
});

// Query: QUÆSTIO
// Mutation: MUTATIONEM
