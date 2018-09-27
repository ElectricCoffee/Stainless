use std::ops::{Add};
use super::dirty::Dirty;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Clean<T>(pub T);

impl<T> Clean<T> {
    /// Maps a `Clean<T>` to a `Clean<U>` by applying a function to the contained value.
    /// 
    /// # Examples 
    /// 
    /// Convert a `Clean<i32>` to a `Clean<String>`, consuming the original:
    /// 
    /// ```
    /// extern crate stainless;
    /// use stainless::taint::Clean;
    /// 
    /// let some_clean_i32 = Clean(5);
    /// let some_clean_string = some_clean_i32.map(|i| format!("{} is a fine number", i));
    /// 
    /// assert_eq!(some_clean_string, Clean("5 is a fine number".to_string()));
    /// ```
    pub fn map<U, F>(self, f: F) -> Clean<U> where F: FnOnce(T) -> U {
        let Clean(a) = self;
        Clean(f(a))
    }

    /// Maps a `Clean<T>` to a `Clean<U>` by applying a function that itself returns a `Clean<U>`.
    /// 
    /// This is also called "flatMap", "bind", or ">>=" in other languages.
    /// 
    /// # Examples
    /// 
    /// ```
    /// extern crate stainless;
    /// use stainless::taint::Clean;
    /// 
    /// fn to_clean_string(i: i32) -> Clean<String> { Clean(format!("{} is a fine number", i)) }
    /// 
    /// let result = Clean(28).and_then(to_clean_string);
    /// 
    /// assert_eq!(result, Clean("28 is a fine number".to_string()));
    /// ```
    pub fn and_then<U, F>(self, f: F) -> Clean<U> where F: FnOnce(T) -> Clean<U> {
        let Clean(a) = self;
        f(a)
    }
}

// implements monadic return for Clean
impl<T> From<T> for Clean<T> {
    /// Wraps a value in Clean
    /// 
    /// # Examples
    /// 
    /// ```
    /// extern crate stainless;
    /// use stainless::taint::Clean;
    /// 
    /// let a: Clean<i32> = 42.into();
    /// 
    /// assert_eq!(a, Clean(42));
    /// ```
    fn from(data: T) -> Clean<T> {
        Clean(data)
    }
}

impl<A> Add<Clean<A>> for Clean<A> where A: Add {
    type Output = Clean<A::Output>;
    
    fn add(self, other: Clean<A>) -> Self::Output {
        Clean(Add::add(self.0, other.0))
    }
}

impl<A> Add<Dirty<A>> for Clean<A> where A: Add {
    type Output = Dirty<A::Output>;
    
    fn add(self, other: Dirty<A>) -> Self::Output {
        Dirty(self.0 + other.0)
    }
}