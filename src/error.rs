use ::std::error::Error as StdError;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::fmt::Result as FmtResult;

error_chain! {

    errors {
        /// Unknown dataid
        UnknownDataId(dataid: u8) {
            description("Unknown data id")
            display("Unknown data id {}", dataid)
        }
        /// The access method isn't allowed for the dataid
        InvalidAccessMethod(dataid: u8) {
            description("Invalid access method")
            display("Invalid access method for data id {}", dataid)
        }

        /// The msgtype of the message is invalid
        InvalidMsgType {
            description("Invalid msg type")
            display("Invalid msg type")
        }

        /// The data payload of the message is invalid
        InvalidData {
            description("Invalid data")
            display("Invalid data")
        }

        /// The application data payload of the message is invalid
        InvalidApplicationData {
            description("Invalid application data")
            display("Invalid application data")
        }

        /// The request of the response is missing
        MissingRequest {
            description("Missing request")
            display("Missing request")
        }

        /// No request is received in time
        ResponseTimeout {
            description("Response timeout")
            display("Response timeout")
        }

        /// Message contains an incorrect parity
        IncorrectParity {
            description("Incorrect parity")
            display("Incorrect parity")
        }
    }
}

/*impl Error
{
    /// Creates a missing_request error. This is an error which is described in the OpenTherm
    /// specification but can't occur during parsing because it is caused by data not available.
    pub fn missing_request() -> Error
    {
        Error::from(ErrorKind::MissingRequest)
    }
    /// Creates a request_timeout error. This is an error which is described in the OpenTherm
    /// specification but can't occur during parsing because it is caused by data not available.
    pub fn response_timeout() -> Error
    {
        Error::from(ErrorKind::ResponseTimeout)
    }

    /// Creates an unknown_dataid error
    pub fn unknown_dataid(dataid: u8) -> Error
    {
        Error::from(ErrorKind::UnknownDataId(dataid))
    }

    /// Creates an invalid access method error
    pub fn invalid_access_method(dataid: u8) -> Error
    {
        Error::from(ErrorKind::InvalidAccessMethod(dataid))
    }

    /// Creates an invalid msgtype error
    pub fn invalid_msgtype() -> Error
    {
        Error::from(ErrorKind::InvalidMsgType)
    }

    /// Creates an invalid data error
    pub fn invalid_data() -> Error
    {
        Error::from(ErrorKind::InvalidData)
    }

    /// Creates an invalid applicationdata error
    pub fn invalid_applicationdata() -> Error
    {
        Error::from(ErrorKind::InvalidApplicationData)
    }

    /// Creates an incorrect parity error
    pub fn incorrect_parity() -> Error
    {
        Error::from(ErrorKind::IncorrectParity)
    }
}*/

/*impl StdError for Error
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
*/