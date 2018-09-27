use std::ops::{Add};

pub trait Sanitizer<T> {
    fn sanitize(input: Dirty<T>) -> Clean<T>;
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Clean<T>(pub T);
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Dirty<T>(pub T);

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

impl<T> Dirty<T> {
    /// Maps a `Dirty<T>` to a `Dirty<U>` by applying a function to the contained value.
    /// 
    /// # Examples 
    /// 
    /// Convert a `Dirty<i32>` to a `Dirty<String>`, consuming the original:
    /// 
    /// ```
    /// extern crate stainless;
    /// use stainless::taint::Dirty;
    /// 
    /// let dirty_i32 = Dirty(5);
    /// let result = dirty_i32.map(|i| format!("{} is not a good number", i));
    /// 
    /// assert_eq!(result, Dirty("5 is not a good number".to_string()));
    /// ```
    pub fn map<U, F>(self, f: F) -> Dirty<U> where F: FnOnce(T) -> U {
        let Dirty(a) = self;
        Dirty(f(a))
    }

    /// Maps a `Dirty<T>` to a `Dirty<U>` by applying a function that itself returns a `Dirty<U>`.
    /// 
    /// This is also called "flatMap", "bind", or ">>=" in other languages.
    /// 
    /// # Examples
    /// 
    /// ```
    /// extern crate stainless;
    /// use stainless::taint::Dirty;
    /// 
    /// fn to_dirty_string(i: i32) -> Dirty<String> { Dirty(format!("{} is a fine number", i)) }
    /// 
    /// let result = Dirty(28).and_then(to_dirty_string);
    /// 
    /// assert_eq!(result, Dirty("28 is a fine number".to_string()));
    /// ```
    pub fn and_then<U, F>(self, f: F) -> Dirty<U> where F: FnOnce(T) -> Dirty<U> {
        let Dirty(a) = self;
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

// implements monadic return for Dirty
impl<T> From<T> for Dirty<T> {
    /// Wraps a value in Dirty
    /// 
    /// # Examples
    /// 
    /// ```
    /// extern crate stainless;
    /// use stainless::taint::Dirty;
    /// 
    /// let a: Dirty<char> = 'A'.into();
    /// 
    /// assert_eq!(a, Dirty('A'));
    /// ```
    fn from(data: T) -> Dirty<T> {
        Dirty(data)
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

impl<A> Add<Clean<A>> for Dirty<A> where A: Add {
    type Output = Dirty<A::Output>;
    
    fn add(self, other: Clean<A>) -> Self::Output {
        Dirty(self.0 + other.0)
    }
}

impl<A> Add<Dirty<A>> for Dirty<A> where A: Add {
    type Output = Dirty<A::Output>;
    
    fn add(self, other: Dirty<A>) -> Self::Output {
        Dirty(self.0 + other.0)
    }
}