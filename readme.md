# mumuse

![build](https://github.com/alelouis/mumuse/actions/workflows/rust.yml/badge.svg)
![license](https://img.shields.io/github/last-commit/alelouis/mumuse)
[![doc.rs](https://img.shields.io/badge/doc.rs-mumuse-red)](https://docs.rs/mumuse/latest/mumuse/)
[![crates.io](https://img.shields.io/badge/crates.io-mumuse-red)](https://crates.io/crates/mumuse)


Small music theory library with MIDI capabilities written in Rust *(wip)*.  
Scroll documentation : [**doc.rs/mumuse/latest**](https://docs.rs/mumuse/latest/mumuse/)  
(or `cargo doc` for unreleased state).
```rust 
fn main() {
    midi::show_input_ports();
    midi::receive("Arturia KeyStep 32".to_string());
}
```

<p align="center">
  <img width="1000" src="capture.png">
</p>

```rust 
extern crate mumuse;

use mumuse::midi::{self, MidiSend};
use mumuse::music::chord::Chord;
use mumuse::music::note::Note;
use mumuse::music::scale::Scale;

// Secondary dominants function
fn get_five(scale: &Scale, degree: usize) -> Chord {
    Scale::major(scale.notes()[degree-1]).five(4)
}

fn main() {
    // Declare a root note
    let root = Note::try_from("C3").unwrap();
    let major_scale = Scale::major(root);

    // Fill with chords
    let chords = (1..6).map(|degree| major_scale.by_degree(degree, 3).invert(2));
    let secondary_dominants = (1..6).map(|degree| get_five(&major_scale, degree+1));

    // Play them through midi
    // midi::show_output_ports(); // show output ports
    let mut conn_out = midi::get_output_connection("Virtual Midi Bus 1".to_string());
    for (chord, second) in chords.zip(secondary_dominants) {
        chord.send_midi(&mut conn_out, 500, 64);
        second.send_midi(&mut conn_out, 500, 64);
    }
}
```
