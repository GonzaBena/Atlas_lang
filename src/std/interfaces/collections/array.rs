use crate::std::types::string::String;

pub trait Join<T> {
    fn join(&self, separator: T) -> String;
}
