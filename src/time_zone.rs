use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i8)]
pub enum TimeZone {
    #[num_enum(default)]
    Edt = -4,
    Cdt = -5,
    Mdt = -6,
    Pdt = -7,
}

impl TimeZone {
    pub fn region(&self) -> &'static str {
        match self {
            TimeZone::Edt => "EST",
            TimeZone::Cdt => "CST",
            TimeZone::Mdt => "MST",
            TimeZone::Pdt => "PST",
        }
    }
}
