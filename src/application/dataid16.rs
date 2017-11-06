use super::{DataClass, DataIdDefinition};
use ::ErrorKind;
use ::std::marker::PhantomData;

pub type DataIdSimpleType = f32;

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 16,
        class: DataClass::SensorAndInformationalData,
        read: false,
        write: true,
        check: Some(|simpletype| if simpletype >= -40f32 && simpletype <= 127f32 { Ok(()) } else { Err(ErrorKind::InvalidApplicationData.into()) } ),
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(room_setpoint: f32);