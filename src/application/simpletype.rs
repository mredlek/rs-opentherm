use super::flags8::Flags8;
use ::message::{Message, DataId};
use ::Error;

/// The typed payload of the message without the interpretation
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
    pub(crate) fn new(msg: &Message) -> Result<SimpleTypeEnum, Error>
    {
        super::complextype::to_simple_type(msg.data_id(), msg.data_value())
    }
}