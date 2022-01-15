//! Midi and music library
extern crate num;
#[macro_use]
extern crate num_derive;

pub mod conversions;
pub mod messages;
pub mod midi;
pub mod music;

#[cfg(test)]
pub mod tests;
