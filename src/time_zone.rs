use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i8)]
pub enum TimeZone {
    #[num_enum(default)]
    Est = -4,
    Cst = -5,
    Mst = -6,
    Pst = -7,
}

impl TimeZone {
    pub fn region(&self) -> &'static str {
        match self {
            TimeZone::Est => "EST",
            TimeZone::Cst => "CST",
            TimeZone::Mst => "MST",
            TimeZone::Pst => "PST",
        }
    }
}
