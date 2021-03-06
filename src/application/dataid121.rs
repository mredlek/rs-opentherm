use super::{DataClass, DataIdDefinition};
use ::std::marker::PhantomData;

pub type DataIdSimpleType = u16;

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 121,
        class: DataClass::SensorAndInformationalData,
        read: true,
        write: true,
        check: None,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(central_heating_operation_hours: u16);