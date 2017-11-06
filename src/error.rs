#![allow(unused_doc_comment)]

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
