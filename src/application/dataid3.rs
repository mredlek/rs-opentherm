use super::{DataClass, DataIdDefinition, Flags8};
use ::std::marker::PhantomData;
use ::serde::{Serializer,Serialize,Deserializer,Deserialize};

pub type DataIdSimpleType = (Flags8, u8);

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 3,
        class: DataClass::ConfigurationInformation,
        read: true,
        write: false,
        check: None,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(slave_configuration: SlaveConfiguration, slave_memberid_code: u8);

bitflags! {
    /// Slave configuration payload
    pub struct SlaveConfiguration : u8
    {
        /// Domestic Hot Water present
        const DWH_PRESENT = 0x01;
        /// Controle Type
        const CONTROL_TYPE = 0x02;
        /// Cooling config
        const COOLING_CONFIG = 0x04;
        /// DHW config
        const DHW_CONFIG = 0x08;
        /// Master low off and pump control function
        const MASTER_LOW_OFF_AND_PUMP_CONTROL_FUNCTION = 0x10;
        /// CH2 present
        const CH2_PRESENT = 0x20;
    }
}

#[derive(Serialize, Deserialize)]
struct SlaveConfigurationSerde
{
    dwh_present : bool,
    control_type : bool,
    cooling_config : bool,
    dhw_config : bool,
    master_low_off_and_pump_control_function : bool,
    ch2_present : bool,
}

impl From<SlaveConfiguration> for SlaveConfigurationSerde
{
    fn from(input: SlaveConfiguration) -> Self {
        SlaveConfigurationSerde {
            dwh_present : input.contains(SlaveConfiguration::DWH_PRESENT),
            control_type : input.contains(SlaveConfiguration::CONTROL_TYPE),
            cooling_config : input.contains(SlaveConfiguration::COOLING_CONFIG),
            dhw_config : input.contains(SlaveConfiguration::DHW_CONFIG),
            master_low_off_and_pump_control_function : input.contains(SlaveConfiguration::MASTER_LOW_OFF_AND_PUMP_CONTROL_FUNCTION),
            ch2_present : input.contains(SlaveConfiguration::CH2_PRESENT),
        }
    }
}

impl Into<SlaveConfiguration> for SlaveConfigurationSerde
{
    fn into(self) -> SlaveConfiguration {
        let mut ret = SlaveConfiguration::from_bits(0u8).unwrap();
        ret.set(SlaveConfiguration::DWH_PRESENT, self.dwh_present);
        ret.set(SlaveConfiguration::CONTROL_TYPE, self.control_type);
        ret.set(SlaveConfiguration::COOLING_CONFIG, self.cooling_config);
        ret.set(SlaveConfiguration::DHW_CONFIG, self.dhw_config);
        ret.set(SlaveConfiguration::MASTER_LOW_OFF_AND_PUMP_CONTROL_FUNCTION, self.master_low_off_and_pump_control_function);
        ret.set(SlaveConfiguration::CH2_PRESENT, self.ch2_present);
        ret
    }
}

impl Serialize for SlaveConfiguration
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        SlaveConfigurationSerde::from(self.clone()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SlaveConfiguration
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        <SlaveConfigurationSerde as Deserialize<'de>>::deserialize(deserializer).map(SlaveConfigurationSerde::into)
    }
}


flags8!(SlaveConfiguration);

