use midir::{MidiInput, MidiInputPort};
use std::io::stdin;
use crate::messages::message;

// Lists available input port devices 
pub fn show_input_ports() {
    let midi_in = MidiInput::new("midi_in").expect("Could not open midi input.");
    for p in midi_in.ports().iter() {
        println!("{}", midi_in.port_name(&p).unwrap());
    }
}

// Finds port for a given string name 
fn get_port_index_by_name(midi_in: &MidiInput, name: String) -> Option<usize> {
    let mut port_index: Option<usize> = None;
    for (i, p) in midi_in.ports().iter().enumerate() {
        if midi_in.port_name(&p).unwrap().eq(&name) {
            port_index = Some(i);
            break;
        }
    }
    port_index
}

// Midi stream receive and parse 
pub fn receive(name: String) {
    let mut input = String::new();
    let midi_in = MidiInput::new("midi_in").expect("Could not open midi input.");
    let input_ports = midi_in.ports();

    // Getting input device port
    let device_port: Option<&MidiInputPort> = match get_port_index_by_name(&midi_in, name) {
        Some(i) => input_ports.get(i),
        None => None,
    };

    // Opening connection with input midi device
    let _conn_in = midi_in.connect(device_port.expect("Couldn't get device from name."), "midi_conn", 
        move |stamp, message, _| {
            let raw_message = message::Raw::new(stamp, message[0], message[1..].to_vec());
            let parsed: message::Midi = raw_message.parse();
            println!("{:?}", parsed);
        }, ()).unwrap();

    println!("Press any key to terminate.");
    input.clear();
    stdin().read_line(&mut input).unwrap();
}
