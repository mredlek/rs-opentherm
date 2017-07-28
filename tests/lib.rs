extern crate rs_opentherm;

use rs_opentherm::{DataId, Message, MsgType};

#[test]
fn it_works() {
    let msg = Message::new([0, 0, 0, 0]).unwrap();
    assert_eq!(format!("{:x}", msg).as_str(), "00000000");
    assert_eq!(msg.data_id(), 0);
    assert_eq!(msg.msg_type(), MsgType::ReadData);
    /*if let Some(ComplexType::Status(ref data)) = msg.data_value_complex() {
        assert_eq!(data.master_status.into(), rs_opentherm::application::dataid0::MasterStatus::empty());
        assert_eq!(data.slave_status, 0u8);
    } else {
        panic!("Expected ComplexType::Status, found {:?}", msg.data_value_complex());
    }*/
    //assert_eq!(msg.data_value_complex(), ComplexType::)
}