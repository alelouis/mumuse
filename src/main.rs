extern crate midir;

mod conversions;
mod messages;
mod midi;
mod music;
mod tests;

fn main() {
    midi::show_input_ports();
    midi::receive("Arturia KeyStep 32".to_string());
}
