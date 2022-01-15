extern crate mumuse;
use mumuse::music::chord::Chord;
use mumuse::music::note::Note;

fn main() {
    // From an str Vec directly
    let chord_from_str = Chord::from(vec!["C0", "E1", "G2"]);
    println!("From str vector : {}", chord_from_str);

    // Same chord, but from a Vector Note
    let notes = vec!["C0", "E1", "G2"]
        .iter()
        .filter_map(|x| Note::try_from(*x).ok())
        .collect();
    let chord_from_notes = Chord::new(notes);
    println!("From Note vector : {}", chord_from_notes)
}
