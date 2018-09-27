//! # stainless
//! 
//! Stainless is a crate that aims to provide static taint analysis for use in Rust programs, 
//! by providing primitives to work with the notion of taintedness via the two new types [`Clean`] and [`Dirty`].
//! 
//! # Example
//! One could easily imagine a scenario like the following:
//! ```
//! # extern crate stainless;
//! # use stainless::taint::{Clean, Dirty};
//! # fn get_input() -> String { "Corrupted!".into() }
//! # fn db(_: &str) -> Vec<String> { vec!["Boom!".into()] }
//! fn get_users(name: String) -> Vec<String> { 
//!     let query = format!("SELECT * FROM Users WHERE username = '{}';", name);
//!     db(&query)
//! }
//! 
//! let data = get_input(); // reads some input from a website
//! get_users(data);
//! ```
//! And then your entire database vomits out all user info because _someone_ forgot to sanitize the input.
//! 
//! So let's try the same example one more time, this time using stainless.
//! ```compile_fail
//! extern crate stainless;
//! use stainless::taint::{Clean, Dirty};
//! # fn get_input() -> String { "Corrupted!".into() }
//! # fn db(_: &str) -> Vec<String> { vec!["Boom!".into()] }
//! 
//! fn get_users(name: Clean<String>) -> Clean<Vec<String>> { 
//!     name.map(|n| format!("SELECT * FROM Users WHERE username = '{}';", n))
//!         .map(|query| db(&query))
//! }
//! 
//! let data: Dirty<String> = get_input().into(); // input explicitly tagged as `Dirty`
//! get_users(data); // 💥 expected struct `stainless::taint::Clean`, found struct `stainless::taint::Dirty`
//! ```
//! Stainless ensures that you have to sanitize your input before you can go ahead and use it.
//! If not, the compiler will refuse to accept the data. 
//! 
//! [`Clean`]: struct.Clean.html
//! [`Dirty`]: struct.Dirty.html

pub mod taint;

#[cfg(test)]
mod tests {
    use taint::{Clean, Dirty};

    #[test] 
    fn test_add() {
        let a = Clean(2) + Clean(3);
        let b = Clean(2) + Dirty(3);
        let c = Dirty(2) + Clean(3);
        let d = Dirty(2) + Dirty(3);
 
        assert_eq!(a, Clean(5));
        assert_eq!(b, Dirty(5));
        assert_eq!(c, Dirty(5));
        assert_eq!(d, Dirty(5));
    }

    #[test]
    fn test_from() {
        let a: Clean<i32> = 1.into();
        let b: Dirty<i32> = 2.into();

        assert_eq!(a, Clean(1));
        assert_eq!(b, Dirty(2));
    }

    #[test]
    fn test_map() {
        let a = Clean(1).map(|x| x + 3);
        let b = Dirty(3).map(|x| x * 3);
        assert_eq!(a, Clean(4));
        assert_eq!(b, Dirty(9));
    }

    fn double_clean(x: i32) -> Clean<i32> {
        Clean(x * 2)
    }

    fn triple_dirty(x: i32) -> Dirty<i32> {
        Dirty(x * 3)
    }

    #[test]
    fn test_and_then() {
        let a = Clean(8).and_then(double_clean).and_then(double_clean);
        let b = Dirty(2).and_then(triple_dirty).and_then(triple_dirty);
        assert_eq!(a, Clean(32));
        assert_eq!(b, Dirty(18));
    }
}
