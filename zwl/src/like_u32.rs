use std::ops::Sub;

use crate::traits::{CustomWriteSize, TrailingOnesR, LeadingZerosR, RequiredBits, ToBits};
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct LikeU32(pub u32);
impl TryFrom<usize> for LikeU32{
    type Error = std::num::TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let internal = u32::try_from(value)?;
        Ok(Self(internal))
    }
}

impl ToBits for LikeU32{
    fn bits_vec(&self) -> Vec<bool> {
        let v: Vec<_> = (0..32).into_iter().map(|position| (self.0 >> position) & 1 == 1).collect();
        //v;
        v[0..self.required_bits()].to_vec()
    }
}
impl CustomWriteSize for LikeU32{
    fn custom_size() -> usize {
        return 32;
    }
}

impl From<u8> for LikeU32{
    fn from(value: u8) -> Self {
        Self(u32::from(value))
    }
}
impl min_max_traits::Max for LikeU32{
    const MAX: Self = Self(u32::MAX);
}
impl std::fmt::Debug for LikeU32{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Sub<LikeU32> for LikeU32{
    type Output = LikeU32;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl TryFrom<&[bool]> for LikeU32{
    type Error = String;

    fn try_from(value: &[bool]) -> Result<Self, Self::Error> {
        if value.len() > Self::custom_size(){
            return Err(format!("Requires {} bits but {} were provided", Self::custom_size(), value.len()));
        }
        let mut internal = 0;
        // let offest = Self::custom_size() - value.len();
        value.iter().enumerate().for_each(|(i, b)| {
            // let flag = 1u32 << (offest + i);
            let flag = 1u32 << (i);
            if *b{
                internal |= flag
            }
        });
        Ok(Self(internal))
    }
}

impl RequiredBits for LikeU32{
    fn required_bits(&self) -> usize{
        Self::custom_size() - self.0.leading_zeros() as usize
    }
}
impl LeadingZerosR for LikeU32{
    fn leading_zeros(&self) -> usize {
        self.0.leading_zeros().try_into().unwrap()
    }
}


impl TrailingOnesR for LikeU32{
    fn trailing_ones(&self) -> usize {
        self.0.trailing_ones().try_into().unwrap()
    }
}
impl TryInto<usize> for LikeU32{
    type Error = std::num::TryFromIntError;

    fn try_into(self) -> Result<usize, Self::Error> {
        self.0.try_into()
    }
}