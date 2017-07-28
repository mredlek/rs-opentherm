use super::{DataClass, DataIdDefinition};
use ::std::marker::PhantomData;

pub type DataIdSimpleType = f32;

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 23,
        class: DataClass::SensorAndInformationalData,
        read: false,
        write: true,
        check: Some(|simpletype| if simpletype >= -40f32 && simpletype <= 127f32 { Ok(()) } else { Err(super::Error::InvalidApplicationData) } ),
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(room_setpoint_central_heating2: f32);