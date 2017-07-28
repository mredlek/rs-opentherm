use super::{DataClass, DataIdDefinition, Flags8};
use ::std::marker::PhantomData;

pub type DataIdSimpleType = (Flags8, Flags8);

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 6,
        class: DataClass::RemoteBoilerParameters,
        read: true,
        write: false,
        check: None,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(transfer_enabled_flags: RemoteParameter, readwrite_flags: RemoteParameter);

bitflags! {
    /// Remote parameter
    pub struct RemoteParameter : u8
    {
        const DHW_SETPOINT = 0x01;
        const MAX_CH_SETPOINT = 0x02;
    }
}

flags8!(RemoteParameter);
