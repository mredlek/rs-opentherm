use super::{DataClass, DataIdDefinition};
use ::std::marker::PhantomData;
use ::ErrorKind;

pub type DataIdSimpleType = i16;

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 33,
        class: DataClass::SensorAndInformationalData,
        read: true,
        write: false,
        check: Some(|simpletype| if simpletype >= -40i16 && simpletype <= 500i16 { Ok(()) } else { Err(ErrorKind::InvalidApplicationData.into()) } ),
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(exhaust_temperature: i16);