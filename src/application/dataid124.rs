use super::{DataClass, DataIdDefinition};
use ::ErrorKind;
use ::std::marker::PhantomData;

pub type DataIdSimpleType = f32;

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 124,
        class: DataClass::ConfigurationInformation,
        read: false,
        write: true,
        check: Some(|simpletype| if simpletype >= 0f32 && simpletype <= 127f32 { Ok(()) } else { Err(ErrorKind::InvalidApplicationData.into()) } ),
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(opentherm_version_master: f32);