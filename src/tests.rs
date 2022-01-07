use std::vec;

use crate::messages;
use crate::music::{Chord, Note, Letter};
use crate::music;

// Tests conversion from Midi to Note
#[test]
fn from_key_number_to_note() {
    for kn in 21..127 {
        let data_kn = messages::Data::KeyNumber(kn);
        let note = match Note::from_key_number(&data_kn) {
            Some(note) => note,
            None => panic!("Oups"),
        };
        assert_eq!(note.letter, music::KEYBOARD[((kn - 21) % 12) as usize]);
        assert_eq!(note.octave, (kn - 21) / 12);
    }
}

#[test]
fn note_from_str(){
    let a = Note::from_str("A0").unwrap();
    assert_eq!(a.letter, Letter::A);
    assert_eq!(a.octave, 0);

    let bb = Note::from_str("Bb2").unwrap();
    assert_eq!(bb.letter, Letter::Bb);
    assert_eq!(bb.octave, 2);
}

#[test]
fn distance_between_notes() {
    let note_1 = Note {
        letter: Letter::C,
        octave: 0,
    };
    let note_2 = Note {
        letter: Letter::E,
        octave: 0,
    };
    let note_3 = Note {
        letter: Letter::E,
        octave: 1,
    };
    let note_4 = Note {
        letter: Letter::B,
        octave: 0,
    };
    let note_5 = Note {
        letter: Letter::C,
        octave: 1,
    };
    assert_eq!(note_1.dist_to(&note_2), 4);
    assert_eq!(note_2.dist_to(&note_1), 4);
    assert_eq!(note_1.dist_to(&note_3), 16);
    assert_eq!(note_3.dist_to(&note_1), 16);
    assert_eq!(note_4.dist_to(&note_5), 1);
}

#[test]
fn chord_from_struct() {
    let note_1 = Note {
        letter: Letter::C,
        octave: 0,
    };
    let note_2 = Note {
        letter: Letter::E,
        octave: 0,
    };
    let note_3 = Note {
        letter: Letter::G,
        octave: 0,
    };
    let chord = Chord::new(vec![note_1, note_2, note_3]);
    assert_eq!(chord.notes.len(), 3);
}

#[test]
fn chord_from_str() {
    let chord = Chord::from_str(vec!["C0", "E1", "G2"]);
    assert_eq!(chord.notes[0].letter, Letter::C);
    assert_eq!(chord.notes[1].letter, Letter::E);
    assert_eq!(chord.notes[2].letter, Letter::G);
}

#[test]
fn chord_transition() {
    let from = Chord::from_str(vec!["C4", "E4", "G4"]);
    let target = Chord::from_str(vec!["G4", "C5", "Eb5"]);
    let mut dist_vec: Vec<Vec<Vec<u8>>> = vec![];

    for note in &from.notes {
        let mut note_vec: Vec<Vec<u8>> = vec![];
        for other_note in &target.notes {
            let mut octave_vec: Vec<u8> = vec![];
            for octave in other_note.octave-1..=other_note.octave+1 {
                let swipe_note = Note::new(other_note.letter, octave);
                octave_vec.push(note.dist_to(&swipe_note));
            }
            note_vec.push(octave_vec);
        }
        dist_vec.push(note_vec);
    }

    let first: Vec<usize> = (0..from.notes.len()).collect();
    let mut max: u32 = 100;
    let mut voice_lead: Option<Chord> = None;

    for f in &first {
        let mut second = first.clone();
        second.remove(second.iter().position(|x| *x == *f).unwrap());
        for s in &second {
            let mut third = second.clone();
            third.remove(third.iter().position(|x| *x == *s).unwrap());
            for t in &third {
                for i in 0..3 {
                    for j in 0..3 {
                        for k in 0..3 {
                            let sum: u32 = (dist_vec[0][*f][i] + dist_vec[1][*s][j] + dist_vec[2][*t][k]).try_into().unwrap();
                            if sum < max {
                                max = sum;
                                let note1 = Note::new(target.notes[*f].letter, target.notes[*f].octave + i as u8 - 1);
                                let note2 = Note::new(target.notes[*s].letter, target.notes[*s].octave + j as u8 - 1);
                                let note3 = Note::new(target.notes[*t].letter, target.notes[*t].octave + k as u8 - 1);
                                voice_lead = Some(Chord::new(vec![note1, note2, note3]));
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", dist_vec);
    println!("{:?}", max);
    println!("{:?}", voice_lead);
}