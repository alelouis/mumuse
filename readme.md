# mumuse

Small midi parser and music theory library written in Rust *(wip)*.  
Scroll documentation : [**doc.rs/mumuse**](https://docs.rs/mumuse/0.1.0/mumuse/)  
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

use mumuse::music::note::Note;
use mumuse::music::common::Interval;
use mumuse::music::scale::Scale;

fn main() {
    // Declare a root note
    let root = Note::from_str("C0").unwrap();

    // Declare vector of intervals
    let intervals = vec![Interval::MajorSecond, Interval::Fifth];

    // Declare scale from root and intervals
    let scale = Scale::new(root, intervals);

    // Get notes of scale
    let notes = scale.notes();
    println!("{:?}", notes);
}
```