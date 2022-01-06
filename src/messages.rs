use crate::conversions::encode_hex;

// module containing Raw and Midi structs
pub mod message {
    use super::Status;
    use super::Data;

    // Raw message contains bytes values
    #[derive(Debug)]
    pub struct Raw {
        pub stamp: u64,
        pub status: u8,
        pub data: Vec<u8>
    }
    
    // Midi message contains custom type events
    #[derive(Debug)]
    pub struct Midi {
        pub channel: u8,
        pub stamp: u64,
        pub status: Status,
        pub data: [Data; 2]
    }
}

// Midi status, first byte
#[derive(Debug)]
pub enum Status {
    NoteOff,
    NoteOn,
    PolyphonicKeyPressure,
    ControlChange,
    ProgramChange,
    ChannelPressure,
    PitchBend,
    Unknown,
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
    None
}


impl message::Raw {

    // Constructor for Raw message
    pub fn new(stamp: u64, status: u8, data: Vec<u8>) -> message::Raw {
        message::Raw {stamp, status, data}
    }

    // Parse Raw message into Midi message
    pub fn parse(&self) -> message::Midi {
         
        let status_hex = &encode_hex(&[self.status])[..];
        match &status_hex[0..1] {
            "8" => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status : Status::NoteOff, 
                data : [Data::KeyNumber(self.data[0]), Data::Velocity(self.data[1])]
            },
            "9" => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status : Status::NoteOn,  
                data : [Data::KeyNumber(self.data[0]), Data::Velocity(self.data[1])]
            },
            "a" => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status : Status::PolyphonicKeyPressure,  
                data : [Data::KeyNumber(self.data[0]), Data::PressureAmount(self.data[1])]
            },
            "b" => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status : Status::ControlChange,  
                data : [Data::ControllerNumber(self.data[0]), Data::ControllerValue(self.data[1])]
            },
            "c" => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status : Status::ProgramChange,  
                data : [Data::ProgramNumber(self.data[0]), Data::None]
            },
            "d" => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status : Status::ChannelPressure,  
                data : [Data::PressureValue(self.data[0]), Data::None]
            },
            "e" => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status : Status::PitchBend,  
                data : [Data::MSB(self.data[0]), Data::LSB(self.data[1])]
            },
            _ => message::Midi {
                channel: u8::from_str_radix(&status_hex[1..], 16).unwrap(),
                stamp: self.stamp,
                status : Status::Unknown,  
                data : [Data::None, Data::None]
            },
        }
    }
}