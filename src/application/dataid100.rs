use super::{DataClass, DataIdDefinition, Flags8};
use ::std::marker::PhantomData;
use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

pub type DataIdSimpleType = Flags8;

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType,DataIdType> =
    DataIdDefinition {
        data_id: 100,
        class: DataClass::ControlAndStatusInformation,
        read: true,
        write: false,
        check: None,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(remote_override_function: RemoteOverride);

bitflags! {
    /// RemoteOverride
    pub struct RemoteOverride : u8
    {
        /// Manual change configuration
        const MANUAL_CHANGE_CONFIGURATION = 0x01;
        /// Program change configuration
        const PROGRAM_CHANGE_CONFIGURATION = 0x01;
    }
}

#[derive(Serialize, Deserialize)]
struct RemoteOverrideSerde
{
    manual_change_configuration : bool,
    program_change_configuration : bool,
}

impl From<RemoteOverride> for RemoteOverrideSerde
{
    fn from(input: RemoteOverride) -> Self {
        RemoteOverrideSerde {
            manual_change_configuration : input.contains(RemoteOverride::MANUAL_CHANGE_CONFIGURATION),
            program_change_configuration : input.contains(RemoteOverride::PROGRAM_CHANGE_CONFIGURATION),
        }
    }
}

impl Into<RemoteOverride> for RemoteOverrideSerde
{
    fn into(self) -> RemoteOverride {
        let mut ret = RemoteOverride::from_bits(0u8).unwrap();
        ret.set(RemoteOverride::MANUAL_CHANGE_CONFIGURATION, self.manual_change_configuration);
        ret.set(RemoteOverride::PROGRAM_CHANGE_CONFIGURATION, self.program_change_configuration);
        ret
    }
}

impl Serialize for RemoteOverride
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        RemoteOverrideSerde::from(self.clone()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RemoteOverride
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        <RemoteOverrideSerde as Deserialize<'de>>::deserialize(deserializer).map(RemoteOverrideSerde::into)
    }
}

flags8!(RemoteOverride);

