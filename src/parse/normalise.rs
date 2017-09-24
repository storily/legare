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
                Some(
                    chars
                        .map(|c| match c.as_rule() {
                            Rule::double_quote => "\"".into(),
                            Rule::single_quote => "'".into(),
                            Rule::dash => "-".into(),
                            Rule::left_bracket => "(".into(),
                            Rule::right_bracket => ")".into(),
                            Rule::equals => "=".into(),
                            _ => c.clone().into_span().as_str().into(),
                        })
                        .collect::<Vec<String>>()
                        .join(""),
                )
            }
            _ => None,
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
macro_rules! test_normaliser {
    ($input:expr, $output:expr) =>
    (assert_eq!(normalise(format!("{}", $input)), $output))
}

#[cfg(test)]
macro_rules! test_spaces {
    ($input:expr) =>
    (test_normaliser!(format!("x{}x", $input), "x x"))
}

#[test]
fn spaces() {
    // Usual spaces
    test_spaces!("\n");
    test_spaces!("\t");
    test_spaces!(" ");
    test_spaces!("　"); // IDEOGRAPHIC SPACE

    // Unusual spaces
    test_spaces!(" "); // NBSP
    test_spaces!(" "); // EN QUAD
    test_spaces!(" "); // EM QUAD
    test_spaces!(" "); // EN SPACE
    test_spaces!(" "); // EM SPACE
    test_spaces!(" "); // FIGURE SPACE

    // Rare spaces
    test_spaces!("\u{85}");
    test_spaces!("\u{2004}");
    test_spaces!("\u{2005}");
    test_spaces!("\u{2006}");
    test_spaces!("\u{2008}");
    test_spaces!("\u{2009}");
    test_spaces!("\u{200A}");
    test_spaces!("\u{202F}");
    test_spaces!("\u{205F}");

    // Space collapsing
    test_spaces!("      ");
    test_spaces!(" \n\t ");
}

#[test]
fn double_quotes() {
    test_normaliser!("\"", "\"");
    test_normaliser!("“", "\"");
    test_normaliser!("”", "\"");
    test_normaliser!("«", "\"");
    test_normaliser!("»", "\"");
}

#[test]
fn single_quotes() {
    test_normaliser!("'", "'");
    test_normaliser!("‘", "'");
    test_normaliser!("’", "'");
    test_normaliser!("‹", "'");
    test_normaliser!("›", "'");
}

#[test]
fn brackets() {
    test_normaliser!("(", "(");
    test_normaliser!("{", "(");
    test_normaliser!("[", "(");

    test_normaliser!(")", ")");
    test_normaliser!("}", ")");
    test_normaliser!("]", ")");
}

#[test]
fn equals() {
    test_normaliser!("=", "=");
    test_normaliser!(":", "=");
    test_normaliser!("≈", "=");
}

#[test]
fn dashes() {
    test_normaliser!("-", "-");
    test_normaliser!("--", "-");
    test_normaliser!("—", "-");
    test_normaliser!("–", "-");
    test_normaliser!("‒", "-");
    test_normaliser!("⁓", "-");
    test_normaliser!("ー", "-");
    test_normaliser!("―", "-");
    test_normaliser!("⸺", "-");
    test_normaliser!("⸻", "-");
    test_normaliser!("〜", "-");
    test_normaliser!("～", "-");
}
