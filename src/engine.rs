/// Root module for the whole program, instanciated at the start.
/// Contains the engine, handling the current state of the program (menu,
/// simulation running, simulation paused), instanciates all the other
/// modules (automatons, cameras, etc.) and handles the simulation
/// each tick before requesting the rendering of the frame
use crate::automatons::*;
use macroquad::ui;
use macroquad::input;

#[derive(PartialEq)]
enum EngineState {
    Menu,
    Running,
    Paused
}

pub struct Engine {
    state: EngineState,
    automatons: Vec<Box<dyn Automaton>>,
    currentAutomaton: Option<Box<dyn Automaton>>
}

impl Engine {
    pub fn new(automatons: Vec<Box<dyn Automaton>>) -> Engine {
        Engine {
            state: EngineState::Menu,
            automatons,
            currentAutomaton: None
        }
    }

    /// Called each tick, handles all the stuff (input, simulation iteration, etc.)
    /// and finally requests the rendering of the frame
    pub fn update(&mut self) {
        // Input - @TODO
        // Simulation
        match self.state {
            EngineState::Menu => println!("Menu"),
            EngineState::Running => println!("Running"),
            EngineState::Paused => println!("Paused")
        }

        if input::is_key_pressed(input::KeyCode::Space) {
            match self.state {
                EngineState::Running => self.state = EngineState::Paused,
                EngineState::Paused => self.state = EngineState::Running,
                _ => ()
            }
        }

        if self.state == EngineState::Running {
            self.currentAutomaton.as_mut().expect("NO_AUTOMATON").iterate();
        }
        if [EngineState::Running, EngineState::Paused].contains(&self.state) {
            self.currentAutomaton.as_mut().expect("NO_AUTOMATON").render();
        }
    }

    pub fn render(&mut self) {
        if self.state == EngineState::Menu {
            if ui::root_ui().button(None, "Start the simulation") {
                self.currentAutomaton = Some(Box::new(GameOfLife::new(40, 40)));
                self.state = EngineState::Running;
            }
        }
    }
}
