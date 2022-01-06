use crate::conversions::encode_hex;

// module containing Raw and Midi structs
pub mod message {
    use super::Data;
    use super::Status;
    use std::fmt;
    use colored::Colorize;
    use crate::music::*;

    // Raw message contains bytes values
    #[derive(Debug)]
    pub struct Raw {
        pub stamp: u64,
        pub status: u8,
        pub data: Vec<u8>,
    }

    // Midi message contains custom type events
    #[derive(Debug)]
    pub struct Midi {
        pub channel: u8,
        pub stamp: u64,
        pub status: Status,
        pub data: [Data; 2],
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
                    write!(f, "Ti: {} | Ch: {:2} | St: {:15} | No : {} | Da: {}", stamp, channel, status, note, data)
                },
                _ => write!(f, "Ti: {} | Ch: {:2} | St: {:15} | Da: {}", stamp, channel, status, data)
            }
            
        }
    }

    impl Midi {

        // Get Note struct from Midi message
        pub fn get_midi_note(&self) -> Option<Note> {
            match &self.status {
                Status::NoteOn | Status::NoteOff => Note::from_key_number(&self.data[0]),
                _ => None,
            }
        } 
    }
}

// Status is determined by first byte of midi frame
#[derive(Debug)]
pub enum Status {
    NoteOff,               // 8x
    NoteOn,                // 9x
    PolyphonicKeyPressure, // Ax
    ControlChange,         // Bx
    ProgramChange,         // Cx
    ChannelPressure,       // Dx
    PitchBend,             // Ex
    MidiTimingCode,        // F1
    SongPositionPointer,   // F2
    SongSelect,            // F3
    TuneRequest,           // F6
    TimingClock,           // F8
    StartSequence,         // FA
    ContinueSequence,      // FB
    StopSequence,          // FC
    ActiveSensing,         // FE
    SystemReset,           // FF
    Unknown,               // ??
}

// Midi data, second and optional third bytes
#[derive(Debug)]
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

impl message::Raw {
    
    // Constructor for Raw message
    pub fn new(stamp: u64, status: u8, data: Vec<u8>) -> message::Raw {
        message::Raw {
            stamp,
            status,
            data,
        }
    }

    // Parse Raw message into Midi message
    pub fn parse(&self) -> message::Midi {
        let status_hex = &encode_hex(&[self.status])[..];
        match &status_hex[0..1] {
            "8" => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status: Status::NoteOff,
                data: [Data::KeyNumber(self.data[0]), Data::Velocity(self.data[1])],
            },
            "9" => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status: Status::NoteOn,
                data: [Data::KeyNumber(self.data[0]), Data::Velocity(self.data[1])],
            },
            "a" => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status: Status::PolyphonicKeyPressure,
                data: [
                    Data::KeyNumber(self.data[0]),
                    Data::PressureAmount(self.data[1]),
                ],
            },
            "b" => match &encode_hex(&[self.data[1]])[..] {
                "79" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::ResetAllControllers, Data::None],
                },
                "7a" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::LocalControl(self.data[2]), Data::None],
                },
                "7b" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::AllNotesOff, Data::None],
                },
                "7c" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::OmniModeOff, Data::None],
                },
                "7d" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::OmniModeOn, Data::None],
                },
                "7e" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::MonoModeOn, Data::None],
                },
                "7f" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::PolyModeOn, Data::None],
                },
                _ => message::Midi {
                    channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [
                        Data::ControllerNumber(self.data[0]),
                        Data::ControllerValue(self.data[1]),
                    ],
                },
            },
            "c" => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status: Status::ProgramChange,
                data: [Data::ProgramNumber(self.data[0]), Data::None],
            },
            "d" => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status: Status::ChannelPressure,
                data: [Data::PressureValue(self.data[0]), Data::None],
            },
            "e" => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status: Status::PitchBend,
                data: [Data::MSB(self.data[0]), Data::LSB(self.data[1])],
            },
            "f" => match &status_hex[1..] {
                "1" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::MidiTimingCode,
                    data: [Data::Generic(self.data[1]), Data::None],
                },
                "2" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::SongPositionPointer,
                    data: [Data::Generic(self.data[1]), Data::Generic(self.data[2])],
                },
                "3" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::SongSelect,
                    data: [Data::Generic(self.data[1]), Data::None],
                },
                "6" => message::Midi {
                    stamp: self.stamp,
                    status: Status::TuneRequest,
                    ..Default::default()
                },
                "8" => message::Midi {
                    stamp: self.stamp,
                    status: Status::TimingClock,
                    ..Default::default()
                },
                "a" => message::Midi {
                    stamp: self.stamp,
                    status: Status::StartSequence,
                    ..Default::default()
                },
                "b" => message::Midi {
                    stamp: self.stamp,
                    status: Status::ContinueSequence,
                    ..Default::default()
                },
                "c" => message::Midi {
                    stamp: self.stamp,
                    status: Status::StopSequence,
                    ..Default::default()
                },
                "e" => message::Midi {
                    stamp: self.stamp,
                    status: Status::ActiveSensing,
                    ..Default::default()
                },
                "f" => message::Midi {
                    stamp: self.stamp,
                    status: Status::SystemReset,
                    ..Default::default()
                },
                _ => message::Midi {
                    stamp: self.stamp,
                    status: Status::Unknown,
                    ..Default::default()
                },
            },
            _ => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status: Status::Unknown,
                data: [Data::None, Data::None],
            },
        }
    }
}
