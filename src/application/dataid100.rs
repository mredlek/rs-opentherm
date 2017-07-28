use super::{DataClass, DataIdDefinition, Flags8};
use ::std::marker::PhantomData;

pub type DataIdSimpleType = Flags8;

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType,DataIdType> =
    DataIdDefinition {
        data_id: 100,
        class: DataClass::ControlAndStatusInformation,
        read: true,
        write: false,
        check: None,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(remote_override_function: RemoteOverride);

bitflags! {
    /// RemoteOverride
    pub struct RemoteOverride : u8
    {
        const MANUAL_CHANGE_CONFIGURATION = 0x01;
        const PROGRAM_CHANGE_CONFIGURATION = 0x01;
    }
}

flags8!(RemoteOverride);

