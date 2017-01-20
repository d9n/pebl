use std::ops::Add;

use super::*;

pub fn plus<T, E1: IntoExpression<T>, E2: IntoExpression<T>>(lhs: E1, rhs: E2) -> BinaryExpression<T, T, T>
    where T: PartialEq + Copy + Add<Output = T> {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 + val2)
}
