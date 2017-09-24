use iron::prelude::*;
use iron::status;
use iron_json_response::JsonResponse;
use std::io::{self, Read};
use super::parse::normalise::normalise;
use super::parse::search::{tokenise, Expression};

fn bad_io(e: io::Error) -> Response {
    let err = json!({
        "error": "io",
        "reason": "Something went wrong reading the body of the request.",
        "details": format!("{}", e)
    });

    let mut resp = Response::new();
    resp.set_mut(JsonResponse::json(err))
        .set_mut(status::InternalServerError);
    resp
}

fn bad_parse(details: Vec<String>) -> Response {
    let err = json!({
        "error": "parse",
        "reason": "The format of the search query is bad.",
        "details": details
    });

    let mut resp = Response::new();
    resp.set_mut(JsonResponse::json(err))
        .set_mut(status::BadRequest);
    resp
}

fn good(normed: String, parsed: Expression) -> Response {
    let err = json!({
        "error": false,
        "normalised": normed,
        "parsed": parsed
    });

    let mut resp = Response::new();
    resp.set_mut(JsonResponse::json(err)).set_mut(status::Ok);
    resp
}

pub fn parse(req: &mut Request) -> IronResult<Response> {
    let mut query = String::new();
    if let Err(e) = req.body.read_to_string(&mut query) {
        return Ok(bad_io(e));
    }

    debug!("Query: {}", query);
    let normed = normalise(query);
    let data = tokenise(normed.clone());
    debug!("Result: {:?}", data);

    Ok(match data {
        Err(details) => bad_parse(details),
        Ok(tokens) => good(normed, tokens),
    })
}
