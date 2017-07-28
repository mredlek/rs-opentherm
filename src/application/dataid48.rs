use super::{DataClass, DataIdDefinition};
use ::std::marker::PhantomData;

pub type DataIdSimpleType = (i8, i8);

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType,DataIdType> =
    DataIdDefinition {
        data_id: 48,
        class: DataClass::RemoteBoilerParameters,
        read: true,
        write: false,
        check: None,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(upper_bound: i8, lower_bound: i8);