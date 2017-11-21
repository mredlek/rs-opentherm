extern crate opentherm;

use ::std::ops::Deref;
use opentherm::{AsDataId, Message, MsgType, AsMsgType, AsDataValue};

pub struct TestMessage
{
    data: [u8; 4]
}

impl From<[u8; 4]> for TestMessage
{
    fn from(data: [u8; 4]) -> Self {
        TestMessage { data: data }
    }
}

impl Message for TestMessage
{
    fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Deref for TestMessage
{
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        self
    }
}

#[test]
fn it_works() {
    let msg = TestMessage::from([0u8, 0u8, 0u8, 0u8]);
    assert_eq!(msg.as_data_id(), 0);
    assert_eq!(msg.as_data_value(), [0u8, 0u8]);
    assert_eq!(msg.as_msg_type(), MsgType::ReadData);
    /*if let Some(ComplexType::Status(ref data)) = msg.data_value_complex() {
        assert_eq!(data.master_status.into(), rs_opentherm::application::dataid0::MasterStatus::empty());
        assert_eq!(data.slave_status, 0u8);
    } else {
        panic!("Expected ComplexType::Status, found {:?}", msg.data_value_complex());
    }*/
    //assert_eq!(msg.data_value_complex(), ComplexType::)
}