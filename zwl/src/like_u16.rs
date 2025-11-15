use std::ops::Sub;

use crate::traits::{CustomWriteSize, TrailingOnesR, LeadingZerosR, RequiredBits, ToBits};
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct LikeU16(pub u16);
impl TryFrom<usize> for LikeU16{
    type Error = std::num::TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let internal = u16::try_from(value)?;
        Ok(Self(internal))
    }
}

impl Into<usize> for LikeU16{
    fn into(self) -> usize {
        usize::from(self.0)
    }
}

impl ToBits for LikeU16{
    fn bits_vec(&self) -> Vec<bool> {
        let v: Vec<_> = (0..16).into_iter().map(|position| (self.0 >> position) & 1 == 1).collect();
        //v;
        v[0..self.required_bits()].to_vec()
    }
}
impl CustomWriteSize for LikeU16{
    fn custom_size() -> usize {
        return 16;
    }
}

impl From<u8> for LikeU16{
    fn from(value: u8) -> Self {
        Self(u16::from(value))
    }
}
impl min_max_traits::Max for LikeU16{
    const MAX: Self = Self(u16::MAX);
}
impl std::fmt::Debug for LikeU16{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Sub<LikeU16> for LikeU16{
    type Output = LikeU16;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl TryFrom<&[bool]> for LikeU16{
    type Error = String;

    fn try_from(value: &[bool]) -> Result<Self, Self::Error> {
        if value.len() > Self::custom_size(){
            return Err(format!("Requires {} bits but {} were provided", Self::custom_size(), value.len()));
        }
        let mut internal = 0;
        // let offest = Self::custom_size() - value.len();
        value.iter().enumerate().for_each(|(i, b)| {
            // let flag = 1u16 << (offest + i);
            let flag = 1u16 << (i);
            if *b{
                internal |= flag
            }
        });
        Ok(Self(internal))
    }
}

impl RequiredBits for LikeU16{
    fn required_bits(&self) -> usize{
        Self::custom_size() - self.0.leading_zeros() as usize
    }
}
impl LeadingZerosR for LikeU16{
    fn leading_zeros(&self) -> usize {
        self.0.leading_zeros().try_into().unwrap()
    }
}


impl TrailingOnesR for LikeU16{
    fn trailing_ones(&self) -> usize {
        self.0.trailing_ones().try_into().unwrap()
    }
}