use crate::std::collections::{enumerate::Enumerate, map::Map, vector::Vec};

#[allow(dead_code)]
pub trait Iterator {
    type Item;

    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized;

    fn next(&mut self) -> Option<Self::Item>;

    fn collect<B: Clone + PartialEq + Copy>(self) -> Vec<B>
    where
        Self: Sized,
        B: From<Self::Item> + std::iter::Iterator<Item = Self::Item>;

    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B;

    fn into_iter(self) -> Self;

    fn clone(&self) -> Self;
}

#[allow(dead_code)]
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
