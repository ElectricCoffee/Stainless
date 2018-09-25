use std::ops::{Add};
use std::result;
use std::convert::Into;

pub type Result<T> = result::Result<Clean<T>, Dirty<T>>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Clean<T>(pub T);
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Dirty<T>(pub T);

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