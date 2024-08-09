use core::{
    borrow::Borrow,
    fmt, hash,
    str::{from_utf8_unchecked, from_utf8_unchecked_mut},
};
use std::hint::unreachable_unchecked;

mod owned;
pub use owned::*;

/// DNS label
#[repr(transparent)]
pub struct Label {
    bytes: [u8],
}

impl Label {
    pub const MAX_LEN: usize = 63;

    #[inline]
    #[must_use]
    pub const fn try_scan_bytes<'a>(
        bytes: &'a [u8],
        scan_root: bool,
        same_length: bool,
    ) -> Result<(&'a Label, &'a [u8]), LabelError> {
        match scan(bytes, scan_root, same_length) {
            Ok(len) => {
                let (label, rest) = unsafe { bytes.split_at_unchecked(len) };
                let label = unsafe { &*(label as *const [u8] as *const Label) };

                Ok((label, rest))
            }
            Err(err) => Err(err),
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn scan_bytes<'a>(
        bytes: &'a [u8],
        scan_root: bool,
        same_length: bool,
    ) -> (&'a Label, &'a [u8]) {
        match Label::try_scan_bytes(bytes, scan_root, same_length) {
            Ok(label) => label,
            Err(err) => panic!("{}", err.message()),
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub const unsafe fn scan_bytes_unchecked<'a>(
        bytes: &'a [u8],
        scan_root: bool,
        same_length: bool,
    ) -> (&'a Label, &'a [u8]) {
        match Label::try_scan_bytes(bytes, scan_root, same_length) {
            Ok(label) => label,
            Err(err) if cfg!(debug_assertions) => panic!("{}", err.message()),
            Err(_) => unsafe { unreachable_unchecked() },
        }
    }

    #[inline]
    #[must_use]
    pub const fn try_from_bytes<'a>(bytes: &'a [u8]) -> Result<&'a Label, LabelError> {
        match Label::try_scan_bytes(bytes, true, true) {
            Ok((label, _)) => Ok(label),
            Err(err) => Err(err),
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn from_bytes<'a>(bytes: &'a [u8]) -> &'a Label {
        match Label::try_from_bytes(bytes) {
            Ok(label) => label,
            Err(err) => panic!("{}", err.message()),
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub const unsafe fn from_bytes_unchecked<'a>(bytes: &'a [u8]) -> &'a Label {
        match Label::try_from_bytes(bytes) {
            Ok(label) => label,
            Err(err) if cfg!(debug_assertions) => panic!("{}", err.message()),
            Err(_) => unsafe { unreachable_unchecked() },
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub const unsafe fn transmute_bytes<'a>(bytes: &'a [u8]) -> &'a Label {
        if cfg!(debug_assertions) {
            unsafe { Label::from_bytes_unchecked(bytes) }
        } else {
            unsafe { &*(bytes as *const [u8] as *const Label) }
        }
    }

    #[inline]
    #[must_use]
    pub fn try_scan_bytes_mut<'a>(
        bytes: &'a mut [u8],
        scan_root: bool,
        same_length: bool,
    ) -> Result<(&'a mut Label, &'a mut [u8]), LabelError> {
        match scan(bytes, scan_root, same_length) {
            Ok(len) => {
                let (label, rest) = unsafe { bytes.split_at_mut_unchecked(len) };
                let label = unsafe { &mut *(label as *mut [u8] as *mut Label) };

                Ok((label, rest))
            }
            Err(err) => Err(err),
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn scan_bytes_mut<'a>(
        bytes: &'a mut [u8],
        scan_root: bool,
        same_length: bool,
    ) -> (&'a mut Label, &'a mut [u8]) {
        match Label::try_scan_bytes_mut(bytes, scan_root, same_length) {
            Ok(label) => label,
            Err(err) => panic!("{}", err.message()),
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub unsafe fn scan_bytes_unchecked_mut<'a>(
        bytes: &'a mut [u8],
        scan_root: bool,
        same_length: bool,
    ) -> (&'a mut Label, &'a mut [u8]) {
        match Label::try_scan_bytes_mut(bytes, scan_root, same_length) {
            Ok(label) => label,
            Err(err) if cfg!(debug_assertions) => panic!("{}", err.message()),
            Err(_) => unsafe { unreachable_unchecked() },
        }
    }

    #[inline]
    #[must_use]
    pub fn try_from_bytes_mut<'a>(bytes: &'a mut [u8]) -> Result<&'a mut Label, LabelError> {
        match Label::try_scan_bytes_mut(bytes, true, true) {
            Ok((label, _)) => Ok(label),
            Err(err) => Err(err),
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_bytes_mut<'a>(bytes: &'a mut [u8]) -> &'a mut Label {
        match Label::try_from_bytes_mut(bytes) {
            Ok(label) => label,
            Err(err) => panic!("{}", err.message()),
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub unsafe fn from_bytes_unchecked_mut<'a>(bytes: &'a mut [u8]) -> &'a mut Label {
        match Label::try_from_bytes_mut(bytes) {
            Ok(label) => label,
            Err(err) if cfg!(debug_assertions) => panic!("{}", err.message()),
            Err(_) => unsafe { unreachable_unchecked() },
        }
    }

    #[inline]
    #[must_use]
    #[track_caller]
    pub unsafe fn transmute_bytes_mut<'a>(bytes: &'a mut [u8]) -> &'a mut Label {
        if cfg!(debug_assertions) {
            unsafe { Label::from_bytes_unchecked_mut(bytes) }
        } else {
            unsafe { &mut *(bytes as *mut [u8] as *mut Label) }
        }
    }

    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.bytes.len()
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    #[inline]
    #[must_use]
    pub const fn is_root(&self) -> bool {
        self.is_empty()
    }

    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[inline]
    #[must_use]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { &mut self.bytes }
    }

    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &str {
        unsafe { from_utf8_unchecked(self.as_bytes()) }
    }

    #[inline]
    #[must_use]
    pub unsafe fn as_str_mut(&mut self) -> &mut str {
        unsafe { from_utf8_unchecked_mut(self.as_bytes_mut()) }
    }

    #[inline]
    #[must_use]
    pub fn make_ascii_lowercase(&mut self) {
        self.bytes.make_ascii_lowercase();
    }

    #[inline]
    #[must_use]
    pub fn make_ascii_uppercase(&mut self) {
        self.bytes.make_ascii_uppercase();
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl fmt::Debug for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl AsRef<[u8]> for Label {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<str> for Label {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<[u8]> for Label {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Borrow<str> for Label {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<'a> From<&'a Label> for &'a [u8] {
    #[inline]
    fn from(value: &'a Label) -> Self {
        value.as_bytes()
    }
}

impl<'a> From<&'a Label> for &'a str {
    #[inline]
    fn from(value: &'a Label) -> Self {
        value.as_str()
    }
}

impl<'a> TryFrom<&'a [u8]> for &'a Label {
    type Error = LabelError;

    #[inline]
    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Label::try_from_bytes(value)
    }
}

impl<'a> TryFrom<&'a mut [u8]> for &'a mut Label {
    type Error = LabelError;

    #[inline]
    fn try_from(value: &'a mut [u8]) -> Result<Self, Self::Error> {
        Label::try_from_bytes_mut(value)
    }
}

impl<'a> TryFrom<&'a str> for &'a Label {
    type Error = LabelError;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Label::try_from_bytes(value.as_bytes())
    }
}

impl<'a> TryFrom<&'a mut str> for &'a mut Label {
    type Error = LabelError;

    #[inline]
    fn try_from(value: &'a mut str) -> Result<Self, Self::Error> {
        // SAFETY: a label is always valid UTF-8.
        unsafe { Label::try_from_bytes_mut(value.as_bytes_mut()) }
    }
}

impl<'a, const N: usize> TryFrom<&'a [u8; N]> for &'a Label {
    type Error = LabelError;

    #[inline]
    fn try_from(value: &'a [u8; N]) -> Result<Self, Self::Error> {
        Label::try_from_bytes(value)
    }
}

impl<'a, const N: usize> TryFrom<&'a mut [u8; N]> for &'a mut Label {
    type Error = LabelError;

    #[inline]
    fn try_from(value: &'a mut [u8; N]) -> Result<Self, Self::Error> {
        Label::try_from_bytes_mut(value)
    }
}

impl PartialEq for Label {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes().eq(other.as_bytes())
    }
}

impl Eq for Label {}

impl PartialOrd for Label {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Label {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let this = self.as_bytes().iter().map(u8::to_ascii_lowercase);
        let other = other.as_bytes().iter().map(u8::to_ascii_lowercase);

        this.cmp(other)
    }
}

impl hash::Hash for Label {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        for b in self.as_bytes() {
            state.write_u8(b.to_ascii_lowercase());
        }
    }
}

/// Error type which is returned when scanning a [`Label`] fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LabelError {
    /// Labels cannot have a hypen at their start or end.
    StrayHyphen,

    /// The length of the scanned label differs from its source.
    LengthMismatch,

    /// Scanned the root label unexpectedly.
    FoundRoot,
}

impl LabelError {
    /// Get the error message.
    #[inline]
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            LabelError::StrayHyphen => "labels cannot have a hypen at their start or end",
            LabelError::LengthMismatch => "length of the scanned label differs from its source",
            LabelError::FoundRoot => "scanned the root label unexpectedly",
        }
    }
}

impl fmt::Display for LabelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LabelError {}

/// Scans a DNS name label, returning how many bytes were matched.
///
/// # Safety
///
/// You can safely create a label from `&bytes[..n]`,
/// where `n` is the returned amount.
#[inline]
#[must_use]
const fn scan(bytes: &[u8], scan_root: bool, same_length: bool) -> Result<usize, LabelError> {
    match scan_inner(bytes) {
        Some(len) => {
            if (len == 0) & !scan_root {
                Err(LabelError::FoundRoot)
            } else if (len != bytes.len()) & same_length {
                Err(LabelError::LengthMismatch)
            } else {
                Ok(len)
            }
        }
        None => Err(LabelError::StrayHyphen),
    }
}

/// Returns `None` if there are stray hyphens.
#[inline]
#[must_use]
const fn scan_inner(bytes: &[u8]) -> Option<usize> {
    let mut len = if bytes.len() > Label::MAX_LEN {
        Label::MAX_LEN
    } else {
        bytes.len()
    };
    let mut offset = 0usize;

    while offset < len {
        let octet = unsafe { bytes.as_ptr().add(offset).read() };

        if octet.is_ascii_alphanumeric() | (octet == b'-') {
            offset += 1;
        } else {
            len = 0;
        }
    }

    if offset != 0 {
        // SAFETY: If offset is not zero then `bytes.len()` is not zero.

        let first = unsafe { bytes.as_ptr().read() };
        let last = unsafe { bytes.as_ptr().add(offset - 1).read() };

        if (first == b'-') | (last == b'-') {
            return None;
        }
    }

    Some(offset)
}
