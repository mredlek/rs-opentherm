use super::{DataClass, DataIdDefinition};
use ::std::marker::PhantomData;

pub type DataIdSimpleType = u8;

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 4,
        class: DataClass::RemoteCommands,
        read: true,
        write: true,
        check: None,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

/*
pub static DATAID_READ_DEFINITION : DataIdDefinition<u8, DataIdType> =
    DataIdDefinition {
        data_id: 4,
        class: DataClass::RemoteCommands,
        read: true,
        write: false,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

pub static DATAID_WRITE_DEFINITION : DataIdDefinition<u8, DataIdType> =
    DataIdDefinition {
        data_id: 4,
        class: DataClass::RemoteCommands,
        read: false,
        write: true,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };*/

dataidtypedef!(command_code: u8);