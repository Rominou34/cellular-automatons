/// Module containing the automatons. Each automaton is a struct using the
/// Automaton trait and declaring these functions: new(), iterate(), render()
enum AutomatonType {
    Elementary,
    Grid
}

// pub struct Automaton {
//     name: String,
//     automaton_type: AutomatonType
// }

pub trait Automaton {
    fn iterate(&self);
    fn render(&self);
}
