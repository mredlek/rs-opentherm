use super::{DataClass, DataIdDefinition, Flags8};
use ::std::marker::PhantomData;

pub type DataIdSimpleType = (Flags8, u8);

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 2,
        class: DataClass::ConfigurationInformation,
        read: false,
        write: true,
        check: None,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(master_configuration: Flags8, master_memberid_code: u8);

/*bitflags! {
    pub struct MasterConfiguration : u8
    {
        const RESERVED1 = 0x01;
        const RESERVED2 = 0x02;
    }
}
flags8!(MasterConfiguration );*/

