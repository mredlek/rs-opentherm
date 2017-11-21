use ::{Error, ErrorKind};

use ::std::fmt::Result as FmtResult;
use ::std::fmt::{Display,Formatter,Binary};

/// Opentherm message
pub trait Message
{
    /// Get a slice tot the message
    fn data(&self) -> &[u8];
}

/// MsgType of the opentherm message
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum MsgType
{
    /// Message to initiate reading
    ReadData,
    /// Message to initiate writing
    WriteData,
    /// Message to reset a value
    InvalidData,
    /// Message response after succesfull read
    ReadAck,
    /// Message response after succesfull write
    WriteAck,
    /// Message response after invalid or null payload
    DataInvalid,
    /// Message response to indicate that the dataid wasn't recognized
    UnknownDataId
}

/// Trait for all types where an opentherm msgtype can be extracted
pub trait AsMsgType
{
    /// The msg_type of self
    fn as_msg_type(&self) -> MsgType;
}

/// Trait for all types where an opentherm dataid can be extracted
pub trait AsDataId
{
    /// Function to return the data id of an opentherm message
    fn as_data_id(&self) -> u8;
}

/// Trait for all types where an opentherm datavalue can be extracted
pub trait AsDataValue
{
    /// Function to return the data value
    fn as_data_value(&self) -> [u8; 2];
}

impl Message
{
    /// Creates a new message from four input bytes.
    /// The payload of msg is described in the OpenTherm documentation.
    pub fn check(&self) -> Result<(), Error>
    {
        let msg = self.data();
        if msg[0] & 0x70 == 0b00110000 {
            Err(ErrorKind::InvalidMsgType.into())
        } else if parity_vec(&msg) != 0u8 {
            Err(ErrorKind::IncorrectParity.into())
        } else {
            Ok(())
        }
    }

    /*
    /// Returns the payload of the message. The interprestation of the payload depends on the dataid.
    pub(crate) fn data_value<'this>(&'this self) -> DataValueImpl<'this>
    {
        DataValueImpl::from(&self.data()[2..4])
    }*/
}

impl<T: Message> AsDataId for T
{
    fn as_data_id(&self) -> u8 {
        self.data()[1]
    }
}

impl<T: Message> AsDataValue for T
{
    fn as_data_value(&self) -> [u8; 2]
    {
        [ self.data()[2], self.data()[3] ]
    }
}

impl<T: Message> AsMsgType for T
{
    fn as_msg_type(&self) -> MsgType {
        match self.data()[0] & 0x70 {
            0b00000000 => MsgType::ReadData,
            0b00010000 => MsgType::WriteData,
            0b00100000 => MsgType::InvalidData,
            0b01000000 => MsgType::ReadAck,
            0b01010000 => MsgType::WriteAck,
            0b01100000 => MsgType::DataInvalid,
            0b01110000 => MsgType::UnknownDataId,
            _ => unreachable!()
        }
    }
}

/*impl DataId for Message
{
    fn data_id(&self) -> u8
    {
        self.data()[1]
    }
}

impl DataValue for Message
{
    fn data_value(&self) -> [u8; 2] { [ self.data()[2], self.data()[3] ] }
}*/

impl Display for MsgType
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &MsgType::ReadData => { try!(f.write_str("Read data")); },
            &MsgType::WriteData => { try!(f.write_str("Write data")); },
            &MsgType::InvalidData => { try!(f.write_str("Invalid data")); },
            &MsgType::ReadAck => { try!(f.write_str("Read ack")); },
            &MsgType::WriteAck => { try!(f.write_str("Write ack")); },
            &MsgType::DataInvalid => { try!(f.write_str("Data invalid")); },
            &MsgType::UnknownDataId => { try!(f.write_str("Unknown data id")); },
        }
        Ok(())
    }
}

impl Binary for MsgType
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &MsgType::ReadData => { try!(f.write_str("000")); },
            &MsgType::WriteData => { try!(f.write_str("001")); },
            &MsgType::InvalidData => { try!(f.write_str("010")); },
            &MsgType::ReadAck => { try!(f.write_str("100")); },
            &MsgType::WriteAck => { try!(f.write_str("101")); },
            &MsgType::DataInvalid => { try!(f.write_str("110")); },
            &MsgType::UnknownDataId => { try!(f.write_str("111")); },
        }
        Ok(())
    }
}

fn parity_vec(input: &[u8]) -> u8
{
    let mut res = 0u8;
    for inputbyte in input {
        res ^= parity(*inputbyte)
    }
    res
}

fn parity(input: u8) -> u8
{
    let mut res : u8 = (input >> 4) ^ input;
    res = (res >> 2) ^ res;
    res = (res >> 1) ^ res;
    res & 0x01
}