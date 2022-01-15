//! Raw midi parsing and display

use crate::conversions::encode_hex;
use crate::music::common::KEYBOARD;
use crate::music::note::Note;
use colored::Colorize;
use std::fmt;

/// Raw message contains bytes values
#[derive(Debug)]
pub struct Raw {
    stamp: u64,
    status: u8,
    data: Vec<u8>,
}

impl Raw {
    // Constructor for Raw message
    pub fn new(stamp: u64, status: u8, data: Vec<u8>) -> Raw {
        Raw {
            stamp,
            status,
            data,
        }
    }

    // Parse Raw message into Midi message
    pub fn parse(&self) -> Midi {
        let status_hex = &encode_hex(&[self.status])[..];
        match &status_hex[0..1] {
            "8" => Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status: Status::NoteOff,
                data: [Data::KeyNumber(self.data[0]), Data::Velocity(self.data[1])],
            },
            "9" => Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status: Status::NoteOn,
                data: [Data::KeyNumber(self.data[0]), Data::Velocity(self.data[1])],
            },
            "a" => Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status: Status::PolyphonicKeyPressure,
                data: [
                    Data::KeyNumber(self.data[0]),
                    Data::PressureAmount(self.data[1]),
                ],
            },
            "b" => match &encode_hex(&[self.data[1]])[..] {
                "79" => Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::ResetAllControllers, Data::None],
                },
                "7a" => Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::LocalControl(self.data[2]), Data::None],
                },
                "7b" => Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::AllNotesOff, Data::None],
                },
                "7c" => Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::OmniModeOff, Data::None],
                },
                "7d" => Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::OmniModeOn, Data::None],
                },
                "7e" => Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::MonoModeOn, Data::None],
                },
                "7f" => Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::PolyModeOn, Data::None],
                },
                _ => Midi {
                    channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [
                        Data::ControllerNumber(self.data[0]),
                        Data::ControllerValue(self.data[1]),
                    ],
                },
            },
            "c" => Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status: Status::ProgramChange,
                data: [Data::ProgramNumber(self.data[0]), Data::None],
            },
            "d" => Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status: Status::ChannelPressure,
                data: [Data::PressureValue(self.data[0]), Data::None],
            },
            "e" => Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status: Status::PitchBend,
                data: [Data::MSB(self.data[0]), Data::LSB(self.data[1])],
            },
            "f" => match &status_hex[1..] {
                "1" => Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::MidiTimingCode,
                    data: [Data::Generic(self.data[1]), Data::None],
                },
                "2" => Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::SongPositionPointer,
                    data: [Data::Generic(self.data[1]), Data::Generic(self.data[2])],
                },
                "3" => Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::SongSelect,
                    data: [Data::Generic(self.data[1]), Data::None],
                },
                "6" => Midi {
                    stamp: self.stamp,
                    status: Status::TuneRequest,
                    ..Default::default()
                },
                "8" => Midi {
                    stamp: self.stamp,
                    status: Status::TimingClock,
                    ..Default::default()
                },
                "a" => Midi {
                    stamp: self.stamp,
                    status: Status::StartSequence,
                    ..Default::default()
                },
                "b" => Midi {
                    stamp: self.stamp,
                    status: Status::ContinueSequence,
                    ..Default::default()
                },
                "c" => Midi {
                    stamp: self.stamp,
                    status: Status::StopSequence,
                    ..Default::default()
                },
                "e" => Midi {
                    stamp: self.stamp,
                    status: Status::ActiveSensing,
                    ..Default::default()
                },
                "f" => Midi {
                    stamp: self.stamp,
                    status: Status::SystemReset,
                    ..Default::default()
                },
                _ => Midi {
                    stamp: self.stamp,
                    status: Status::Unknown,
                    ..Default::default()
                },
            },
            _ => Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status: Status::Unknown,
                data: [Data::None, Data::None],
            },
        }
    }
}

/// Midi message contains custom type events
#[derive(Debug)]
pub struct Midi {
    channel: u8,
    stamp: u64,
    status: Status,
    data: [Data; 2],
}

impl Default for Midi {
    fn default() -> Self {
        Self {
            channel: 16,
            stamp: 0,
            status: Status::Unknown,
            data: [Data::None, Data::None],
        }
    }
}

impl fmt::Display for Midi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stamp = format!("{:?}", self.stamp).green();
        let status = format!("{:?}", self.status).blue();
        let data = format!("{:?}", self.data).red();
        let channel = format!("{:?}", self.channel).yellow();
        let note = self.get_midi_note();
        match note {
            Some(note) => {
                let note = format!("{}", note).purple();
                write!(
                    f,
                    "Ti: {} | Ch: {:2} | St: {:15} | No : {} | Da: {}",
                    stamp, channel, status, note, data
                )
            }
            _ => write!(
                f,
                "Ti: {} | Ch: {:2} | St: {:15} | Da: {}",
                stamp, channel, status, data
            ),
        }
    }
}

impl Midi {
    // Get Note struct from Midi message
    pub fn get_midi_note(&self) -> Option<Note> {
        match &self.status {
            Status::NoteOn | Status::NoteOff => Some(Note::try_from(&self.data[0]).unwrap()),
            _ => None,
        }
    }
}

/// Status is determined by first byte of midi frame
#[derive(Debug)]
#[allow(dead_code)]
pub enum Status {
    NoteOff = 0x80,               // 8x
    NoteOn = 0x90,                // 9x
    PolyphonicKeyPressure = 0xA0, // Ax
    ControlChange = 0xB0,         // Bx
    ProgramChange = 0xC0,         // Cx
    ChannelPressure = 0xD0,       // Dx
    PitchBend = 0xE0,             // Ex
    MidiTimingCode = 0xF1,        // F1
    SongPositionPointer = 0xF2,   // F2
    SongSelect = 0xF3,            // F3
    TuneRequest = 0xF6,           // F6
    TimingClock = 0xF8,           // F8
    StartSequence = 0xFA,         // FA
    ContinueSequence = 0xFB,      // FB
    StopSequence = 0xFC,          // FC
    ActiveSensing = 0xFE,         // FE
    SystemReset = 0xFF,           // FF
    Unknown,                      // ??
}

/// Midi data, second and optional third bytes
#[derive(Debug)]
#[allow(dead_code)]
pub enum Data {
    KeyNumber(u8),
    Velocity(u8),
    ControllerNumber(u8),
    ControllerValue(u8),
    PressureAmount(u8),
    ProgramNumber(u8),
    PressureValue(u8),
    MSB(u8),
    LSB(u8),
    ResetAllControllers,
    LocalControl(u8),
    AllNotesOff,
    OmniModeOff,
    OmniModeOn,
    MonoModeOn,
    PolyModeOn,
    Generic(u8),
    None,
}

pub fn from_note(note: &Note) -> Data {
    let p = KEYBOARD.iter().position(|&n| n == note.letter).unwrap() as u8;
    Data::KeyNumber(12 + p + (note.octave as u8) * 12)
}
