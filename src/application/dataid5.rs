use super::{DataClass, DataIdDefinition, Flags8};
use ::std::marker::PhantomData;

pub type DataIdSimpleType = (Flags8, u8);

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 5,
        class: DataClass::ControlAndStatusInformation,
        read: true,
        write: false,
        check: None,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(application_specific_fault_flags: ApplicationSpecificFaultFlags, oem_fault_code: u8);

bitflags! {
    /// Application specific fault flags
    pub struct ApplicationSpecificFaultFlags : u8
    {
        const SERVICE_REQUEST = 0x01;
        const LOCKOUT_RESET = 0x02;
        const LOW_WATER_PRESSURE = 0x04;
        const GAS_FLAME_FAULT = 0x08;
        const AIR_PRESSURE_FAULT = 0x10;
        const WATER_OVER_TEMP = 0x20;
    }
}

flags8!(ApplicationSpecificFaultFlags);
