extern crate midir;

mod conversions;
mod messages;
mod midi;
mod music;
mod tests;

fn main() {
    midi::show_output_ports();
    midi::show_input_ports();
    midi::receive("Arturia KeyStep 32".to_string());
    midi::send("Virtual Midi Bus 1".to_string());
}
