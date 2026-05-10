extern crate fastly;
mod game;
mod moves;

use std::convert::TryInto;
use fastly::http::{header, Method, StatusCode};
use fastly::{Error, Request, Response};
use crate::game::{Game, Player};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Log service version
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    match req.get_method() {
        &Method::POST => {
            let game: Game = req.into_body_str().try_into().unwrap();
            let best_move = game.compute_best_move(Player::White);
            Ok(Response::from_status(StatusCode::OK)
                .with_body_text_plain(&best_move))
        }

        _ => Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
            .with_header(header::ALLOW, "POST")
            .with_body_text_plain("Send a POST request with the game state as the body\n")),
    }
}
