use ::std::error::Error as StdError;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result as FmtResult;

/// List of errors that can occur while processing OpenTherm messages
pub enum Error
{
    /// The dataid isn't supported
    UnknownDataId(u8),
    /// A dataid is used for writing which is readonly or reading data which is writeonly.
    InvalidAccessMethod(u8),
    /// Use of a reserved and thus current unsupported msgtype
    InvalidMsgType,
    /// Indicates that there is a problem with the payload of the message
    InvalidData,
    /// The payload didn't parse correct, probably because some values are out of range
    InvalidApplicationData,
    /// A response is received for unknown request
    MissingRequest,
    /// A request was send but no answer was received in tme
    ResponseTimeout,
    /// The parity of the message was odd instead of even.
    IncorrectParity
}

impl Error
{
    /// Creates a missing_request error. This is an error which is described in the OpenTherm
    /// specification but can't occur during parsing because it is caused by data not available.
    pub fn missing_request() -> Error
    {
        Error::MissingRequest
    }
    /// Creates a request_timeout error. This is an error which is described in the OpenTherm
    /// specification but can't occur during parsing because it is caused by data not available.
    pub fn response_timeout() -> Error
    {
        Error::ResponseTimeout
    }
}

impl StdError for Error
{
    fn description(&self) -> &str {
        match self {
            &Error::UnknownDataId(_) => "Unknown data id",
            &Error::InvalidAccessMethod(_) => "Invalid access method",
            &Error::InvalidMsgType => "Invalid msg type",
            &Error::InvalidData => "Invalid data",
            &Error::InvalidApplicationData => "Invalid application data",
            &Error::MissingRequest => "Missing request",
            &Error::ResponseTimeout => "Response timeout",
            &Error::IncorrectParity => "Incorrect parity"
        }
    }
}

impl Display for Error
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &Error::UnknownDataId(dataid) => write!(f, "Unknown data id {}", dataid),
            &Error::InvalidAccessMethod(dataid) => write!(f, "Invalid access method for data id {}", dataid),
            &Error::InvalidData => write!(f, "Invalid data"),
            &Error::InvalidMsgType => write!(f, "Invalid msg type"),
            &Error::InvalidApplicationData=> write!(f, "Unknown application data"),
            &Error::MissingRequest => write!(f, "Missing request"),
            &Error::ResponseTimeout => write!(f, "Response timeout"),
            &Error::IncorrectParity => write!(f, "Incorrect parity")
        }
    }
}

impl Debug for Error
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &Error::UnknownDataId(dataid) => write!(f, "Unknown data id {}", dataid),
            &Error::InvalidAccessMethod(dataid) => write!(f, "Invalid access method for data id {}", dataid),
            &Error::InvalidMsgType => write!(f, "Invalid msg type"),
            &Error::InvalidData => write!(f, "Invalid data"),
            &Error::InvalidApplicationData => write!(f, "Invalid application data"),
            &Error::MissingRequest => write!(f, "Missing request"),
            &Error::ResponseTimeout => write!(f, "Response timeout"),
            &Error::IncorrectParity => write!(f, "Incorrect parity")
        }
    }
}
