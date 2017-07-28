use super::{DataClass, DataIdDefinition};
use ::std::marker::PhantomData;

pub type DataIdSimpleType = f32;

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 8,
        class: DataClass::ControlAndStatusInformation,
        read: false,
        write: true,
        check: Some(|simpletype| if simpletype >= 0f32 && simpletype <= 100f32 { Ok(()) } else { Err(super::Error::InvalidApplicationData) } ),
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(control_setpoint2: f32);