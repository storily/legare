use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("normalise.pest");

#[derive(Parser)]
#[grammar = "parse/normalise.pest"]
struct Normaliser;

pub fn normalise(input: String) -> String {
    Normaliser::parse_str(Rule::space_split, &(input.to_lowercase()))
    .unwrap()
    .filter_map(|pair| match pair.as_rule() {
        Rule::not_space => {
            let chars = pair.clone().into_inner();
            Some(chars.map(|c| match c.as_rule() {
                Rule::double_quote => "\"".into(),
                Rule::single_quote => "'".into(),
                Rule::dash => "-".into(),
                Rule::left_bracket => "(".into(),
                Rule::right_bracket => ")".into(),
                Rule::equals => "=".into(),
                _ => c.clone().into_span().as_str().into()
            }).collect::<Vec<String>>().join(""))
        },
        _ => None
    })
    .collect::<Vec<String>>()
    .join(" ")
}
