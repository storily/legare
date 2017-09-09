use juniper::Context;
use parse::normalise::normalise;
use parse::search::tokenise;

pub struct Root {}
impl Context for Root {}

impl Root {
    pub fn new() -> Root {
        Root {}
    }
}

graphql_object!(Root: Root as "Query" |&self| {
    description: "Normalising and parsing services for Cogitare.nz"

    field hello() -> Vec<String> {
        vec![
            "Mihi est hic?".into(),
            "Tu exaudi me?".into(),
            "Si vales bene est, ego valeo.".into()
        ]
    }

    field normalise(search: String) -> String {
        normalise(search)
    }

    field parse(search: String) -> Vec<String> {
        let normed = normalise(search);
        if normed.len() == 0 {
            vec![]
        } else {
            tokenise(normed)
        }
    }
});

