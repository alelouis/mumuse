use mumuse::music::common::{Interval, Letter};
use mumuse::music::note::Note;

fn main() {
    // Declare Note from &str
    let _a = Note::try_from("A0").unwrap(); // can fail
    let _b = Note::new(Letter::A, 0);

    let mut n = Note::new(Letter::A, 0);
    n = n + Interval::Fifth;
    println!("{}", n);
}
