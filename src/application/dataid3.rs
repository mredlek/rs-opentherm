use super::{DataClass, DataIdDefinition, Flags8};
use ::std::marker::PhantomData;

pub type DataIdSimpleType = (Flags8, u8);

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 3,
        class: DataClass::ConfigurationInformation,
        read: true,
        write: false,
        check: None,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(slave_configuration: SlaveConfiguration, slave_memberid_code: u8);

bitflags! {
    /// Slave configuration payload
    #[derive(Serialize, Deserialize)]
    pub struct SlaveConfiguration : u8
    {
        const DWH_PRESENT = 0x01;
        const CONTROL_TYPE = 0x02;
        const COOLING_CONFIG = 0x04;
        const DHW_CONFIG = 0x08;
        const MASTER_LOW_OFF_AND_PUMP_CONTROL_FUNCTION = 0x10;
        const CH2_PRESENT = 0x20;
    }
}
flags8!(SlaveConfiguration);

