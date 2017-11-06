use super::{DataClass, DataIdDefinition};
use ::ErrorKind;
use ::std::marker::PhantomData;

pub type DataIdSimpleType = f32;

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 17,
        class: DataClass::SensorAndInformationalData,
        read: true,
        write: false,
        check: Some(|simpletype| if simpletype >= 0f32 && simpletype <= 100f32 { Ok(()) } else { Err(ErrorKind::InvalidApplicationData.into()) } ),
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(relative_modulation_level: f32);