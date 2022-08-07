use crate::math::exponentiation::*;
use crate::math::exponentiation_impl::*;
#[test]
fn pow_test() {
    let x = pow(2);
    assert_eq!(x, 4)
}
#[test]
fn pow_impl_test() {
    let x = pow_impl(2);
    assert_eq!(x, 4)
}
