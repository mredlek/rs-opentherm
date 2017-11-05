use ::std::fmt::{Binary, Debug, Formatter};
use ::std::fmt::Result as FmtResult;

/// This struct represent 8 bits where the content is dependend on other information.
/// Usually, this a flags depending on the dataid.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Flags8
{
    /// The field containing the bits
    pub bits: u8
}

impl From<u8> for Flags8
{
    fn from(input: u8) -> Flags8
    {
        Flags8 { bits: input }
    }
}

impl Into<u8> for Flags8
{
    fn into(self) -> u8
    {
        self.bits
    }
}

impl Binary for Flags8 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:b}", self.bits)
    }
}

impl Debug for Flags8 {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:b}", self.bits)
    }
}
