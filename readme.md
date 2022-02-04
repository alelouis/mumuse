# **mumuse**

![build](https://github.com/alelouis/mumuse/actions/workflows/rust.yml/badge.svg)
![license](https://img.shields.io/github/last-commit/alelouis/mumuse)
[![doc.rs](https://img.shields.io/badge/doc.rs-mumuse-red)](https://docs.rs/mumuse/latest/mumuse/)
[![crates.io](https://img.shields.io/badge/crates.io-mumuse-red)](https://crates.io/crates/mumuse)
![rustc](https://img.shields.io/badge/rustc-%3E%201.58.0-important)

 A Rust small music theory library, featuring:
  - Elementary operations with notes and chords
  - Building chords from scales degrees 
  - Scale modes
  - Sequence building with streams
  - Midi real time playing of streams

This is still in active development, things **will** brake.


```rust
  let mut stream: Stream = Stream::new();
  let mut conn_out = midi::get_output_connection("Virtual Midi Bus 1".to_string());
  
  // Constructing event stream
  let notes = ["A3", "B3", "C4", "D4", "A3", "B3", "C4", "D4"];
  let mut time = Time::new(1, 4, 1);
  let duration = Duration::new(16, 1); // 16th notes

  for n in notes {
      let note = Note::try_from(n).unwrap();
      stream.add_note(note, time, duration);
      time = time + duration;
  }

  // Real time play of events
  stream.play(&mut conn_out, 120.0, 4); // midi_connection, beat per minute, beats per bar
```

## **How to use**
Add mumuse lib crate to your `Cargo.toml` file.
```toml
mumuse = "version"
``` 

Browse the `examples/` folder in order to see capabilities of the library.  
Also, you can check the [documentation](https://docs.rs/mumuse/latest/mumuse/).
