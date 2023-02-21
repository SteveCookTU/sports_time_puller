use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i8)]
pub enum TimeZone {
    #[num_enum(default)]
    Est = -5,
    Cst = -6,
    Mst = -7,
    Pst = -8,
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
