use mumuse::music::note::Note;
use mumuse::music::scale::Scale;

fn main() {
    // Declare a root note
    let root = Note::try_from("C0").unwrap();

    // Declare major scale
    let major_scale = Scale::major(root);
    println!("{}", major_scale);

    // Construct modes
    let ionian = major_scale.mode(1); // Does not change anything on Major scale
    let dorian = major_scale.mode(2);
    let phrygian = major_scale.mode(3);

    // Print scales
    println!("{ionian}");
    println!("{dorian}");
    println!("{phrygian}");
}
