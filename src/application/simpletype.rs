use super::flags8::Flags8;
use ::message::{AsDataId, AsDataValue};
use ::application::ComplexType;
use ::Error;

/// The typed payload of the message without the interpretation
#[derive(Copy, Clone)]
pub enum SimpleTypeEnum
{
    /// High byte u8, low byte u8
    U8u8(u8, u8),
    /// High byte u8, low byte s8
    U8s8(u8, i8),
    /// High byte u8, low byte flags8
    U8flags8(u8, Flags8),
    /// High byte s8, low byte u8
    S8u8(i8, u8),
    /// High byte s8, low byte s8
    S8s8(i8, i8),
    /// High byte s8, low byte flags8
    S8flags8(i8, Flags8),
    /// High byte flags8, low byte u8
    Flags8u8(Flags8, u8),
    /// High byte flags8, low byte i8
    Flags8s8(Flags8, i8),
    /// High byte flags8, low byte flags8
    Flags8flags8(Flags8, Flags8),
    /// High byte u8, no low byte defined
    U8(u8),
    /// High byte s8, no low byte defined
    S8(i8),
    /// High byte flags8, no low byte defined
    Flags8(Flags8),
    /// High and low bytes combined into u16
    U16(u16),
    /// High and low bytes combined into s16
    S16(i16),
    /// High and low bytes combined into f8.8 (fixed point number)
    F88(f32)
}

impl From<(u8,u8)> for SimpleTypeEnum
{
    fn from(input: (u8,u8)) -> SimpleTypeEnum
    {
        SimpleTypeEnum::U8u8(input.0, input.1)
    }
}

impl From<(u8,i8)> for SimpleTypeEnum
{
    fn from(input: (u8,i8)) -> SimpleTypeEnum
    {
        SimpleTypeEnum::U8s8(input.0, input.1)
    }
}

impl From<(u8,Flags8)> for SimpleTypeEnum
{
    fn from(input: (u8,Flags8)) -> SimpleTypeEnum
    {
        SimpleTypeEnum::U8flags8(input.0, input.1)
    }
}

impl From<(i8,u8)> for SimpleTypeEnum
{
    fn from(input: (i8,u8)) -> SimpleTypeEnum
    {
        SimpleTypeEnum::S8u8(input.0, input.1)
    }
}

impl From<(i8,i8)> for SimpleTypeEnum
{
    fn from(input: (i8,i8)) -> SimpleTypeEnum
    {
        SimpleTypeEnum::S8s8(input.0, input.1)
    }
}

impl From<(i8,Flags8)> for SimpleTypeEnum
{
    fn from(input: (i8,Flags8)) -> SimpleTypeEnum
    {
        SimpleTypeEnum::S8flags8(input.0, input.1)
    }
}
impl From<(Flags8,u8)> for SimpleTypeEnum
{
    fn from(input: (Flags8,u8)) -> SimpleTypeEnum
    {
        SimpleTypeEnum::Flags8u8(input.0, input.1)
    }
}

impl From<(Flags8,i8)> for SimpleTypeEnum
{
    fn from(input: (Flags8,i8)) -> SimpleTypeEnum
    {
        SimpleTypeEnum::Flags8s8(input.0, input.1)
    }
}

impl From<(Flags8,Flags8)> for SimpleTypeEnum
{
    fn from(input: (Flags8,Flags8)) -> SimpleTypeEnum
    {
        SimpleTypeEnum::Flags8flags8(input.0, input.1)
    }
}

impl From<u8> for SimpleTypeEnum
{
    fn from(input: u8) -> SimpleTypeEnum
    {
        SimpleTypeEnum::U8(input)
    }
}

impl From<i8> for SimpleTypeEnum
{
    fn from(input: i8) -> SimpleTypeEnum
    {
        SimpleTypeEnum::S8(input)
    }
}

impl From<Flags8> for SimpleTypeEnum
{
    fn from(input: Flags8) -> SimpleTypeEnum
    {
        SimpleTypeEnum::Flags8(input)
    }
}

impl From<u16> for SimpleTypeEnum
{
    fn from(input: u16) -> SimpleTypeEnum
    {
        SimpleTypeEnum::U16(input)
    }
}

impl From<i16> for SimpleTypeEnum
{
    fn from(input: i16) -> SimpleTypeEnum
    {
        SimpleTypeEnum::S16(input)
    }
}

impl From<f32> for SimpleTypeEnum
{
    fn from(input: f32) -> SimpleTypeEnum
    {
        SimpleTypeEnum::F88(input)
    }
}

impl SimpleTypeEnum
{
    /*pub(crate) fn new_from_message(msg: &(Message + 'static)) -> Result<SimpleTypeEnum, Error>
    {
        super::complextype::to_simple_type((msg as &Message).data_id(), msg.data_value())
    }*/

    /// Creates a new simpletype from a class implementing dataid and a class implementing datavalue
    pub fn new<TDataId: AsDataId, TDataValue: AsDataValue>(dataid: &TDataId, datavalue: &AsDataValue) -> Result<SimpleTypeEnum, Error>
    {
        let complextype = try!(ComplexType::new_from_data(dataid.as_data_id(), datavalue.as_data_value()));
        Ok(complextype.into())
    }
}

impl AsDataValue for SimpleTypeEnum
{
    fn as_data_value(&self) -> [u8; 2] {
        match self {
            &SimpleTypeEnum::U8u8(first, second) => (first, second).to_data(),
            &SimpleTypeEnum::U8s8(first, second) => (first, second).to_data(),
            &SimpleTypeEnum::U8flags8(first, second) => (first, second).to_data(),
            &SimpleTypeEnum::S8u8(first, second) => (first, second).to_data(),
            &SimpleTypeEnum::S8s8(first, second) => (first, second).to_data(),
            &SimpleTypeEnum::S8flags8(first, second) => (first, second).to_data(),
            &SimpleTypeEnum::Flags8u8(first, second) => (first, second).to_data(),
            &SimpleTypeEnum::Flags8s8(first, second) => (first, second).to_data(),
            &SimpleTypeEnum::Flags8flags8(first, second) => (first, second).to_data(),
            &SimpleTypeEnum::U8(first) => SimpleType::to_data(first),
            &SimpleTypeEnum::S8(first) => SimpleType::to_data(first),
            &SimpleTypeEnum::Flags8(first) => SimpleType::to_data(first),
            &SimpleTypeEnum::U16(first) => SimpleType::to_data(first),
            &SimpleTypeEnum::S16(first) => SimpleType::to_data(first),
            &SimpleTypeEnum::F88(first) => SimpleType::to_data(first),
        }
    }
}

/// Simple types
pub(crate) trait SimpleSubtype : Copy + Clone
{
    fn from_data(input: u8) -> Self;
    fn to_data(self) -> u8;
}

pub(crate) trait SimpleType : Copy + Clone
{
    fn from_data(input: [u8; 2]) -> Self;
    fn to_data(self) -> [u8; 2];
}

impl SimpleSubtype for u8
{
    fn from_data(input: u8) -> Self {
        input
    }

    fn to_data(self) -> u8 {
        self
    }
}

impl SimpleSubtype for i8
{
    fn from_data(input: u8) -> Self {
        input as i8
    }

    fn to_data(self) -> u8 {
        self as u8
    }
}

impl SimpleSubtype for Flags8
{
    fn from_data(input: u8) -> Self {
        Flags8::from(input)
    }

    fn to_data(self) -> u8 {
        self.bits
    }
}

impl<T1 : SimpleSubtype,T2: SimpleSubtype> SimpleType for (T1, T2)
{
    fn from_data(input: [u8; 2]) -> Self {
        (T1::from_data(input[0]), T2::from_data(input[1]))
    }

    fn to_data(self) -> [u8; 2] {
        [ self.0.to_data(), self.1.to_data() ]
    }
}

impl<T: SimpleSubtype> SimpleType for T
{
    fn from_data(input: [u8; 2]) -> Self {
        T::from_data(input[0])
    }

    fn to_data(self) -> [u8; 2] {
        [self.to_data(), 0]
    }
}

impl SimpleType for u16
{
    fn from_data(input: [u8; 2]) -> Self {
        ((input[0] as u16) << 8) | input[1] as u16
    }

    fn to_data(self) -> [u8; 2] {
        [ (self >> 8) as u8, (self & 0xff) as u8 ]
    }
}

impl SimpleType for i16
{
    fn from_data(input: [u8; 2]) -> Self {
        u16::from_data(input) as i16
    }
    fn to_data(self) -> [u8; 2] {
        (self as u16).to_data()
    }
}

impl SimpleType for f32
{
    fn from_data(input: [u8; 2]) -> Self {
//        (input.data[0] as f32) + (input.data[1] as f32) / 256f32
        (i16::from_data(input) as f32) / 256f32
    }

    fn to_data(self) -> [u8; 2] {
        ((self * 256f32) as i16).to_data()
    }
}
