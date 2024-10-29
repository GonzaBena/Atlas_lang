use crate::std::collections::map::Map;

use super::vector::Vec;
use crate::std::interfaces::collections::iterator::{IntoIterator, Iterator};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Enumerate<I> {
    iter: I,
    count: usize,
}

impl<I> Enumerate<I> {
    pub fn new(iter: I) -> Enumerate<I>
    where
        I: Iterator,
    {
        Enumerate { iter, count: 0 }
    }
}

impl<I> Iterator for Enumerate<I>
where
    I: Iterator + Clone + std::iter::Iterator,
    <I as std::iter::Iterator>::Item: Clone + std::convert::From<<I as std::iter::Iterator>::Item>,
{
    type Item = (usize, <I as Iterator>::Item);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }

    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        // self.enumerate()
        todo!()
    }

    fn into_iter(self) -> Self {
        self
    }

    fn collect<B>(self) -> Vec<B>
    where
        Self: Sized,
        B: From<Self::Item>,
        B: Clone + Copy + PartialEq,
    {
        let mut vec: Vec<B> = Vec::new();
        for item in self {
            vec.push(B::from(item));
        }
        vec
    }

    fn clone(&self) -> Self {
        // Enumerate {
        //     iter: self.iter,
        //     count: self.count,
        // }
        todo!()
    }

    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map::new(self, f)
    }
}

impl<I> IntoIterator for Enumerate<I>
where
    I: Iterator + std::iter::Iterator + Clone,
    <I as std::iter::Iterator>::Item: Clone + std::convert::From<<I as std::iter::Iterator>::Item>,
{
    type Item = (usize, <I as Iterator>::Item);
    type IntoIter = Enumerate<I>;

    fn into_iter(self) -> Self::IntoIter {
        self
    }
}

macro_rules! impl_std_iterator {
    (
        $iter_type:ident < $($params:tt),* > $(where $($where_clause:tt)*)?
    ) => {
        impl<$($params),*> std::iter::Iterator for $iter_type<$($params),*>
            $(where $($where_clause)*)?
        {
            type Item = <Self as Iterator>::Item;

            fn next(&mut self) -> Option<Self::Item> {
                Iterator::next(self)
            }
        }
    };
}

impl_std_iterator!(Enumerate<I> where I: Iterator + Clone + std::iter::Iterator, <I as std::iter::Iterator>::Item: Clone);
