// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Traits for conversions between types.
//!
//! The traits in this module provide a general way to talk about conversions
//! from one type to another. They follow the standard Rust conventions of
//! `as`/`into`/`from`.
//!
//! Like many traits, these are often used as bounds for generic functions, to
//! support arguments of multiple types.
//!
//! - Impl the `As*` traits for reference-to-reference conversions
//! - Impl the `Into` trait when you want to consume the value in the conversion
//! - The `From` trait is the most flexible, useful for values _and_ references conversions
//!
//! As a library writer, you should prefer implementing `From<T>` rather than
//! `Into<U>`, as `From` provides greater flexibility and offer the equivalent `Into`
//! implementation for free, thanks to a blanket implementation in the standard library.
//!
//! **Note: these traits must not fail**. If the conversion can fail, you must use a dedicated
//! method which return an `Option<T>` or a `Result<T, E>`.
//!
//! # Generic impl
//!
//! - `AsRef` and `AsMut` auto-dereference if the inner type is a reference
//! - `From<U> for T` implies `Into<T> for U`
//! - `From` and `Into` are reflexive, which means that all types can `into()`
//!   themselves and `from()` themselves
//!
//! See each trait for usage examples.

#![stable(feature = "rust1", since = "1.0.0")]

use marker::Sized;

/// A cheap, reference-to-reference conversion.
///
/// `AsRef` is very similar to, but different than, `Borrow`. See
/// [the book][book] for more.
///
/// [book]: ../../book/borrow-and-asref.html
///
/// **Note: this trait must not fail**. If the conversion can fail, use a dedicated method which
/// return an `Option<T>` or a `Result<T, E>`.
///
/// # Examples
///
/// Both `String` and `&str` implement `AsRef<str>`:
///
/// ```
/// fn is_hello<T: AsRef<str>>(s: T) {
///    assert_eq!("hello", s.as_ref());
/// }
///
/// let s = "hello";
/// is_hello(s);
///
/// let s = "hello".to_string();
/// is_hello(s);
/// ```
///
/// # Generic Impls
///
/// - `AsRef` auto-dereference if the inner type is a reference or a mutable
/// reference (eg: `foo.as_ref()` will work the same if `foo` has type `&mut Foo` or `&&mut Foo`)
///
#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsRef<T: ?Sized> {
    /// Performs the conversion.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_ref(&self) -> &T;
}

/// A cheap, mutable reference-to-mutable reference conversion.
///
/// **Note: this trait must not fail**. If the conversion can fail, use a dedicated method which
/// return an `Option<T>` or a `Result<T, E>`.
///
/// # Generic Impls
///
/// - `AsMut` auto-dereference if the inner type is a reference or a mutable
/// reference (eg: `foo.as_ref()` will work the same if `foo` has type `&mut Foo` or `&&mut Foo`)
///
#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsMut<T: ?Sized> {
    /// Performs the conversion.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_mut(&mut self) -> &mut T;
}

/// A conversion that consumes `self`, which may or may not be expensive.
///
/// **Note: this trait must not fail**. If the conversion can fail, use a dedicated method which
/// return an `Option<T>` or a `Result<T, E>`.
///
/// Library writer should not implement directly this trait, but should prefer the implementation
/// of the `From` trait, which offer greater flexibility and provide the equivalent `Into`
/// implementation for free, thanks to a blanket implementation in the standard library.
///
/// # Examples
///
/// `String` implements `Into<Vec<u8>>`:
///
/// ```
/// fn is_hello<T: Into<Vec<u8>>>(s: T) {
///    let bytes = b"hello".to_vec();
///    assert_eq!(bytes, s.into());
/// }
///
/// let s = "hello".to_string();
/// is_hello(s);
/// ```
///
/// # Generic Impls
///
/// - `From<T> for U` implies `Into<U> for T`
/// - `into()` is reflexive, which means that `Into<T> for T` is implemented
///
#[stable(feature = "rust1", since = "1.0.0")]
pub trait Into<T>: Sized {
    /// Performs the conversion.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn into(self) -> T;
}

/// Construct `Self` via a conversion.
///
/// **Note: this trait must not fail**. If the conversion can fail, use a dedicated method which
/// return an `Option<T>` or a `Result<T, E>`.
///
/// # Examples
///
/// `String` implements `From<&str>`:
///
/// ```
/// let string = "hello".to_string();
/// let other_string = String::from("hello");
///
/// assert_eq!(string, other_string);
/// ```
/// # Generic impls
///
/// - `From<T> for U` implies `Into<U> for T`
/// - `from()` is reflexive, which means that `From<T> for T` is implemented
///
#[stable(feature = "rust1", since = "1.0.0")]
pub trait From<T>: Sized {
    /// Performs the conversion.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn from(T) -> Self;
}

////////////////////////////////////////////////////////////////////////////////
// GENERIC IMPLS
////////////////////////////////////////////////////////////////////////////////

// As lifts over &
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T: ?Sized, U: ?Sized> AsRef<U> for &'a T where T: AsRef<U> {
    fn as_ref(&self) -> &U {
        <T as AsRef<U>>::as_ref(*self)
    }
}

// As lifts over &mut
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T: ?Sized, U: ?Sized> AsRef<U> for &'a mut T where T: AsRef<U> {
    fn as_ref(&self) -> &U {
        <T as AsRef<U>>::as_ref(*self)
    }
}

// FIXME (#23442): replace the above impls for &/&mut with the following more general one:
// // As lifts over Deref
// impl<D: ?Sized + Deref, U: ?Sized> AsRef<U> for D where D::Target: AsRef<U> {
//     fn as_ref(&self) -> &U {
//         self.deref().as_ref()
//     }
// }

// AsMut lifts over &mut
#[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T: ?Sized, U: ?Sized> AsMut<U> for &'a mut T where T: AsMut<U> {
    fn as_mut(&mut self) -> &mut U {
        (*self).as_mut()
    }
}

// FIXME (#23442): replace the above impl for &mut with the following more general one:
// // AsMut lifts over DerefMut
// impl<D: ?Sized + Deref, U: ?Sized> AsMut<U> for D where D::Target: AsMut<U> {
//     fn as_mut(&mut self) -> &mut U {
//         self.deref_mut().as_mut()
//     }
// }

// From implies Into
#[stable(feature = "rust1", since = "1.0.0")]
impl<T, U> Into<U> for T where U: From<T> {
    fn into(self) -> U {
        U::from(self)
    }
}

// From (and thus Into) is reflexive
#[stable(feature = "rust1", since = "1.0.0")]
impl<T> From<T> for T {
    fn from(t: T) -> T { t }
}

////////////////////////////////////////////////////////////////////////////////
// CONCRETE IMPLS
////////////////////////////////////////////////////////////////////////////////

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> AsRef<[T]> for [T] {
    fn as_ref(&self) -> &[T] {
        self
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T> AsMut<[T]> for [T] {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsRef<str> for str {
    #[inline]
    fn as_ref(&self) -> &str {
        self
    }
}
