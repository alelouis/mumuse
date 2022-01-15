use mumuse::music::common::Interval;
use mumuse::music::note::Note;
use mumuse::music::scale::Scale;

fn main() {
    // Declare a root note
    let root = Note::try_from("C0").unwrap();

    // Declare vector of intervals
    let intervals = vec![Interval::MajorSecond, Interval::Fifth];

    // Declare scale from root and intervals and print it
    let scale = Scale::new(root, intervals);
    println!("{}", scale);

    // Declare existing scales
    let major_scale = Scale::major(root);
    println!("{}", major_scale);

    // Construct Chord from Scale degree
    let one_chord = Scale::major(root).one(3); // 3 notes chord
    let five_chord = Scale::major(root).five(4); // 4 notes chord
    println!("I chord of C Major Scale : {}", one_chord);
    println!("V7 chord of C Major Scale : {}", five_chord);
}
