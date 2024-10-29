use super::array::Array;
use super::map::Map;
use super::vector::Vec;

use crate::std::collections::enumerate::Enumerate;

use crate::std::interfaces::collections::iterator;
use crate::std::types::char::Char;

#[derive(Debug, Clone)]
pub struct Iter<'a, T: 'a>
where
    T: Default + PartialEq + Clone + Copy,
{
    pub vec: Box<Array<'a, T>>,
    index: usize,
}

impl<'a, T: 'a> iterator::Iterator for Iter<'a, T>
where
    T: Clone + PartialEq + Default + Copy,
{
    type Item = T;

    fn clone(&self) -> Self {
        Iter {
            vec: self.vec.clone(),
            index: self.index,
        }
    }

    fn collect<B: Clone + PartialEq>(self) -> Vec<B>
    where
        Self: Sized,
        B: From<Self::Item>,
        B: Clone,
    {
        let mut vec = Vec::new();
        for item in self {
            vec.push(item.into());
        }
        vec
    }

    fn into_iter(self) -> Self {
        self
    }

    fn enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        todo!("Implementar Enumerate para Iter")
        // Enumerate::new(self)
    }

    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map::new(self, f)
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.data.len() {
            let result = &self.vec.data[self.index];
            self.index += 1;
            Some(result.clone())
        } else {
            None
        }
    }
}

impl<'a, T: 'a + Clone> iterator::IntoIterator for Iter<'a, T>
where
    T: PartialEq + Default + Copy,
{
    type Item = T;
    type IntoIter = Iter<'a, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self
    }
}

impl<'a, T> From<Vec<T>> for Iter<'a, T>
where
    T: Clone + PartialEq + Default + Copy,
{
    fn from(vec: Vec<T>) -> Self {
        let array = Array::<'a, T>::from(vec);
        Iter::new(array)
    }
}

impl<'a> FromIterator<Char> for Iter<'a, Char> {
    fn from_iter<I: IntoIterator<Item = Char>>(iter: I) -> Self {
        let vec = Vec::from_iter(iter);
        Iter::from(vec)
    }
}

impl<'a, T> From<Array<'a, T>> for Iter<'a, T>
where
    T: Clone + PartialEq + Default + Copy,
{
    fn from(vec: Array<'a, T>) -> Self {
        Iter::new(vec)
    }
}

impl<'a, T: 'a> Iter<'a, T>
where
    T: Clone + PartialEq + Default + Copy,
{
    pub fn new(vec: Array<'a, T>) -> Iter<'a, T> {
        Iter {
            vec: Box::new(vec),
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

impl<T> std::iter::Iterator for Iter<'_, T>
where
    T: Clone + PartialEq + Default + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        iterator::Iterator::next(self)
    }
}

// macro_rules! impl_std_iterator {
//     (
//         $iter_type:ident < $($params:tt),* > $(where $($where_clause:tt)*)?
//     ) => {
//         impl<$($params),*> std::iter::Iterator for $iter_type<$($params),*>
//             $(where $($where_clause)*)?
//         {
//             type Item = <Self as iterator::Iterator>::Item;

//             fn next(&mut self) -> Option<Self::Item> {
//                 iterator::Iterator::next(self)
//             }
//         }
//     };
// }

// macro_rules! impl_std_into_iterator {
//     (
//         $collection_type:ident < $($params:tt),* > $(where $($where_clause:tt)*)?
//     ) => {
//         impl<$($params),*> std::iter::IntoIterator for $collection_type<$($params),*>
//             $(where $($where_clause)*)?
//         {
//             type Item = <Self as IntoIterator>::Item;
//             type IntoIter = <Self as IntoIterator>::IntoIter;

//             fn into_iter(self) -> Self::IntoIter {
//                 IntoIterator::into_iter(self)
//             }
//         }
//     };
// }

// Implementa std::iter::Iterator para Iter<'a, T> usando la macro
// impl_std_iterator!(Iter<'a, T> where T: Clone + PartialEq + Default,);

// Implementa std::iter::IntoIterator para &MiColeccion<T> usando la macro

// impl_std_into_iterator!(Iter<'a, T> where T: Clone);
