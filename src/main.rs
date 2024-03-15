mod engine;
mod automatons;

use macroquad::prelude::*;
use ::rand::prelude::*;
use std::{thread, time};

use crate::engine::Engine;
use crate::automatons::Automaton;

#[macroquad::main("cellular-automatons")]
async fn main() {
    let automatons = Vec::new();
    let mut engine = Engine::new(automatons);

    loop {
        engine.update();
        engine.render();

        //let one_tenth = time::Duration::from_millis(100);
        //thread::sleep(one_tenth);
        next_frame().await
    }
}