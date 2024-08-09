use core::{
    borrow::BorrowMut,
    slice::{from_raw_parts, from_raw_parts_mut},
};
use std::ops::{Deref, DerefMut};

use super::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct OwnedLabel {
    len: u8,
    bytes: [u8; 63],
}

impl OwnedLabel {
    #[inline]
    #[must_use]
    #[no_mangle]
    pub const fn from_label(label: &Label) -> OwnedLabel {
        // Trickery to ensure remove bounds checks.
        if label.len() > 63 {
            // SAFETY: the length of a label is always less than 64.
            unsafe { unreachable_unchecked() }
        }

        let mut bytes = [0u8; 63];
        let mut offset = 0usize;

        while offset < label.len() {
            bytes[offset] = label.as_bytes()[offset];

            offset += 1;
        }

        OwnedLabel {
            len: label.len() as u8,
            bytes,
        }
    }

    #[inline]
    #[must_use]
    pub const fn as_label(&self) -> &Label {
        let label = unsafe { from_raw_parts(self.bytes.as_ptr(), self.len as usize) };

        unsafe { Label::transmute_bytes(label) }
    }

    #[inline]
    #[must_use]
    pub fn as_label_mut(&mut self) -> &mut Label {
        let label = unsafe { from_raw_parts_mut(self.bytes.as_mut_ptr(), self.len as usize) };

        unsafe { Label::transmute_bytes_mut(label) }
    }
}

impl Deref for OwnedLabel {
    type Target = Label;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_label()
    }
}

impl DerefMut for OwnedLabel {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_label_mut()
    }
}

#[cfg(feature = "alloc")]
impl alloc::borrow::ToOwned for Label {
    type Owned = OwnedLabel;

    #[inline]
    fn to_owned(&self) -> Self::Owned {
        OwnedLabel::from_label(self)
    }
}

impl<T: ?Sized> Borrow<T> for OwnedLabel
where
    Label: Borrow<T>,
{
    #[inline]
    fn borrow(&self) -> &T {
        self.as_label().borrow()
    }
}

impl<T: ?Sized> BorrowMut<T> for OwnedLabel
where
    Label: BorrowMut<T>,
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self.as_label_mut().borrow_mut()
    }
}
