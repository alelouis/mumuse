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
    Tritone,
    Fifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    Octave,
}