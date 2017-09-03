use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("search.pest");

#[derive(Parser)]
#[grammar = "parse/search.pest"]
struct Searcher;

pub fn tokenise(input: String) -> Vec<String> {
    let maybe_expr = Searcher::parse_str(Rule::search, &input);
    match maybe_expr {
        Err(err) => format!("{}", err)
            .lines()
            .map(|l| format!("{}", l))
            .collect(),
        Ok(mut pairs) => pairs.next().unwrap().into_inner().filter_map(|pair| match pair.as_rule() {
            rule @ _ => {
                Some(format!("{:?} => {}", rule, pair.clone().into_span().as_str()))
            }
        })
        .collect()
    }
}

#[cfg(test)]
macro_rules! test_parser {
    ($input:expr, $tokens:tt) => (parses_to! {
        parser: Searcher,
        input: $input,
        rule: Rule::search,
        tokens: $tokens
    })
}

#[test]
fn single_word() {
    test_parser!("word", [
        expression(0, 4, [
            word(0, 4)
        ])
    ]);
}

#[test]
fn multiple_words() {
    test_parser!("one two three", [
        expression(0, 13, [
            word(0, 3),
            word(4, 7),
            word(8, 13)
        ])
    ]);
}

#[test]
fn ids() {
    //           0|       10|        20|  |22
    test_parser!("#123 #ash (#123) \"#123\"", [
        expression(0, 23, [
            id(0, 4),
            word(5, 9),
            group(10, 16, [
                expression(11, 15, [
                    id(11, 15)
                ])
            ]),
            quote(17, 23)
        ])
    ]);
}

#[test]
fn empty_group() {
    test_parser!("()", [
        expression(0, 2, [
            group(0, 2)
        ])
    ]);
}

#[test]
fn single_1group() {
    test_parser!("(item)", [
        expression(0, 6, [
            group(0, 6, [
                expression(1, 5, [
                    word(1, 5)
                ])
            ])
        ])
    ]);
}

#[test]
fn single_3group() {
    test_parser!("(one two three)", [
        expression(0, 15, [
            group(0, 15, [
                expression(1, 14, [
                    word(1, 4),
                    word(5, 8),
                    word(9, 14)
                ])
            ])
        ])
    ]);
}

#[test]
fn spaced_groups() {
    //           0|       10|       20|       30|       40|       50|     59|
    test_parser!("( spaced ) (tight) ( lefty) (righty ) ( a b ) (c d ) ( e f)", [
        expression(0, 59, [
            group(0, 10, [ // spaced
                expression(2, 8, [
                    word(2, 8)
                ])
            ]),

            group(11, 18, [ // tight
                expression(12, 17, [
                    word(12, 17)
                ])
            ]),

            group(19, 27, [ // lefty
                expression(21, 26, [
                    word(21, 26)
                ])
            ]),

            group(28, 37, [ // righty
                expression(29, 35, [
                    word(29, 35)
                ])
            ]),

            group(38, 45, [ // a b
                expression(40, 43, [
                    word(40, 41),
                    word(42, 43)
                ])
            ]),

            group(46, 52, [ // c d
                expression(47, 50, [
                    word(47, 48),
                    word(49, 50)
                ])
            ]),

            group(53, 59, [ // e f
                expression(55, 58, [
                    word(55, 56),
                    word(57, 58)
                ])
            ])
        ])
    ]);
}

#[test]
fn nested_groups() {
    test_parser!("( outer (inner ))", [
        expression(0, 17, [
            group(0, 17, [
                expression(2, 16, [
                    word(2, 7),
                    group(8, 16, [
                        expression(9, 14, [
                            word(9, 14)
                        ])
                    ])
                ])
            ])
        ])
    ]);
}

#[test]
fn quotes() {
    //           0|          10|         20|           30|
    test_parser!("a (b \"c\") \"d e\" \"f (g) h\" \")\" \"\"", [
        expression(0, 32, [
            word(0, 1),
            group(2, 9, [
                expression(3, 8, [
                    word(3, 4),
                    quote(5, 8)
                ])
            ]),
            quote(10, 15),
            quote(16, 25),
            quote(26, 29),
            quote(30, 32)
        ])
    ]);
}
