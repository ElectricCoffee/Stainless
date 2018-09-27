// IMPORTANT! Everything general about the library goes here.
//            Everything else goes into the other folders like clean and dirty.

pub mod clean;
pub mod dirty;

// re-exports Clean and Dirty, so users don't have to write `use stainless::taint::clean::Clean`, 
// but instead just `use stainless::taint::Clean` (and similar for Dirty)
pub use self::clean::Clean;
pub use self::dirty::Dirty;

pub trait Sanitizer<T> {
    fn sanitize(input: Dirty<T>) -> Clean<T>;
}