use mumuse::music::chord::Chord;
use mumuse::music::common::Interval;
use mumuse::music::note::Note;

fn main() {
    // From an str Vec directly
    let chord_from_str = Chord::from(vec!["C4", "E4", "G4"]);
    println!("From str vector : {}", chord_from_str);

    // Same chord, but from a Vector Note
    let notes = vec!["C4", "E4", "G4"]
        .iter()
        .filter_map(|x| Note::try_from(*x).ok())
        .collect();
    let chord_from_notes = Chord::new(notes);
    println!("From Note vector : {}", chord_from_notes);


    // Chord from Note
    let note = Note::try_from("C3").unwrap();
    let chord = note.chord("major");
    println!("Note to Chord : {chord}");

    // Inversion
    let first_inv = chord_from_str.invert(1);
    let second_inv = chord_from_str.invert(2);
    println!("First inversion : {}", first_inv);
    println!("First inversion : {}", second_inv);

    // Transposition
    let chord_one_fifth = chord_from_str.clone() + Interval::Fifth; // Chord is not Copy
    let chord_one_octave_down = chord_from_str.clone() - Interval::Octave;
    println!("Fifth up : {}", chord_one_fifth);
    println!("Octave down : {}", chord_one_octave_down);
}
