use iron::prelude::*;
use iron::status;
use iron_json_response::JsonResponse;
use std::io::{self, Read};
use super::parse::normalise::normalise;
use super::parse::search::tokenise;

fn io_error(e: io::Error) -> Response {
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

fn parse_error(details: Vec<String>) -> Response {
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

fn good(normed: String, parsed: Vec<String>) -> Response {
    let err = json!({
        "error": false,
        "normalised": normed,
        "parsed": parsed
    });

    let mut resp = Response::new();
    resp.set_mut(JsonResponse::json(err))
        .set_mut(status::Ok);
    resp
}

pub fn parse(req: &mut Request) -> IronResult<Response> {
    let mut query = String::new();
    if let Err(e) = req.body.read_to_string(&mut query) {
        return Ok(io_error(e))
    }

    debug!("Query: {}", query);
    let normed = normalise(query);
    let data = tokenise(normed.clone());
    debug!("Result: {:?}", data);

    match data {
        Err(details) => Ok(parse_error(details)),
        Ok(tokens) => Ok(good(normed, tokens))
    }
}
