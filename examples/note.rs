extern crate mumuse;

use mumuse::music::note::Note;
use mumuse::music::common::Letter;

fn main() {
    // Declare Note from &str
    let _a = Note::from_str("A0").unwrap(); // can fail
    let _b = Note::new(Letter::A, 0);
}