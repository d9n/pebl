use super::*;

pub fn and<E1: IntoExpression<bool>, E2: IntoExpression<bool>>(lhs: E1, rhs: E2) -> BinaryExpression<bool, bool, bool> {
    ::expr::binary(lhs, rhs, |&val1, &val2| val1 && val2)
}
