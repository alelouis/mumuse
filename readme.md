# midi-parse

Small midi parser written in Rust for debug purposes.

```rust 
fn main() {
    midi::show_input_ports();
    midi::receive("Arturia KeyStep 32".to_string());
}
```

```bash
Scarlett 18i20 USB
Arturia KeyStep 32
Press any key to terminate.
Midi { channel: 0, stamp: 154875800452, status: NoteOn, data: [KeyNumber(69), Velocity(31)] }
Midi { channel: 0, stamp: 154875856286, status: NoteOn, data: [KeyNumber(64), Velocity(25)] }
Midi { channel: 0, stamp: 154875899872, status: NoteOn, data: [KeyNumber(60), Velocity(65)] }
Midi { channel: 0, stamp: 154875930942, status: NoteOff, data: [KeyNumber(69), Velocity(64)] }
Midi { channel: 0, stamp: 154875959193, status: NoteOff, data: [KeyNumber(64), Velocity(64)] }
Midi { channel: 0, stamp: 154876072006, status: NoteOff, data: [KeyNumber(60), Velocity(64)] }
```