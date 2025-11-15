use std::ops::Sub;

use crate::traits::{CustomWriteSize, ToBits};
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct LikeU12(u16);
impl TryFrom<usize> for LikeU12{
    type Error = std::num::TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let internal = u16::try_from(value)?;
        if internal >= 2_u16.pow(12) {
            return Err(u8::try_from(402).unwrap_err());
        }
        Ok(Self(internal))
    }
}

impl Into<usize> for LikeU12{
    fn into(self) -> usize {
        usize::from(self.0)
    }
}

impl ToBits for LikeU12{
    fn bits_vec(&self) -> Vec<bool> {
        (0..12).into_iter().map(|position| (self.0 >> position) & 1 == 1).collect()
    }
}
impl CustomWriteSize for LikeU12{
    fn custom_size() -> usize {
        return 12;
    }
}

impl From<u8> for LikeU12{
    fn from(value: u8) -> Self {
        Self(u16::from(value))
    }
}
impl min_max_traits::Max for LikeU12{
    const MAX: Self = Self(2_u16.pow(12) - 1);
}
impl std::fmt::Debug for LikeU12{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Sub<LikeU12> for LikeU12{
    type Output = LikeU12;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl TryFrom<&[bool]> for LikeU12{
    type Error = String;

    fn try_from(value: &[bool]) -> Result<Self, Self::Error> {
        if value.len() != Self::custom_size(){
            return Err(format!("Requires {} bits but {} were provided", Self::custom_size(), value.len()));
        }
        let mut internal = 0;
        value.iter().enumerate().for_each(|(i, b)| {
            let flag = 1u16 << i;
            if *b{
                internal |= flag
            }
        });
        Ok(Self(internal))
    }
}