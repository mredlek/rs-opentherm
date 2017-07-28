use super::{DataClass, DataIdDefinition};
use ::std::marker::PhantomData;

pub type DataIdSimpleType = f32;

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 125,
        class: DataClass::ConfigurationInformation,
        read: true,
        write: false,
        check: Some(|simpletype| if simpletype >= 0f32 && simpletype <= 127f32 { Ok(()) } else { Err(super::Error::InvalidApplicationData) } ),
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(opentherm_version_slave: f32);
