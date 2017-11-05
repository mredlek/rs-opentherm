macro_rules! flags8 {
    ($BitFlags:ident) => {
        impl From<u8> for $BitFlags
        {
            fn from(from: u8) -> Self {
                $BitFlags::from_bits_truncate(from)
            }
        }

        impl Into<u8> for $BitFlags
        {
            fn into(self) -> u8
            {
                self.bits
            }
        }
        impl From<Flags8> for $BitFlags
        {
            fn from(from: Flags8) -> Self {
                $BitFlags::from_bits_truncate(from.bits)
            }
        }

        impl Into<Flags8> for $BitFlags
        {
            fn into(self) -> Flags8
            {
                Flags8 { bits: self.bits }
            }
        }
    }
}

macro_rules! dataidtypedef {
    ($name1:ident: $type1:ident) => {
        /// The payload of a message with dataid $id
        #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
        pub struct DataIdType {
            /// Field $name1 (description is available in the OpenTherm documentation)
            pub $name1: $type1
        }
        impl<T> From<T> for DataIdType
            where $type1: From<T>
        {
            fn from(input: T) -> Self {
                DataIdType{ $name1: $type1::from(input)                 }
            }
        }
        impl Into<self::DataIdSimpleType> for DataIdType
        {
            fn into(self) -> self::DataIdSimpleType
            {
                self.$name1.into()
            }
        }
    };
    ($name1:ident: $type1:ident, $name2:ident: $type2:ident) => {
        /// The payload of a message with dataid $id
        #[derive(Copy, Clone, Debug, Serialize, Deserialize)]
        pub struct DataIdType {
            /// Field $name1 (description is available in the OpenTherm documentation)
            pub $name1: $type1,
            /// Field $name2 (description is available in the OpenTherm documentation)
            pub $name2: $type2
        }
        impl<High, Low> From<(High,Low)> for DataIdType
            where $type1: From<High>,
                $type2: From<Low>
        {
            fn from(from: (High, Low)) -> Self {
                DataIdType { $name1: $type1::from(from.0), $name2: $type2::from(from.1) }
            }
        }
        impl Into<self::DataIdSimpleType> for DataIdType {
            fn into(self) -> self::DataIdSimpleType
            {
                (self.$name1.into(), self.$name2.into())
            }
        }
    };
}

mod complextype;
mod dataid0;
mod dataid1;
mod dataid2;
mod dataid3;
mod dataid4;
mod dataid5;
mod dataid6;
mod dataid7;
mod dataid8;
mod dataid9;
mod dataid10;
mod dataid11;
mod dataid12;
mod dataid13;
mod dataid14;
mod dataid15;
mod dataid16;
mod dataid17;
mod dataid18;
mod dataid19;
mod dataid20;
mod dataid21;
mod dataid22;
mod dataid23;
mod dataid24;
mod dataid25;
mod dataid26;
mod dataid27;
mod dataid28;
mod dataid29;
mod dataid30;
mod dataid31;
mod dataid32;
mod dataid33;
mod dataid48;
mod dataid49;
mod dataid56;
mod dataid57;
mod dataid100;
mod dataid115;
mod dataid116;
mod dataid117;
mod dataid118;
mod dataid119;
mod dataid120;
mod dataid121;
mod dataid122;
mod dataid123;
mod dataid124;
mod dataid125;
mod dataid126;
mod dataid127;
mod flags8;
mod simpletype;

use ::Error;
use ::message::{DataId, Message, MsgType};
use ::std::ops::Index;
use ::std::marker::PhantomData;
pub use self::flags8::Flags8;
pub use self::simpletype::SimpleTypeEnum;
pub use self::complextype::ComplexType;

pub use self::dataid0::{DataIdType as DataId0Type,MasterStatus,SlaveStatus};
pub use self::dataid1::DataIdType as DataId1Type;
pub use self::dataid2::DataIdType as DataId2Type;
pub use self::dataid3::{DataIdType as DataId3Type,SlaveConfiguration};
pub use self::dataid4::DataIdType as DataId4Type;
pub use self::dataid5::{DataIdType as DataId5Type,ApplicationSpecificFaultFlags};
pub use self::dataid6::{DataIdType as DataId6Type,RemoteParameter};
pub use self::dataid7::DataIdType as DataId7Type;
pub use self::dataid8::DataIdType as DataId8Type;
pub use self::dataid10::DataIdType as DataId10Type;
pub use self::dataid11::DataIdType as DataId11Type;
pub use self::dataid12::DataIdType as DataId12Type;
pub use self::dataid13::DataIdType as DataId13Type;
pub use self::dataid14::DataIdType as DataId14Type;
pub use self::dataid15::DataIdType as DataId15Type;
pub use self::dataid16::DataIdType as DataId16Type;
pub use self::dataid17::DataIdType as DataId17Type;
pub use self::dataid18::DataIdType as DataId18Type;
pub use self::dataid19::DataIdType as DataId19Type;
pub use self::dataid20::{DataIdType as DataId20Type,DayOfWeek};
pub use self::dataid21::DataIdType as DataId21Type;
pub use self::dataid22::DataIdType as DataId22Type;
pub use self::dataid23::DataIdType as DataId23Type;
pub use self::dataid24::DataIdType as DataId24Type;
pub use self::dataid25::DataIdType as DataId25Type;
pub use self::dataid26::DataIdType as DataId26Type;
pub use self::dataid27::DataIdType as DataId27Type;
pub use self::dataid28::DataIdType as DataId28Type;
pub use self::dataid29::DataIdType as DataId29Type;
pub use self::dataid30::DataIdType as DataId30Type;
pub use self::dataid31::DataIdType as DataId31Type;
pub use self::dataid32::DataIdType as DataId32Type;
pub use self::dataid33::DataIdType as DataId33Type;
pub use self::dataid48::DataIdType as DataId48Type;
pub use self::dataid49::DataIdType as DataId49Type;
pub use self::dataid56::DataIdType as DataId56Type;
pub use self::dataid57::DataIdType as DataId57Type;
pub use self::dataid100::{DataIdType as DataId100Type,RemoteOverride};
pub use self::dataid115::DataIdType as DataId115Type;
pub use self::dataid116::DataIdType as DataId116Type;
pub use self::dataid117::DataIdType as DataId117Type;
pub use self::dataid118::DataIdType as DataId118Type;
pub use self::dataid119::DataIdType as DataId119Type;
pub use self::dataid120::DataIdType as DataId120Type;
pub use self::dataid121::DataIdType as DataId121Type;
pub use self::dataid122::DataIdType as DataId122Type;
pub use self::dataid123::DataIdType as DataId123Type;
pub use self::dataid124::DataIdType as DataId124Type;
pub use self::dataid125::DataIdType as DataId125Type;
pub use self::dataid126::DataIdType as DataId126Type;
pub use self::dataid127::DataIdType as DataId127Type;

#[derive(Clone,Copy)]
pub(crate) enum DataClass
{
    ControlAndStatusInformation,
    ConfigurationInformation,
    RemoteCommands,
    SensorAndInformationalData,
    RemoteBoilerParameters,
    TransparantSlaveParameters,
    FaultHistoryInformation,
    ControlOfSpecialialApplications
}


#[derive(Copy, Clone)]
pub(crate) struct DataValue<'a>
{
    data: &'a[u8]
}

impl<'dv> From<&'dv[u8]> for DataValue<'dv>
{
    fn from(input: &'dv [u8]) -> DataValue<'dv>
    {
        if input.len() != 2 {
            panic!("DataValue::from expect a slice of [u8; 2], got a slice of length {}", input.len());
        }
        DataValue { data: input }
    }
}

impl<'dv> Index<usize> for DataValue<'dv>
{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output
    {
        if index > 1 {
            panic!("Index out of range: expected an index 0 or 1");
        }
        &self.data[index]
    }
}

impl<'dv> Into<(u8, u8)> for DataValue<'dv>
{
    fn into(self) -> (u8, u8)
    {
        (self.data[0], self.data[1])
    }
}

impl<'dv> Into<(u8, i8)> for DataValue<'dv>
{
    fn into(self) -> (u8, i8)
    {
        (self.data[0], self.data[1] as i8)
    }
}

impl<'dv> Into<(u8, Flags8)> for DataValue<'dv>
{
    fn into(self) -> (u8, Flags8)
    {
        (self.data[0], Flags8::from(self.data[1]))
    }
}
impl<'dv> Into<(i8, u8)> for DataValue<'dv>
{
    fn into(self) -> (i8, u8)
    {
        (self.data[0] as i8, self.data[1])
    }
}

impl<'dv> Into<(i8, i8)> for DataValue<'dv>
{
    fn into(self) -> (i8, i8)
    {
        (self.data[0] as i8, self.data[1] as i8)
    }
}

impl<'dv> Into<(i8, Flags8)> for DataValue<'dv>
{
    fn into(self) -> (i8, Flags8)
    {
        (self.data[0] as i8, Flags8::from(self.data[1]))
    }
}

impl<'dv> Into<(Flags8, u8)> for DataValue<'dv>
{
    fn into(self) -> (Flags8, u8)
    {
        (Flags8::from(self.data[0]), self.data[1])
    }
}

impl<'dv> Into<(Flags8, i8)> for DataValue<'dv>
{
    fn into(self) -> (Flags8, i8)
    {
        (Flags8::from(self.data[0]), self.data[1] as i8)
    }
}

impl<'dv> Into<(Flags8, Flags8)> for DataValue<'dv>
{
    fn into(self) -> (Flags8, Flags8)
    {
        (Flags8::from(self.data[0]), Flags8::from(self.data[1]))
    }
}


impl<'dv> Into<u8> for DataValue<'dv>
{
    fn into(self) -> u8
    {
        self.data[0]
    }
}

impl<'dv> Into<i8> for DataValue<'dv>
{
    fn into(self) -> i8
    {
        self.data[0] as i8
    }
}

impl<'dv> Into<Flags8> for DataValue<'dv>
{
    fn into(self) -> Flags8
    {
        Flags8::from(self.data[0])
    }
}

impl<'dv> Into<u16> for DataValue<'dv>
{
    fn into(self) -> u16
    {
        ((self.data[0] as u16) << 8) | self.data[1] as u16
    }
}


impl<'dv> Into<i16> for DataValue<'dv>
{
    fn into(self) -> i16
    {
        (((self.data[0] as u16) << 8) | self.data[1] as u16) as i16
    }
}


impl<'dv> Into<f32> for DataValue<'dv>
{
    fn into(self) -> f32
    {
        (self.data[0] as f32) + (self.data[1] as f32) / 256f32
    }
}

pub(crate) struct DataIdDefinition<SimpleType, ComplexType>
    where self::simpletype::SimpleTypeEnum: From<SimpleType>,
        ComplexType: From<SimpleType>
{
    data_id: u8,
    read: bool,
    write: bool,
    class: DataClass,
    check: Option<fn(SimpleType) -> Result<(), Error>>,
    phantom_simple: PhantomData<SimpleType>,
    phantom_complex: PhantomData<ComplexType>
}

impl<SimpleType, ComplexType> DataIdDefinition<SimpleType, ComplexType>
    where self::simpletype::SimpleTypeEnum: From<SimpleType>,
          ComplexType: From<SimpleType> + Into<SimpleType>
{
    pub fn simple_data<'dv>(&self, data_value: DataValue<'dv>) -> SimpleType
        where DataValue<'dv>: Into<SimpleType>
    {
        data_value.into()
    }

    pub fn complex_data<'dv>(&self, data_value: DataValue<'dv>) -> ComplexType
        where DataValue<'dv>: Into<SimpleType>
    {
        ComplexType::from(self.simple_data(data_value))
    }

    pub fn complex_to_simple(&self, complex: ComplexType) -> SimpleType
    {
        complex.into()
    }
}

pub(crate) trait DataIdDefinitionUntyped
{
    fn data_id(&self) -> u8;
    fn read(&self) -> bool;
    fn write(&self) -> bool;
    fn class(&self) -> DataClass;
}

pub(crate) trait DataIdDefinitionSimpleType
{
    type SimpleType;

    fn data<'dv>(&self, data_value: DataValue<'dv>) -> Self::SimpleType
        where Self::SimpleType: From<DataValue<'dv>>;
}

impl<SimpleType, ComplexType> DataIdDefinitionUntyped for DataIdDefinition<SimpleType, ComplexType>
    where self::simpletype::SimpleTypeEnum: From<SimpleType>,
          ComplexType: From<SimpleType>
{
    fn data_id(&self) -> u8
    {
        self.data_id
    }

    fn read(&self) -> bool
    {
        self.read
    }

    fn write(&self) -> bool
    {
        self.write
    }

    fn class(&self) -> DataClass
    {
        self.class
    }
}

impl<SimpleType, ComplexType> DataIdDefinitionSimpleType for DataIdDefinition<SimpleType, ComplexType>
    where self::simpletype::SimpleTypeEnum: From<SimpleType>,
          ComplexType: From<SimpleType>
{
    type SimpleType = SimpleType;

    fn data<'dv>(&self, data_value: DataValue<'dv>) -> Self::SimpleType
        where DataValue<'dv>: Into<SimpleType>
    {
        data_value.into()
    }
}

pub(crate) trait OpenthermMessage
{
    fn data_id_definition(&self) -> Result<&'static DataIdDefinitionUntyped, Error>;
    fn data_value_simple(&self) -> Result<SimpleTypeEnum, Error>;
    fn data_value_complex(&self) -> Result<ComplexType, Error>;
}

impl OpenthermMessage for Message
{
    fn data_id_definition(&self) -> Result<&'static DataIdDefinitionUntyped, Error>
    {
        let definition : &'static DataIdDefinitionUntyped = match self.data_id() {
            0 => &dataid0::DATAID_DEFINITION,
            1 => &dataid1::DATAID_DEFINITION,
            2 => &dataid2::DATAID_DEFINITION,
            3 => &dataid3::DATAID_DEFINITION,
            4 => &dataid4::DATAID_DEFINITION,
            5 => &dataid5::DATAID_DEFINITION,
            6 => &dataid6::DATAID_DEFINITION,
            7 => &dataid7::DATAID_DEFINITION,
            8 => &dataid8::DATAID_DEFINITION,
            10 => &dataid10::DATAID_DEFINITION,
            11 => &dataid11::DATAID_DEFINITION,
            12 => &dataid12::DATAID_DEFINITION,
            13 => &dataid13::DATAID_DEFINITION,
            14 => &dataid14::DATAID_DEFINITION,
            15 => &dataid15::DATAID_DEFINITION,
            16 => &dataid16::DATAID_DEFINITION,
            17 => &dataid17::DATAID_DEFINITION,
            18 => &dataid18::DATAID_DEFINITION,
            19 => &dataid19::DATAID_DEFINITION,
            20 => &dataid20::DATAID_DEFINITION,
            21 => &dataid21::DATAID_DEFINITION,
            22 => &dataid22::DATAID_DEFINITION,
            23 => &dataid23::DATAID_DEFINITION,
            24 => &dataid24::DATAID_DEFINITION,
            25 => &dataid25::DATAID_DEFINITION,
            26 => &dataid26::DATAID_DEFINITION,
            27 => &dataid27::DATAID_DEFINITION,
            28 => &dataid28::DATAID_DEFINITION,
            29 => &dataid29::DATAID_DEFINITION,
            30 => &dataid30::DATAID_DEFINITION,
            31 => &dataid31::DATAID_DEFINITION,
            32 => &dataid32::DATAID_DEFINITION,
            33 => &dataid33::DATAID_DEFINITION,
            48 => &dataid48::DATAID_DEFINITION,
            49 => &dataid49::DATAID_DEFINITION,
            56 => &dataid56::DATAID_DEFINITION,
            57 => &dataid57::DATAID_DEFINITION,
            100 => &dataid100::DATAID_DEFINITION,
            115 => &dataid115::DATAID_DEFINITION,
            116 => &dataid116::DATAID_DEFINITION,
            117 => &dataid117::DATAID_DEFINITION,
            118 => &dataid118::DATAID_DEFINITION,
            119 => &dataid119::DATAID_DEFINITION,
            120 => &dataid120::DATAID_DEFINITION,
            121 => &dataid121::DATAID_DEFINITION,
            122 => &dataid122::DATAID_DEFINITION,
            123 => &dataid123::DATAID_DEFINITION,
            124 => &dataid124::DATAID_DEFINITION,
            125 => &dataid125::DATAID_DEFINITION,
            126 => &dataid126::DATAID_DEFINITION,
            127 => &dataid127::DATAID_DEFINITION,
            _ => { return Err(Error::UnknownDataId(self.data_id())) }
        };

        match self.msg_type() {
            MsgType::ReadData | MsgType::ReadAck => if definition.read() { Ok(definition) } else { Err(Error::InvalidAccessMethod(self.data_id())) },
            MsgType::WriteData | MsgType::WriteAck => if definition.write() { Ok(definition) } else { Err(Error::InvalidAccessMethod(self.data_id())) },
            _ => Err(Error::InvalidAccessMethod(self.data_id())) //No access method is an invalid one
        }
    }

    fn data_value_simple(&self) -> Result<SimpleTypeEnum, Error>
    {
        SimpleTypeEnum::new(self)
    }

    fn data_value_complex(&self) -> Result<ComplexType, Error>
    {
        ComplexType::new(self)
    }
}

