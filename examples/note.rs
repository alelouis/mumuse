extern crate mumuse;

use mumuse::music::common::Letter;
use mumuse::music::note::Note;

fn main() {
    // Declare Note from &str
    let _a = Note::try_from("A0").unwrap(); // can fail
    let _b = Note::new(Letter::A, 0);
}
