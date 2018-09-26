use std::ops::{Add};
use std::result;
use std::convert::Into;

pub type Result<T> = result::Result<Clean<T>, Dirty<T>>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Clean<T>(pub T);
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Dirty<T>(pub T);

// (a -> b) -> m a -> m b
impl<T> Clean<T> {
    pub fn map<U, F>(self, f: F) -> Clean<U> where F: FnOnce(T) -> U {
        let Clean(a) = self;
        Clean(f(a))
    }

    pub fn and_then<U, F>(self, f: F) -> Clean<U> where F: FnOnce(T) -> Clean<U> {
        let Clean(a) = self;
        f(a)
    }
}

impl<T> Dirty<T> {
    pub fn map<U, F>(self, f: F) -> Dirty<U> where F: FnOnce(T) -> U {
        let Dirty(a) = self;
        Dirty(f(a))
    }

    pub fn and_then<U, F>(self, f: F) -> Dirty<U> where F: FnOnce(T) -> Dirty<U> {
        let Dirty(a) = self;
        f(a)
    }
}

impl<T> Into<Result<T>> for Clean<T> {
    fn into(self) -> Result<T> {
        Ok(self)
    }
}

impl<T> Into<Result<T>> for Dirty<T> {
    fn into(self) -> Result<T> {
        Err(self)
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