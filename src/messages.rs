use crate::conversions::encode_hex;

// module containing Raw and Midi structs
pub mod message {
    use super::Data;
    use super::Status;

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
}

// System Common Message   Status Byte      Number of Data Bytes
// ---------------------   -----------      --------------------
// MIDI Timing Code            F1                   1
// Song Position Pointer       F2                   2
// Song Select                 F3                   1
// Tune Request                F6                  None

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
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::TuneRequest,
                    data: [Data::None, Data::None],
                },
                "8" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::TimingClock,
                    data: [Data::None, Data::None],
                },
                "a" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::StartSequence,
                    data: [Data::None, Data::None],
                },
                "b" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ContinueSequence,
                    data: [Data::None, Data::None],
                },
                "c" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::StopSequence,
                    data: [Data::None, Data::None],
                },
                "e" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ActiveSensing,
                    data: [Data::None, Data::None],
                },
                "f" => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::SystemReset,
                    data: [Data::None, Data::None],
                },
                _ => message::Midi {
                    channel: 16,
                    stamp: self.stamp,
                    status: Status::ControlChange,
                    data: [Data::ResetAllControllers, Data::None],
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
