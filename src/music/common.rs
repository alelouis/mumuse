//! Common music definitions

/// Twelve tone temperament Keyboard vector of Note
pub const KEYBOARD: [Letter; 12] = [
    Letter::C,
    Letter::Db,
    Letter::D,
    Letter::Eb,
    Letter::E,
    Letter::F,
    Letter::Gb,
    Letter::G,
    Letter::Ab,
    Letter::A,
    Letter::Bb,
    Letter::B,
];

/// Letters to describe notes
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Letter {
    A,
    Bb,
    B,
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
}

/// Letters to describe notes
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Interval {
    Unison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    Fourth,
    Tritone,
    Fifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    Octave,
}

impl Default for Letter {
    fn default() -> Self {
        Letter::C
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval::Unison
    }
}

/// Find index of Letter in Keyboard
pub fn find_letter_idx(letter: Letter) -> i8{
    KEYBOARD.iter().position(|&x| x == letter).unwrap() as i8
}