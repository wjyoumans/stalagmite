

// By vector we mean a tuple of elements with addition and scalar multiplication.
// There is no distinction between row or column vectors.

use stalagmite_base::integer::ZZElem;
use malachite::base::num::basic::traits::{Zero, One};
use std::fmt;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ZZVector {
    entries: Vec<ZZElem>
}

// Impl Deref but NOT DerefMut.
impl std::ops::Deref for ZZVector {
    type Target = Vec<ZZElem>;

    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl fmt::Display for ZZVector {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut iter = self.iter();
        if let Some(first) = iter.next() {
            write!(f, "({}", first)?;
            for item in iter {
                write!(f, " {}", item)?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl ZZVector {
    #[inline]
    pub fn new<T: Into<ZZVector>>(input: T) -> Self {
        input.into()
    }

    #[inline]
    pub fn from_raw(entries: Vec<ZZElem>) -> Self {
        ZZVector { entries }
    }
    
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        ZZVector::from_raw(Vec::with_capacity(capacity))
    }
    
    #[inline]
    pub fn zeros(len: usize) -> Self {
        ZZVector::from_raw(vec![ZZElem::ZERO; len])
    }

    #[inline]
    pub fn ones(len: usize) -> Self {
        ZZVector::from_raw(vec![ZZElem::ONE; len])
    }
    
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.iter().all(|x| *x == 0)
    }

}

impl<T> From<Vec<T>> for ZZVector
where
    T: Into<ZZElem>
{
    fn from(entries: Vec<T>) -> Self {
        let entries = entries.into_iter().map(|x| x.into()).collect();
        ZZVector::from_raw(entries)
    }
}


