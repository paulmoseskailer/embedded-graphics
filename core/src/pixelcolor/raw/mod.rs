//! Raw color types.
//!
//! This module contains structs to represent the raw data used to store color
//! information. Colors that implement the [`PixelColor`] trait can use the
//! associated [`Raw`] type to define their raw data representation.
//!
//! Specifying a [`Raw`] type for a [`PixelColor`] is required to use that color
//! with the [`Image`] struct.
//!
//! # Converting colors to raw data
//!
//! Colors can be converted into raw data by using two different methods. The [`into_storage`]
//! method is used to convert a color into a single integer value. To convert a color into a byte
//! array the methods provided by the [`ToBytes`] trait can be used. By using [`to_be_bytes`] the
//! color components will have the same order in memory as in the name of the type.
//!
//! ```
//! use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
//!
//! let color = Rgb888::new(0x11, 0x22, 0x33);
//!
//! assert_eq!(color.into_storage(), 0x00112233);
//!
//! assert_eq!(color.to_be_bytes(), [0x11, 0x22, 0x33]);
//! ```
//!
//! # Implementing PixelColor with Raw support
//!
//! This example shows how to implement a new [`PixelColor`] that can be used
//! with images.
//!
//! The RGBI color type uses 4 bits per pixel, one for each color channel and
//! an additional intensity bit.
//!
//! ```rust
//! use embedded_graphics::{image::ImageRaw, pixelcolor::raw::RawU4, prelude::*};
//!
//! /// RGBI color
//! #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
//! pub struct RGBI(RawU4);
//!
//! impl RGBI {
//!     /// Creates a RGBI color.
//!     pub fn new(red: bool, green: bool, blue: bool, intensity: bool) -> Self {
//!         let mut value = 0;
//!
//!         if red {
//!             value |= 0b0100;
//!         }
//!         if green {
//!             value |= 0b0010;
//!         }
//!         if blue {
//!             value |= 0b0001;
//!         }
//!         if intensity {
//!             value |= 0b1000;
//!         }
//!
//!         Self(RawU4::new(value))
//!     }
//! }
//!
//! /// Implement `PixelColor` to associate a raw data type with the `RGBI` struct.
//! impl PixelColor for RGBI {
//!     type Raw = RawU4;
//! }
//!
//! /// `From<RawU4>` is used by `Image` to construct RGBI colors.
//! impl From<RawU4> for RGBI {
//!     fn from(data: RawU4) -> Self {
//!         Self(data)
//!     }
//! }
//!
//! /// Implement conversion into `RawU4` to make `RGBI` usable in framebuffers.
//! impl From<RGBI> for RawU4 {
//!     fn from(color: RGBI) -> Self {
//!         color.0
//!     }
//! }
//!
//! /// Raw image data with 2 pixels per byte.
//! #[rustfmt::skip]
//! const IMAGE_DATA: &[u8] = &[
//!     0b0001_0010,
//!     0b0100_1111,
//! ];
//!
//! // Create new image with RGBI colors.
//! let image_raw: ImageRaw<RGBI> = ImageRaw::new(IMAGE_DATA, Size::new(2, 2)).unwrap();
//!
//! // In a real application the image could now be drawn to a display:
//! // display.draw(&image);
//! #
//! # use embedded_graphics::{mock_display::MockDisplay, image::Image};
//! #
//! # let mut display = MockDisplay::new();
//! # Image::new(&image_raw, Point::zero()).draw(&mut display).unwrap();
//! #
//! # let expected_pixels = [
//! #     Pixel(Point::new(0, 0), RGBI::new(false, false, true, false)),
//! #     Pixel(Point::new(1, 0), RGBI::new(false, true, false, false)),
//! #     Pixel(Point::new(0, 1), RGBI::new(true, false, false, false)),
//! #     Pixel(Point::new(1, 1), RGBI::new(true, true, true, true)),
//! # ];
//! #
//! # let mut expected_display = MockDisplay::new();
//! # expected_pixels.iter().copied().draw(&mut expected_display).unwrap();
//! #
//! # // assert_eq can't be used because ColorMapping isn't implemented for RGBI
//! # assert!(display.eq(&expected_display));
//! ```
//!
//! [`PixelColor`]: super::PixelColor
//! [`Raw`]: super::PixelColor::Raw
//! [`Image`]: https://docs.rs/embedded-graphics/latest/embedded_graphics/image/struct.Image.html
//! [`into_storage`]: super::IntoStorage::into_storage
//! [`to_be_bytes`]: ToBytes::to_be_bytes

mod load_store;
mod to_bytes;

pub use to_bytes::ToBytes;

/// Out of bounds error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutOfBoundsError;

/// Trait implemented by all `RawUx` types.
pub trait RawData: Sized + private::Sealed + From<<Self as RawData>::Storage> + ToBytes {
    /// Storage type.
    ///
    /// A primitive unsigned integer storage type that contains at least `BITS_PER_PIXEL` bits.
    type Storage;

    /// Bits per pixel.
    const BITS_PER_PIXEL: usize;

    /// Bit mask.
    const MASK: Self::Storage;

    /// Converts this raw data into the storage type.
    ///
    /// If the primitive integer types used as the storage type contains more bits
    /// than used by this type the unused most significant bits are set to `0`.
    fn into_inner(self) -> Self::Storage;

    /// Converts a `u32` into a `RawData` type.
    ///
    /// This method can be used to generically construct all `RawData` types from
    /// the same integer type. If the width of the `RawData` type is less than
    /// 32 bits only the least significant bits are used.
    fn from_u32(value: u32) -> Self;

    /// Loads raw data from a buffer.
    ///
    /// Returns `None` if the index is out of bounds.
    fn load<O: DataOrder>(buffer: &[u8], index: usize) -> Option<Self>;

    /// Stores raw data into a buffer.
    ///
    /// Returns an error if the index is out of bounds.
    fn store<O: DataOrder>(self, buffer: &mut [u8], index: usize) -> Result<(), OutOfBoundsError>;
}

impl private::Sealed for () {}

macro_rules! impl_raw_data {
    ($type:ident : $storage_type:ident, $bpp:expr, $bpp_str:expr, $doc:expr) => {
        #[doc = $bpp_str]
        #[doc = "per pixel raw data."]
        #[doc = ""]
        #[doc = $doc]
        #[doc = ""]
        #[doc = "See the [module-level documentation](super) for more information."]
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
        #[cfg_attr(feature = "defmt", derive(::defmt::Format))]
        pub struct $type($storage_type);

        impl $type {
            /// Creates a new color from the least significant
            #[doc = $bpp_str]
            /// of value.
            #[inline]
            pub const fn new(value: $storage_type) -> Self {
                $type(value & <Self as RawData>::MASK)
            }

            #[inline]
            #[allow(unused)]
            pub(crate) const fn new_unmasked(value: $storage_type) -> Self {
                Self(value)
            }
        }

        impl RawData for $type {
            type Storage = $storage_type;

            const BITS_PER_PIXEL: usize = $bpp;
            const MASK: Self::Storage = Self::Storage::MAX >> (Self::Storage::BITS - $bpp);

            fn into_inner(self) -> Self::Storage {
                self.0
            }

            fn from_u32(value: u32) -> Self {
                #[allow(trivial_numeric_casts)]
                Self::new(value as $storage_type)
            }

            fn load<O: DataOrder>(buffer: &[u8], index: usize) -> Option<Self> {
                load_store::LoadStore::<O>::load(buffer, index)

            }

            fn store<O: DataOrder>(self, buffer: &mut [u8], index: usize) -> Result<(), OutOfBoundsError> {
                load_store::LoadStore::<O>::store(self, buffer, index)
            }
        }

        impl From<$storage_type> for $type {
            #[inline]
            fn from(value: $storage_type) -> Self {
                Self::new(value)
            }
        }

        impl private::Sealed for $type {}
    };
    ($type:ident : $storage_type:ident, $bpp:expr, $bpp_str:expr) => {
        impl_raw_data!(
            $type: $storage_type,
            $bpp,
            $bpp_str,
            concat!(
                "`",
                stringify!($type),
                "` is internally stored in an `",
                stringify!($storage_type),
                "`. It can be constructed from an [`",
                stringify!($storage_type),
                "`] by using the ",
                "[`new`](Self::new) method or by calling `",
                stringify!($type),
                "::from(",
                stringify!($storage_type),
                "_value)`. ",
                "To convert a `",
                stringify!($type),
                "` back into a [`",
                stringify!($storage_type),
                "`] the [`into_inner`](Self::into_inner) method can be used."
            )
        );
    };
}

impl_raw_data!(RawU1: u8, 1, "1 bit");
impl_raw_data!(RawU2: u8, 2, "2 bits");
impl_raw_data!(RawU4: u8, 4, "4 bits");
impl_raw_data!(RawU8: u8, 8, "8 bits");
impl_raw_data!(RawU16: u16, 16, "16 bits");
impl_raw_data!(RawU24: u32, 24, "24 bits");
impl_raw_data!(RawU32: u32, 32, "32 bits");

/// Little endian or most significant bits first data order.
///
/// For buffers where the pixel bit depth is a multiple of 8, the pixel data is
/// stored in least significant byte first order. For other bit depths, the
/// pixel data is packed into bytes from left to right with the most significant
/// bits used first within each byte.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum LittleEndianMsb0 {}

/// Big endian or least significant bits first data order.
///
/// For buffers where the pixel bit depth is a multiple of 8, the pixel data is
/// stored in most significant byte first order. For other bit depths, the pixel
/// data is packed into bytes from right to left with the least significant bits
/// used first within each byte.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum BigEndianLsb0 {}

/// Raw data order.
pub trait DataOrder: private::Sealed {
    /// Alternate order.
    ///
    /// Internal use constant to simulate a type level enum.
    const IS_ALTERNATE_ORDER: bool;
}
impl DataOrder for LittleEndianMsb0 {
    const IS_ALTERNATE_ORDER: bool = false;
}
impl DataOrder for BigEndianLsb0 {
    const IS_ALTERNATE_ORDER: bool = true;
}

mod private {
    /// Sealed trait to prevent implementation of traits in other crates.
    pub trait Sealed {}
}

impl private::Sealed for LittleEndianMsb0 {}
impl private::Sealed for BigEndianLsb0 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upper_bits_are_masked() {
        assert_eq!(RawU1::new(u8::max_value()).0, 0x1);
        assert_eq!(RawU2::new(u8::max_value()).0, 0x3);
        assert_eq!(RawU4::new(u8::max_value()).0, 0xF);
        assert_eq!(RawU24::new(u32::max_value()).0, 0xFFFFFF);
    }
}
