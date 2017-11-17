//! Hash infrastructure for items in Merkle Tree.
//!
//! - TODO replace AsBytes() with AsRef<[u8]>?
//! - TODO alg hash(&[u8])
//! - TODO implement #[derive(Hashable<X>)] with [`proc_macro_derive`] as in https://github.com/paritytech/parity-bitcoin/blob/88fdfb3c085ddd2449bde89e4072fcf9f67de0b5/serialization_derive/src/lib.rs

use std::hash::Hasher;

/// A hashable type.
///
/// Types implementing `Hashable` are able to be [`hash`]ed with an instance of
/// [`Hasher`].
///
/// ## Implementing `Hashable`
///
/// You can derive `Hashable` with `#[derive(Hashable)]` if all fields implement `Hashable`.
/// The resulting hash will be the combination of the values from calling
/// [`hash`] on each field.
///
/// ```
/// #[derive(Hashable)]
/// struct Rustacean {
///     name: String,
///     country: String,
/// }
/// ```
///
/// If you need more control over how a value is hashed, you can of course
/// implement the `Hashable` trait yourself:
///
/// ```
/// use merkle::hash::Hashable;
///
/// struct Person {
///     id: u32,
///     name: String,
///     phone: u64,
/// }
///
/// /// where SHA256 : std::hash::Hasher
/// impl Hashable<SHA256> for Person {
///     fn hash(&self, state: &mut SHA256) {
///         self.id.hash(state);
///         self.phone.hash(state);
///     }
/// }
/// ```
///
/// ## `Hashable` and `Eq`
///
/// When implementing both `Hashable` and [`Eq`], it is important that the following
/// property holds:
///
/// ```text
/// k1 == k2 -> hash(k1) == hash(k2)
/// ```
///
/// In other words, if two keys are equal, their hashes must also be equal.
pub trait Hashable<H: Hasher> {
    /// Feeds this value into the given [`Hasher`].
    ///
    /// [`Hasher`]: trait.Hasher.html
    fn hash(&self, state: &mut H);

    /// Feeds a slice of this type into the given [`Hasher`].
    ///
    /// [`Hasher`]: trait.Hasher.html
    fn hash_slice(data: &[Self], state: &mut H)
        where Self: Sized
    {
        for piece in data {
            piece.hash(state);
        }
    }
}

/// Hashing algorithm type.
///
/// Algorithm conforms standard [`Hasher`] trait and provides methods to return
/// full length hash and reset current state.
pub trait Algorithm<T> : Hasher
    where T: AsBytes+Sized+Ord+Clone {

    /// Returns the hash value for the data stream written so far.
    fn hash(&self) -> T;

    /// Reset Hasher state.
    fn reset(&mut self);
}

/// Bytes interface to the hash item.
///
/// Exists to the lack of [`const_generics`] and _unsafe_ [`FixedSizeArray`],
/// [`Unsize`].
///
/// It may be removed in place of AsRef<[u8]>
pub trait AsBytes {
    /// Represent a hash item as a sequence of bytes.
    fn as_bytes(&self) -> &[u8];
}
