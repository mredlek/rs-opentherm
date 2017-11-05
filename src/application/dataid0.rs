use super::{DataClass, DataIdDefinition, Flags8};
use ::std::marker::PhantomData;

pub type DataIdSimpleType = (Flags8, Flags8);

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType,DataIdType> =
    DataIdDefinition {
        data_id: 0,
        class: DataClass::ControlAndStatusInformation,
        read: true,
        write: false,
        check: None,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(master_status: MasterStatus, slave_status: SlaveStatus);

bitflags! {
    /// Masterstatus
    #[derive(Serialize, Deserialize)]
    pub struct MasterStatus : u8
    {
        const CH_ENABLE = 0x01;
        const DHW_ENABLE = 0x02;
        const COOLING_ENABLE = 0x04;
        const OTC_ACTIVE = 0x08;
        const CH2_ENABLE = 0x10;
    }
}

bitflags! {
    /// Slavestatus
    #[derive(Serialize, Deserialize)]
    pub struct SlaveStatus : u8
    {
        const FAULT_INDICATION = 0x01;
        const CH_MODE = 0x02;
        const DHW_MODE = 0x04;
        const FRAME_STATUS = 0x08;
        const COOLING_STATUS = 0x10;
        const CH2_MODE = 0x20;
        const DIAGNOSTIC_INDICATION = 0x40;
    }
}

flags8!(MasterStatus);
flags8!(SlaveStatus);

