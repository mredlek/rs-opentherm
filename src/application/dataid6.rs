use super::{DataClass, DataIdDefinition, Flags8};
use ::std::marker::PhantomData;
use ::serde::{Deserialize, Serialize, Deserializer, Serializer};

pub type DataIdSimpleType = (Flags8, Flags8);

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 6,
        class: DataClass::RemoteBoilerParameters,
        read: true,
        write: false,
        check: None,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(transfer_enabled_flags: RemoteParameter, readwrite_flags: RemoteParameter);

bitflags! {
    /// Remote parameter
    pub struct RemoteParameter : u8
    {
        /// Dhw setpoint
        const DHW_SETPOINT = 0x01;
        /// Max ch setpoint
        const MAX_CH_SETPOINT = 0x02;
    }
}

#[derive(Serialize, Deserialize)]
struct RemoteParameterSerde
{
    dhw_setpoint : bool,
    max_ch_setpoint : bool,
}

impl From<RemoteParameter> for RemoteParameterSerde
{
    fn from(input: RemoteParameter) -> Self {
        RemoteParameterSerde {
            dhw_setpoint : input.contains(RemoteParameter::DHW_SETPOINT),
            max_ch_setpoint : input.contains(RemoteParameter::MAX_CH_SETPOINT),
        }
    }
}

impl Into<RemoteParameter> for RemoteParameterSerde
{
    fn into(self) -> RemoteParameter {
        let mut ret = RemoteParameter::from_bits(0u8).unwrap();
        ret.set(RemoteParameter::DHW_SETPOINT, self.dhw_setpoint);
        ret.set(RemoteParameter::MAX_CH_SETPOINT, self.max_ch_setpoint);
        ret
    }
}

impl Serialize for RemoteParameter
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        RemoteParameterSerde::from(self.clone()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RemoteParameter
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        <RemoteParameterSerde as Deserialize<'de>>::deserialize(deserializer).map(RemoteParameterSerde::into)
    }
}


flags8!(RemoteParameter);
