// IMPORTANT! Everything general about the library goes here.
//            Everything else goes into the other folders like clean and dirty.

//! The taintedness values.
//! 
//! The types [`Clean`] and [`Dirty`] represent two different levels of taintedness within a system.
//! 
//! Data can either be [`Clean`] if it is known to be safe, or [`Dirty`] if it comes from the outside world.
//! 
//! [`Clean`]: struct.Clean.html
//! [`Dirty`]: struct.Dirty.html

mod clean;
mod dirty;

// re-exports Clean and Dirty, so users don't have to write `use stainless::taint::clean::Clean`, 
// but instead just `use stainless::taint::Clean` (and similar for Dirty)
pub use self::clean::Clean;
pub use self::dirty::Dirty;

pub trait Sanitizer<T> {
    fn sanitize(input: Dirty<T>) -> Clean<T>;
}