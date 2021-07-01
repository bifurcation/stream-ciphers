//! 16-bit counter falvors.
use super::CtrFlavor;
use cipher::generic_array::{
    typenum::{U16, U8},
    GenericArray,
};
use core::convert::TryInto;

/// 16-bit big endian counter flavor.
#[derive(Default, Copy, Clone)]
#[repr(transparent)]
pub struct Ctr16BE(u16);

impl CtrFlavor for Ctr16BE {
    type Size = U8;
    type Backend = u16;

    #[inline]
    fn generate_block(&self, nonce: &GenericArray<Self, Self::Size>) -> GenericArray<u8, U16> {
        let mut res = GenericArray::<u8, U16>::default();
        let ctr = self.0.wrapping_add(nonce[3].0);
        res[0..2].copy_from_slice(&nonce[0].0.to_ne_bytes());
        res[2..4].copy_from_slice(&nonce[1].0.to_ne_bytes());
        res[4..6].copy_from_slice(&nonce[2].0.to_ne_bytes());
        res[6..8].copy_from_slice(&nonce[3].0.to_ne_bytes());
        res[8..10].copy_from_slice(&nonce[4].0.to_ne_bytes());
        res[10..12].copy_from_slice(&nonce[5].0.to_ne_bytes());
        res[12..14].copy_from_slice(&nonce[6].0.to_ne_bytes());
        res[14..16].copy_from_slice(&ctr.to_be_bytes());
        res
    }

    #[inline]
    fn load(block: &GenericArray<u8, U16>) -> GenericArray<Self, Self::Size> {
        [
            Self(u16::from_ne_bytes(block[0..2].try_into().unwrap())),
            Self(u16::from_ne_bytes(block[2..4].try_into().unwrap())),
            Self(u16::from_ne_bytes(block[4..6].try_into().unwrap())),
            Self(u16::from_ne_bytes(block[6..8].try_into().unwrap())),
            Self(u16::from_ne_bytes(block[8..10].try_into().unwrap())),
            Self(u16::from_ne_bytes(block[10..12].try_into().unwrap())),
            Self(u16::from_ne_bytes(block[12..14].try_into().unwrap())),
            Self(u16::from_be_bytes(block[14..16].try_into().unwrap())),
        ]
        .into()
    }

    #[inline]
    fn checked_add(&self, rhs: usize) -> Option<Self> {
        rhs.try_into()
            .ok()
            .and_then(|rhs| self.0.checked_add(rhs))
            .map(Self)
    }

    #[inline]
    fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }

    #[inline]
    fn to_backend(&self) -> Self::Backend {
        self.0
    }

    #[inline]
    fn from_backend(v: Self::Backend) -> Self {
        Self(v)
    }
}

/// 16-bit little endian counter flavor.
#[derive(Default, Clone)]
#[repr(transparent)]
pub struct Ctr16LE(u16);

impl CtrFlavor for Ctr16LE {
    type Size = U8;
    type Backend = u16;

    #[inline]
    fn generate_block(&self, nonce: &GenericArray<Self, Self::Size>) -> GenericArray<u8, U16> {
        let mut res = GenericArray::<u8, U16>::default();
        let ctr = self.0.wrapping_add(nonce[0].0);
        res[0..2].copy_from_slice(&ctr.to_le_bytes());
        res[2..4].copy_from_slice(&nonce[1].0.to_ne_bytes());
        res[4..6].copy_from_slice(&nonce[2].0.to_ne_bytes());
        res[6..8].copy_from_slice(&nonce[3].0.to_ne_bytes());
        res[8..10].copy_from_slice(&nonce[4].0.to_ne_bytes());
        res[10..12].copy_from_slice(&nonce[5].0.to_ne_bytes());
        res[12..14].copy_from_slice(&nonce[6].0.to_ne_bytes());
        res[14..16].copy_from_slice(&nonce[7].0.to_ne_bytes());
        res
    }

    #[inline]
    fn load(block: &GenericArray<u8, U16>) -> GenericArray<Self, Self::Size> {
        [
            Self(u16::from_le_bytes(block[0..2].try_into().unwrap())),
            Self(u16::from_ne_bytes(block[2..4].try_into().unwrap())),
            Self(u16::from_ne_bytes(block[4..6].try_into().unwrap())),
            Self(u16::from_ne_bytes(block[6..8].try_into().unwrap())),
            Self(u16::from_ne_bytes(block[8..10].try_into().unwrap())),
            Self(u16::from_ne_bytes(block[10..12].try_into().unwrap())),
            Self(u16::from_ne_bytes(block[12..14].try_into().unwrap())),
            Self(u16::from_be_bytes(block[14..16].try_into().unwrap())),
        ]
        .into()
    }

    #[inline]
    fn checked_add(&self, rhs: usize) -> Option<Self> {
        rhs.try_into()
            .ok()
            .and_then(|rhs| self.0.checked_add(rhs))
            .map(Self)
    }

    #[inline]
    fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }

    #[inline]
    fn to_backend(&self) -> Self::Backend {
        self.0
    }

    #[inline]
    fn from_backend(v: Self::Backend) -> Self {
        Self(v)
    }
}
