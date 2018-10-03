use stat;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Dyn<A> {
    Clean(A),
    Dirty(A),
}

impl<A> From<stat::Clean<A>> for Dyn<A> {
    /// ```
    /// # use stainless::stat::*;
    /// # use stainless::Dyn;
    /// let a: Clean<i32> = Clean(2);
    /// let b = Dyn::from(a);
    /// assert_eq!(Dyn::Clean(2), b);
    /// ```
    fn from(data: stat::Clean<A>) -> Dyn<A> {
        let stat::Clean(a) = data;
        Dyn::Clean(a)
    }
}

impl<A> From<stat::Dirty<A>> for Dyn<A> {
    /// ```
    /// # use stainless::stat::*;
    /// # use stainless::Dyn;
    /// let a: Dirty<i32> = Dirty(42);
    /// let b = Dyn::from(a);
    /// assert_eq!(Dyn::Dirty(42), b);
    /// ```
    fn from(data: stat::Dirty<A>) -> Dyn<A> {
        let stat::Dirty(a) = data;
        Dyn::Dirty(a)
    }
}