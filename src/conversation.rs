use ::message::DataId;
use ::message::{Message,MsgType};
use ::{Error, ErrorKind};
use super::application::{ComplexType, OpenthermMessage};

use ::std::ops::Deref;

/// The payload of a data field, or Null of no data was present.
#[derive(Clone, Debug)]
pub enum NullableComplexType
{
    /// No data was present
    Null{
        /// The dataid of this complex type
        dataid: u8
    },
    /// Data found and returned
    Data(ComplexType)
}

/// A complete and succesfull conversation (request and response)
#[derive(Clone, Debug)]
pub enum Conversation
{
    /// Succesfull read
    Read(NullableComplexType),
    /// Succesfull write
    Write(NullableComplexType)
}

impl DataId for Conversation
{
    fn data_id(&self) -> u8 {
        match self {
            &Conversation::Read(ref data) => data.data_id(),
            &Conversation::Write(ref data) => data.data_id()
        }
    }
}

impl Deref for Conversation
{
    type Target = NullableComplexType;

    fn deref(&self) -> &NullableComplexType
    {
        match self {
            &Conversation::Read(ref data) => data,
            &Conversation::Write(ref data) => data
        }
    }
}

impl Into<Option<ComplexType>> for NullableComplexType
{
    fn into(self) -> Option<ComplexType> {
        match self {
            NullableComplexType::Null { .. } => None,
            NullableComplexType::Data(data) => Some(data)
        }
    }
}

impl Conversation
{
    /// Creates a new conversation
    pub fn new(request: &(Message + 'static), response: &(Message+ 'static)) -> Result<Conversation, Error>
    {
        if request.data_id() != response.data_id() {
            return Err(ErrorKind::InvalidData.into()); // Invalid response for request
        }

        match request.msg_type() {
            MsgType::ReadData => {
                match response.msg_type() {
                    MsgType::ReadAck => Ok(Conversation::Write(NullableComplexType::Data(
                        try!(response.data_value_complex())))),
                    MsgType::DataInvalid => Ok(Conversation::Read( NullableComplexType::Null { dataid: response.data_id()})),
                    MsgType::UnknownDataId => Err(ErrorKind::UnknownDataId(response.data_id()).into()),
                    _ => Err(ErrorKind::InvalidData.into()) // Invalid response for request
                }
            },
            MsgType::WriteData => {
                match response.msg_type() {
                    MsgType::WriteAck=> Ok(Conversation::Write(NullableComplexType::Data(
                        try!(request.data_value_complex())))),
                    MsgType::DataInvalid => Err(ErrorKind::InvalidData.into()),
                    MsgType::UnknownDataId => Err(ErrorKind::UnknownDataId(request.data_id()).into()),
                    _ => Err(ErrorKind::InvalidData.into()) // Invalid response for request
                }
            },
            MsgType::InvalidData => {
                match response.msg_type() {
                    MsgType::DataInvalid => Ok(Conversation::Write(NullableComplexType::Null { dataid: response.data_id() })),
                    MsgType::UnknownDataId => Err(ErrorKind::UnknownDataId(response.data_id()).into()),
                    _ => Err(ErrorKind::InvalidData.into()) // Invalid response for request
                }
            },
            _ => Err(ErrorKind::InvalidData.into()) // Invalid response for request
        }
    }
}

impl DataId for NullableComplexType
{
    fn data_id(&self) -> u8
    {
        match self {
            &NullableComplexType::Null{ dataid } => dataid,
            &NullableComplexType::Data(ref complextype) => complextype.data_id()
        }
    }
}