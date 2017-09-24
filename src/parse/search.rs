use pest::inputs::StrInput;
use pest::iterators::{Pair, Pairs};
use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("search.pest");

#[derive(Parser)]
#[grammar = "parse/search.pest"]
struct Searcher;

pub type Expression = Vec<Token>;

#[derive(Debug, Serialize)]
pub enum Token {
    Word(String),
    Pair(String, String),
    Id(usize),
    Quote(String),
    LogicAnd(()),
    LogicNot(()),
    LogicOr(()),
    Group(Expression)
}

pub fn tokenise(input: String) -> Result<Expression, Vec<String>> {
    let top = parse(&input)?;

    let tokens = expression(top);
    debug!("{:?}", tokens);

    Ok(tokens)
}

fn parse(input: &str) -> Result<Pairs<Rule, StrInput>, Vec<String>> {
    match Searcher::parse_str(Rule::search, input) {
        Ok(mut pairs) => match pairs.next() {
            None => Err(vec!("no input".into())),
            Some(p) => Ok(p.into_inner())
        },
        Err(err) => Err(format!("{}", err)
            .lines()
            .map(|l| format!("{}", l))
            .collect())
    }
}

fn expression(pairs: Pairs<Rule, StrInput>) -> Expression {
    let mut tokens = vec![];
    for tok in pairs.clone() {
        match tok.as_rule() {
            Rule::word => tokens.push(Token::Word(ex(tok))),
            Rule::quote => tokens.push(Token::Quote(quote_ex(tok))),
            Rule::logic_and => tokens.push(Token::LogicAnd(())),
            Rule::logic_not => tokens.push(Token::LogicNot(())),
            Rule::logic_or => tokens.push(Token::LogicOr(())),
            Rule::id => {
                let s = ex(tok);
                let maybe_num = s.trim_left_matches('#').parse::<usize>();
                tokens.push(match maybe_num {
                    Err(_) => Token::Word(s),
                    Ok(n) => Token::Id(n)
                });
            },
            Rule::pair => match pair_ex(tok) {
                None => {},
                Some(p) => tokens.push(Token::Pair(p.0, p.1))
            },
            Rule::group => match group_ex(tok) {
                None => {},
                Some(e) => tokens.push(Token::Group(e))
            },
            _ => {}
        };
    }

    tokens
}

fn ex(tok: Pair<Rule, StrInput>) -> String {
    tok.into_span().as_str().into()
}

fn quote_ex(tok: Pair<Rule, StrInput>) -> String {
    let quoted = ex(tok);
    quoted[1..(quoted.len()-1)].into()
}

fn pair_ex(tok: Pair<Rule, StrInput>) -> Option<(String, String)> {
    let mut inner = tok.into_inner();

    let key;
    match inner.next() {
        None => return None,
        Some(k) => { key = ex(k); }
    };

    let value;
    match inner.next() {
        None => return None,
        Some(v) => { value = ex(v); }
    };

    Some((key, value))
}

fn group_ex(tok: Pair<Rule, StrInput>) -> Option<Expression> {
    let expr;
    match tok.into_inner().next() {
        None => return None,
        Some(e) => match e.as_rule() {
            Rule::expression => { expr = e; },
            _ => return None
        }
    };

    let inner = expr.into_inner();
    debug!("Inner {:?}", inner);

    Some(expression(inner))
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
fn pairs() {
    //           0|       10|       20|        30|      38|
    test_parser!("word pair=2 (n=tuple word) \"pair=false\"", [
        expression(0, 39, [
            word(0, 4),
            pair(5, 11, [
                key(5, 9),
                value(10, 11)
            ]),
            group(12, 26, [
                expression(13, 25, [
                    pair(13, 20, [
                        key(13, 14),
                        value(15, 20)
                    ]),
                    word(21, 25)
                ])
            ]),
            quote(27, 39)
        ])
    ]);
}

#[test]
fn logic() {
    //           0|       10|       20|         30|
    test_parser!("hamlet and to=be or not \"to be\"", [
        expression(0, 31, [
            word(0, 6),
            logic_and(7, 10),
            pair(11, 16, [
                key(11, 13),
                value(14, 16)
            ]),
            logic_or(17, 19),
            logic_not(20, 23),
            quote(24, 31)
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
