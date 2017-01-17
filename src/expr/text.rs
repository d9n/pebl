use std::fmt;

use super::Expression;
use ::property::ToObservablePtr;

pub fn to_string<T: PartialEq + fmt::Display>(target: &ToObservablePtr<T>) -> Expression<T, String> {
    Expression::<T, String>::new_unary(target, Box::new(|opt| {
        match opt {
            Some(value) => String::from(format!("{0}", value.get())),
            None => String::from(""),
        }
    }))
}
