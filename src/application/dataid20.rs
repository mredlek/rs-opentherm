use super::{DataClass, DataIdDefinition, Error, ErrorKind};
use ::std::marker::PhantomData;

pub type DataIdSimpleType = (u8, u8);

pub(crate) static DATAID_DEFINITION : DataIdDefinition<DataIdSimpleType, DataIdType> =
    DataIdDefinition {
        data_id: 20,
        class: DataClass::SensorAndInformationalData,
        read: true,
        write: true,
        check: Some(|(st1, st2)| if (st1 & 0x1f) <= 23u8 && st2 <= 59u8  { Ok(()) } else { Err(ErrorKind::InvalidApplicationData.into()) } ),
        phantom_simple: PhantomData {},
        phantom_complex: PhantomData {}
    };

/// Struct to represent the day of the week
#[derive(Clone,Copy,Debug,Serialize,Deserialize)]
pub enum DayOfWeek
{
    /// Monday (first day of the week)
    Monday,
    /// Tuesday (second day of the week)
    Tuesday,
    /// Wednesday (third day of the week)
    Wednesday,
    /// Thursday (fourth day of the week)
    Thursday,
    /// Friday (fifth day of the week)
    Friday,
    /// Saterday (sixth day of the week)
    Saterday,
    /// Sunday (seventh day of the week)
    Sunday
}

/// Struct for dataid 20
#[derive(Copy,Clone,Debug,Serialize,Deserialize)]
pub struct DataIdType
{
    day_of_week: Option<DayOfWeek>,
    hours: u8,
    minutes: u8
}

impl DataIdType {
    /// Returns the day of the week
    pub fn day_of_week(&self) -> Option<DayOfWeek>
    {
        self.day_of_week
    }

    /// Returns the number of hours
    pub fn hours(&self) -> u8
    {
        self.hours
    }

    /// Returns the number of minutes
    pub fn minutes(&self) -> u8
    {
        self.minutes
    }
}

impl From<(u8,u8)> for DataIdType
{
    fn from((high, low): (u8, u8)) -> Self {
        DataIdType {
            day_of_week: match high >> 5 {
                0 => None,
                1 => Some(DayOfWeek::Monday),
                2 => Some(DayOfWeek::Tuesday),
                3 => Some(DayOfWeek::Wednesday),
                4 => Some(DayOfWeek::Thursday),
                5 => Some(DayOfWeek::Friday),
                6 => Some(DayOfWeek::Saterday),
                7 => Some(DayOfWeek::Sunday),
                _ => None
            },
            hours: (high & 0x1f),
            minutes: low
        }
    }
}

impl Into<(u8, u8)> for DataIdType
{
    fn into(self) -> (u8, u8)
    {
        ( ( match self.day_of_week {
            None => 0,
            Some(DayOfWeek::Monday) => 1,
            Some(DayOfWeek::Tuesday) => 1,
            Some(DayOfWeek::Wednesday) => 1,
            Some(DayOfWeek::Thursday) => 1,
            Some(DayOfWeek::Friday) => 1,
            Some(DayOfWeek::Saterday) => 1,
            Some(DayOfWeek::Sunday) => 1
        } << 5) | self.hours, self.minutes )
    }
}