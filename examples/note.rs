use mumuse::music::common::{Interval, Letter};
use mumuse::music::note::Note;

fn main() {
    // Declare Note from &str
    let _a = Note::try_from("A0").unwrap(); // can fail

    // Declare from struct
    let n = Note::new(Letter::A, 2);
    println!("Note : {}", n);

    // Transpose up by one Fifth
    let fifth = n + Interval::Fifth;
    println!("Fifth up : {}", fifth);

    // Transpose down by one Octave
    let octave = n - Interval::Octave;
    println!("Octave down : {}", octave);
    
}
