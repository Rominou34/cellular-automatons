/// Root module for the whole program, instanciated at the start.
/// Contains the engine, handling the current state of the program (menu,
/// simulation running, simulation paused), instanciates all the other
/// modules (automatons, cameras, etc.) and handles the simulation
/// each tick before requesting the rendering of the frame
use crate::automatons::Automaton;

enum EngineState {
    Menu,
    Running,
    Paused
}

pub struct Engine {
    state: EngineState,
    automatons: Vec<Box<dyn Automaton>>
}

impl Engine {
    pub fn new(automatons: Vec<Box<dyn Automaton>>) -> Engine {
        Engine { state: EngineState::Menu, automatons}
    }

    /// Called each tick, handles all the stuff (input, simulation iteration, etc.)
    /// and finally requests the rendering of the frame
    pub fn tick(&self) {
        // Input - @TODO
        println!("Test");
        // Simulation
        // Rendering
    }
}
