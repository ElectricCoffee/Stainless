use std::ops::{Add};

pub trait Sanitizer<T> {
    fn sanitize(input: Dirty<T>) -> Clean<T>;
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Clean<T>(pub T);
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Dirty<T>(pub T);

impl<T> Clean<T> {
    // implements the Functor map for Clean
    pub fn map<U, F>(self, f: F) -> Clean<U> where F: FnOnce(T) -> U {
        let Clean(a) = self;
        Clean(f(a))
    }

    // implements monadic bind for Clean
    pub fn and_then<U, F>(self, f: F) -> Clean<U> where F: FnOnce(T) -> Clean<U> {
        let Clean(a) = self;
        f(a)
    }
}

impl<T> Dirty<T> {
    // implements Functor map for Dirty
    pub fn map<U, F>(self, f: F) -> Dirty<U> where F: FnOnce(T) -> U {
        let Dirty(a) = self;
        Dirty(f(a))
    }

    // implements monadic bind for Dirty
    pub fn and_then<U, F>(self, f: F) -> Dirty<U> where F: FnOnce(T) -> Dirty<U> {
        let Dirty(a) = self;
        f(a)
    }
}

// implements monadic return for Clean
impl<T> From<T> for Clean<T> {
    fn from(data: T) -> Clean<T> {
        Clean(data)
    }
}

// implements monadic return for Dirty
impl<T> From<T> for Dirty<T> {
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