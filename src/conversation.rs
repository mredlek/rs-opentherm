use ::message::{AsDataId, AsDataValue, MsgType, AsMsgType};
use ::{Error, ErrorKind};
use super::application::{ComplexType};
use super::application::SimpleTypeEnum;

/// The payload of a data field, or Null of no data was present.
#[derive(Clone, Debug, Serialize)]
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



/// Read or Write
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Method
{
    /// Read
    Read,
    /// Write
    Write,
}

/// A complete and succesfull conversation (request and response)
#[derive(Clone, Debug, Serialize)]
pub enum Conversation
{
    /// Succesfull read
    Read(NullableComplexType),
    /// Succesfull write
    Write(NullableComplexType)
}

impl AsDataId for Conversation
{
    fn as_data_id(&self) -> u8 {
        match self {
            &Conversation::Read(ref data) => data.as_data_id(),
            &Conversation::Write(ref data) => data.as_data_id()
        }
    }
}

impl Conversation
{
    /// Returns the data value of the conversation.
    /// It the data is send as Null, it will return None
    pub fn as_data_value(&self) -> Option<[u8; 2]> {
        let nullabledata = match self {
            &Conversation::Read(ref data) => data,
            &Conversation::Write(ref data) => data,
        };
        match nullabledata {
            &NullableComplexType::Null { .. } => None,
            &NullableComplexType::Data(complex) => Some(complex.as_data_value()),
        }
    }
}

/*impl Deref for Conversation
{
    type Target = NullableComplexType;

    fn deref(&self) -> &NullableComplexType
    {
        match self {
            &Conversation::Read(ref data) => data,
            &Conversation::Write(ref data) => data
        }
    }
}*/

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
    pub fn new<Message: AsDataId + AsDataValue + AsMsgType>(request: &Message, response: &Message) -> Result<Conversation, Error>
    {
        if request.as_data_id() != response.as_data_id() {
            return Err(ErrorKind::InvalidData.into()); // Invalid response for request
        }

        match request.as_msg_type() {
            MsgType::ReadData => {
                match response.as_msg_type() {
                    MsgType::ReadAck => Ok(Conversation::Read(NullableComplexType::Data(
                        try!(ComplexType::new_from_message(response))))),
                    MsgType::DataInvalid => Ok(Conversation::Read( NullableComplexType::Null { dataid: response.as_data_id()})),
                    MsgType::UnknownDataId => Err(ErrorKind::UnknownDataId(response.as_data_id()).into()),
                    _ => Err(ErrorKind::InvalidData.into()) // Invalid response for request
                }
            },
            MsgType::WriteData => {
                match response.as_msg_type() {
                    MsgType::WriteAck=> Ok(Conversation::Write(NullableComplexType::Data(
                        try!(ComplexType::new_from_message(request))))),
                    MsgType::DataInvalid => Err(ErrorKind::InvalidData.into()),
                    MsgType::UnknownDataId => Err(ErrorKind::UnknownDataId(request.as_data_id()).into()),
                    _ => Err(ErrorKind::InvalidData.into()) // Invalid response for request
                }
            },
            MsgType::InvalidData => {
                match response.as_msg_type() {
                    MsgType::DataInvalid => Ok(Conversation::Write(NullableComplexType::Null { dataid: response.as_data_id() })),
                    MsgType::UnknownDataId => Err(ErrorKind::UnknownDataId(response.as_data_id()).into()),
                    _ => Err(ErrorKind::InvalidData.into()) // Invalid response for request
                }
            },
            _ => Err(ErrorKind::InvalidData.into()) // Invalid response for request
        }
    }

    /// Returns if this conversation is an attempt to read or to write data
    pub fn method(&self) -> Method
    {
        match self {
            &Conversation::Read(_) => Method::Read,
            &Conversation::Write(_) => Method::Write,
        }
    }

    /// Get the complex type
    pub fn complex_type(&self) -> Option<&ComplexType>
    {
        match self {
            &Conversation::Read(NullableComplexType::Data(ref complex)) |
            &Conversation::Write(NullableComplexType::Data(ref complex)) => Some(complex),
            _ => None
        }
    }

    /// Get the simple type representation of the application data
    pub fn simple_type(&self) -> Option<SimpleTypeEnum>
    {
        match self {
            &Conversation::Read(NullableComplexType::Data(ref complex)) |
            &Conversation::Write(NullableComplexType::Data(ref complex)) => {
                Some(complex.clone().into())
            },
            _ => None
        }
    }
}

impl AsDataId for NullableComplexType
{
    fn as_data_id(&self) -> u8
    {
        match self {
            &NullableComplexType::Null{ dataid } => dataid,
            &NullableComplexType::Data(ref complextype) => complextype.as_data_id()
        }
    }
}