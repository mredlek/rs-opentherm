
use super::{DataClass, DataIdDefinition, Flags8};
use ::std::marker::PhantomData;
use ::serde::{Serialize, Deserialize, Serializer, Deserializer};

pub type DataIdSimpleType = (Flags8, Flags8);

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType,DataIdType> =
    DataIdDefinition {
        data_id: 0,
        class: DataClass::ControlAndStatusInformation,
        read: true,
        write: false,
        check: None,
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

dataidtypedef!(master_status: MasterStatus, slave_status: SlaveStatus);

bitflags! {
    /// Masterstatus
    #[allow(missing_docs)]
    pub struct MasterStatus : u8
    {
        /// Central heating enable
        const CH_ENABLE = 0x01;
        /// Domestic Hot Water Enable
        const DHW_ENABLE = 0x02;
        /// Cooling enable
        const COOLING_ENABLE = 0x04;
        /// Otc active
        const OTC_ACTIVE = 0x08;
        /// Central heating 2 enable
        const CH2_ENABLE = 0x10;
    }
}

bitflags! {
    /// Slavestatus
    pub struct SlaveStatus : u8
    {
        /// Fault indication
        const FAULT_INDICATION = 0x01;
        /// Ch mode
        const CH_MODE = 0x02;
        /// Dhw mode
        const DHW_MODE = 0x04;
        /// Frame status
        const FRAME_STATUS = 0x08;
        /// Cooling Status
        const COOLING_STATUS = 0x10;
        /// CH 2 mode
        const CH2_MODE = 0x20;
        /// Diagnostic indication
        const DIAGNOSTIC_INDICATION = 0x40;
    }
}

#[derive(Serialize, Deserialize)]
struct MasterStatusSerde
{
    ch_enable : bool,
    dhw_enable : bool,
    cooling_enable : bool,
    otc_active : bool,
    ch2_enable : bool,
}

impl From<MasterStatus> for MasterStatusSerde
{
    fn from(input: MasterStatus) -> Self {
        MasterStatusSerde {
            ch_enable : input.contains(MasterStatus::CH_ENABLE),
            dhw_enable : input.contains(MasterStatus::DHW_ENABLE),
            cooling_enable : input.contains(MasterStatus::COOLING_ENABLE),
            otc_active : input.contains(MasterStatus::OTC_ACTIVE),
            ch2_enable : input.contains(MasterStatus::CH2_ENABLE),
        }
    }
}

impl Into<MasterStatus> for MasterStatusSerde
{
    fn into(self) -> MasterStatus {
        let mut ret = MasterStatus::from_bits(0u8).unwrap();
        ret.set(MasterStatus::CH_ENABLE, self.ch_enable);
        ret.set(MasterStatus::DHW_ENABLE, self.dhw_enable);
        ret.set(MasterStatus::COOLING_ENABLE, self.cooling_enable);
        ret.set(MasterStatus::OTC_ACTIVE, self.otc_active);
        ret.set(MasterStatus::CH2_ENABLE, self.ch2_enable);
        ret
    }
}

impl Serialize for MasterStatus
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        MasterStatusSerde::from(self.clone()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MasterStatus
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        <MasterStatusSerde as Deserialize<'de>>::deserialize(deserializer).map(MasterStatusSerde::into)
    }
}

#[derive(Serialize, Deserialize)]
struct SlaveStatusSerde
{
    fault_indication : bool,
    ch_mode : bool,
    dhw_mode : bool,
    frame_status : bool,
    cooling_status : bool,
    ch2_mode : bool,
    diagnostic_indication : bool,
}

impl From<SlaveStatus> for SlaveStatusSerde
{
    fn from(input: SlaveStatus) -> Self {
        SlaveStatusSerde {
            fault_indication: input.contains(SlaveStatus::FAULT_INDICATION),
            ch_mode : input.contains(SlaveStatus::CH_MODE),
            dhw_mode : input.contains(SlaveStatus::DHW_MODE),
            frame_status : input.contains(SlaveStatus::FRAME_STATUS),
            cooling_status : input.contains(SlaveStatus::COOLING_STATUS),
            ch2_mode : input.contains(SlaveStatus::CH2_MODE),
            diagnostic_indication : input.contains(SlaveStatus::DIAGNOSTIC_INDICATION),
        }
    }
}

impl Into<SlaveStatus> for SlaveStatusSerde
{
    fn into(self) -> SlaveStatus {
        let mut ret = SlaveStatus::from_bits(0u8).unwrap();
        ret.set(SlaveStatus::FAULT_INDICATION, self.fault_indication);
        ret.set(SlaveStatus::CH_MODE, self.ch_mode);
        ret.set(SlaveStatus::DHW_MODE, self.dhw_mode);
        ret.set(SlaveStatus::FRAME_STATUS, self.frame_status);
        ret.set(SlaveStatus::COOLING_STATUS, self.cooling_status);
        ret.set(SlaveStatus::CH2_MODE, self.ch2_mode);
        ret.set(SlaveStatus::DIAGNOSTIC_INDICATION, self.diagnostic_indication);
        ret
    }
}

impl Serialize for SlaveStatus
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer {
        SlaveStatusSerde::from(self.clone()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SlaveStatus
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        <SlaveStatusSerde as Deserialize<'de>>::deserialize(deserializer).map(SlaveStatusSerde::into)
    }
}

flags8!(MasterStatus);
flags8!(SlaveStatus);
