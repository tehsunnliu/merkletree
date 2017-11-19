//! Hash infrastructure for items in Merkle Tree.

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
/// #[macro_use]
/// extern crate merkle_light_derive;
/// extern crate merkle_light;
/// use merkle_light::hash::Hashable;
///
/// fn main() {
///     #[derive(Hashable)]
///     struct Foo {
///         name: String,
///         country: String,
///     }
/// }
/// ```
///
/// If you need more control over how a value is hashed, you can of course
/// implement the `Hashable` trait yourself:
///
/// ```
/// extern crate merkle_light;
/// use merkle_light::hash::Hashable;
/// use std::hash::Hasher;
/// use std::collections::hash_map::DefaultHasher;
///
/// fn main() {
///    struct Person {
///        id: u32,
///        name: String,
///        phone: u64,
///    }
///
///    impl<H: Hasher> Hashable<H> for Person {
///        fn hash(&self, state: &mut H) {
///            self.id.hash(state);
///            self.name.hash(state);
///            self.phone.hash(state);
///        }
///    }
///
///    let foo = Person{
///        id: 1,
///        name: String::from("blah"),
///        phone: 2,
///    };
///
///    let hr = &mut DefaultHasher::new();
///    foo.hash(hr);
///    assert_eq!(hr.finish(), 7101638158313343130)
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
    where
        Self: Sized,
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
pub trait Algorithm<T>: Hasher
where
    T: AsRef<[u8]> + Sized + Ord + Clone,
{
    /// Returns the hash value for the data stream written so far.
    fn hash(&self) -> T;

    /// Reset Hasher state.
    fn reset(&mut self);
}