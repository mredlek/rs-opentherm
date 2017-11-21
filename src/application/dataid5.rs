use super::{DataClass, DataIdDefinition, Flags8};
use ::std::marker::PhantomData;
use ::serde::{Serializer,Serialize,Deserializer,Deserialize};

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
        /// Service request
        const SERVICE_REQUEST = 0x01;
        /// Lockout reset
        const LOCKOUT_RESET = 0x02;
        /// Low water pressure
        const LOW_WATER_PRESSURE = 0x04;
        /// Gas flame fault
        const GAS_FLAME_FAULT = 0x08;
        /// Air pressure fault
        const AIR_PRESSURE_FAULT = 0x10;
        /// Water over temp
        const WATER_OVER_TEMP = 0x20;
    }
}

#[derive(Serialize, Deserialize)]
struct ApplicationSpecificFaultFlagsSerde
{
    service_request : bool,
    lockout_reset : bool,
    low_water_pressure : bool,
    gas_flame_fault : bool,
    air_pressure_fault : bool,
    water_over_temp : bool,
}

impl From<ApplicationSpecificFaultFlags> for ApplicationSpecificFaultFlagsSerde
{
    fn from(input: ApplicationSpecificFaultFlags) -> Self {
        ApplicationSpecificFaultFlagsSerde {
            service_request : input.contains(ApplicationSpecificFaultFlags::SERVICE_REQUEST),
            lockout_reset : input.contains(ApplicationSpecificFaultFlags::LOCKOUT_RESET),
            low_water_pressure : input.contains(ApplicationSpecificFaultFlags::LOW_WATER_PRESSURE),
            gas_flame_fault : input.contains(ApplicationSpecificFaultFlags::GAS_FLAME_FAULT),
            air_pressure_fault : input.contains(ApplicationSpecificFaultFlags::AIR_PRESSURE_FAULT),
            water_over_temp : input.contains(ApplicationSpecificFaultFlags::WATER_OVER_TEMP),
        }
    }
}

impl Into<ApplicationSpecificFaultFlags> for ApplicationSpecificFaultFlagsSerde
{
    fn into(self) -> ApplicationSpecificFaultFlags {
        let mut ret = ApplicationSpecificFaultFlags::from_bits(0u8).unwrap();
        ret.set(ApplicationSpecificFaultFlags::SERVICE_REQUEST, self.service_request);
        ret.set(ApplicationSpecificFaultFlags::LOCKOUT_RESET, self.lockout_reset);
        ret.set(ApplicationSpecificFaultFlags::LOW_WATER_PRESSURE, self.low_water_pressure);
        ret.set(ApplicationSpecificFaultFlags::GAS_FLAME_FAULT, self.gas_flame_fault);
        ret.set(ApplicationSpecificFaultFlags::AIR_PRESSURE_FAULT, self.air_pressure_fault);
        ret.set(ApplicationSpecificFaultFlags::WATER_OVER_TEMP, self.water_over_temp);
        ret
    }
}

impl Serialize for ApplicationSpecificFaultFlags
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        ApplicationSpecificFaultFlagsSerde::from(self.clone()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ApplicationSpecificFaultFlags
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        <ApplicationSpecificFaultFlagsSerde as Deserialize<'de>>::deserialize(deserializer).map(ApplicationSpecificFaultFlagsSerde::into)
    }
}


flags8!(ApplicationSpecificFaultFlags);
