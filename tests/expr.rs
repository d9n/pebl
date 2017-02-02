#[macro_use]
extern crate spectral;
extern crate pebl;

use spectral::prelude::*;
use pebl::prelude::*;

#[test]
fn expressions_are_nestable() {
    use pebl::expr::math::plus;

    let mut p1 = Property::new(1);
    let mut p2 = Property::new(10);
    let mut p3 = Property::new(100);

    let sum = plus(&p1, plus(&p2, &p3));

    assert_that(&sum.get()).is_equal_to(&111);

    p3.set(300);
    assert_that(&sum.get()).is_equal_to(&311);

    p1.set(3);
    assert_that(&sum.get()).is_equal_to(&313);

    p2.set(30);
    assert_that(&sum.get()).is_equal_to(&333);
}

#[test]
fn core_expressions_available_for_expressions_and_properties() {
    let p1 = Property::new(1);
    let p2 = Property::new(10);
    let p3 = Property::new(100);

    let sum = p1.plus(&p2);
    let sum2 = sum.plus(&p3);
    assert_that(&sum2.get()).is_equal_to(&111);

    assert_that(&p3.to_string().get()).is_equal_to(String::from("100"));

    let b1 = Property::new(true);
    let b2 = Property::new(false);

    assert_that(&b1.and(&b2).get()).is_equal_to(&false);
}

#[test]
fn can_chain_different_expression_outputs() {
    let mut p_src = Property::new(String::from(""));
    let mut p_dest = Property::<bool>::default();

    // String -> int -> bool
    p_dest.bind(p_src.len().eq_val(5));
    assert_that(p_dest.get()).is_false();

    p_src.set(String::from("Hello"));
    assert_that(p_dest.get()).is_true();
}

mod logic {
    use spectral::prelude::*;
    use pebl::prelude::*;

    #[test]
    fn and_expr_works() {
        let mut p1 = Property::new(true);
        let mut p2 = Property::new(true);
        let e = expr::logic::and(&p1, &p2);

        assert_that(&e.get()).is_equal_to(&true);

        p1.set(false);
        assert_that(&e.get()).is_equal_to(&false);

        p2.set(false);
        assert_that(&e.get()).is_equal_to(&false);

        p1.set(true);
        assert_that(&e.get()).is_equal_to(&false);
    }

    #[test]
    fn or_expr_works() {
        let mut p1 = Property::new(true);
        let mut p2 = Property::new(true);
        let e = expr::logic::or(&p1, &p2);

        assert_that(&e.get()).is_equal_to(&true);

        p1.set(false);
        assert_that(&e.get()).is_equal_to(&true);

        p2.set(false);
        assert_that(&e.get()).is_equal_to(&false);

        p1.set(true);
        assert_that(&e.get()).is_equal_to(&true);
    }

    #[test]
    fn not_expr_works() {
        let mut p = Property::new(true);
        let e = expr::logic::not(&p);

        assert_that(&e.get()).is_equal_to(&false);

        p.set(false);
        assert_that(&e.get()).is_equal_to(&true);
    }
}

mod math {
    use spectral::prelude::*;
    use pebl::prelude::*;

    #[test]
    fn sum_expr_works_with_int() {
        let p1 = Property::new(10);
        let mut p2 = Property::new(20);

        let sum = expr::math::plus(&p1, &p2);
        assert_that(&sum.get()).is_equal_to(&30);

        p2.set(100);
        assert_that(&sum.get()).is_equal_to(&110);
    }

    #[test]
    fn sum_expr_works_with_float() {
        let p1 = Property::new(10.0);
        let mut p2 = Property::new(20.0);

        let sum = expr::math::plus(&p1, &p2);
        assert_that(&sum.get()).is_equal_to(&30.0);

        p2.set(100.5);
        assert_that(&sum.get()).is_equal_to(&110.5);
    }

    #[test]
    fn abs_expr_works() {
        let mut p = Property::new(0);
        let e = expr::math::abs(&p);

        assert_that(&e.get()).is_equal_to(&0);

        p.set(1234);
        assert_that(&e.get()).is_equal_to(&1234);

        p.set(-4321);
        assert_that(&e.get()).is_equal_to(&4321);
    }

    #[test]
    fn neg_expr_works()
    {
        let mut p = Property::new(0);
        let e = expr::math::neg(&p);

        assert_that(&e.get()).is_equal_to(&0);

        p.set(1234);
        assert_that(&e.get()).is_equal_to(&-1234);

        p.set(-4321);
        assert_that(&e.get()).is_equal_to(&4321);
    }

    #[test]
    fn plus_expr_works()
    {
        let mut p1 = Property::new(10);
        let mut p2 = Property::new(1);

        let e = expr::math::plus(&p1, &p2);

        assert_that(&e.get()).is_equal_to(&11);

        p2.set(0);
        assert_that(&e.get()).is_equal_to(&10);

        p1.set(9000);
        assert_that(&e.get()).is_equal_to(&9000);
    }

    #[test]
    fn times_expr_works()
    {
        let mut p1 = Property::new(10);
        let mut p2 = Property::new(1);

        let e = expr::math::times(&p1, &p2);

        assert_that(&e.get()).is_equal_to(&10);

        p2.set(432);
        assert_that(&e.get()).is_equal_to(&4320);

        p1.set(0);
        assert_that(&e.get()).is_equal_to(&0);
    }

    #[test]
    fn eq_expr_works()
    {
        let mut p1 = Property::new(1);
        let mut p2 = Property::new(1);

        let e = expr::math::eq(&p1, &p2);

        assert_that(&e.get()).is_true();

        p2.set(2);
        assert_that(&e.get()).is_false();

        p1.set(3);
        assert_that(&e.get()).is_false();

        p2.set(3);
        assert_that(&e.get()).is_true();
    }

    #[test]
    fn eq_val_expr_works()
    {
        let mut p = Property::new(1);
        let e = expr::math::eq_val(&p, 2);

        assert_that(&e.get()).is_false();

        p.set(3);
        assert_that(&e.get()).is_false();

        p.set(2);
        assert_that(&e.get()).is_true();
    }

    #[test]
    fn ne_expr_works()
    {
        let mut p1 = Property::new(1);
        let mut p2 = Property::new(1);

        let e = expr::math::ne(&p1, &p2);

        assert_that(&e.get()).is_false();

        p2.set(2);
        assert_that(&e.get()).is_true();

        p1.set(3);
        assert_that(&e.get()).is_true();

        p2.set(3);
        assert_that(&e.get()).is_false();
    }

    #[test]
    fn ne_val_expr_works()
    {
        let mut p = Property::new(1);
        let e = expr::math::ne_val(&p, 2);

        assert_that(&e.get()).is_true();

        p.set(3);
        assert_that(&e.get()).is_true();

        p.set(2);
        assert_that(&e.get()).is_false();
    }

    #[test]
    fn gt_expr_works()
    {
        let mut p1 = Property::new(1);
        let mut p2 = Property::new(1);

        let e = expr::math::gt(&p1, &p2);

        assert_that(&e.get()).is_false();

        p2.set(2);
        assert_that(&e.get()).is_false();

        p1.set(3);
        assert_that(&e.get()).is_true();

        p2.set(3);
        assert_that(&e.get()).is_false();
    }

    #[test]
    fn gt_val_expr_works()
    {
        let mut p = Property::new(1);
        let e = expr::math::gt_val(&p, 2);

        assert_that(&e.get()).is_false();

        p.set(3);
        assert_that(&e.get()).is_true();

        p.set(2);
        assert_that(&e.get()).is_false();
    }

    #[test]
    fn lt_expr_works()
    {
        let mut p1 = Property::new(1);
        let mut p2 = Property::new(1);

        let e = expr::math::lt(&p1, &p2);

        assert_that(&e.get()).is_false();

        p2.set(2);
        assert_that(&e.get()).is_true();

        p1.set(3);
        assert_that(&e.get()).is_false();

        p2.set(3);
        assert_that(&e.get()).is_false();
    }

    #[test]
    fn lt_val_expr_works()
    {
        let mut p = Property::new(1);
        let e = expr::math::lt_val(&p, 2);

        assert_that(&e.get()).is_true();

        p.set(3);
        assert_that(&e.get()).is_false();

        p.set(2);
        assert_that(&e.get()).is_false();
    }

    #[test]
    fn gte_expr_works()
    {
        let mut p1 = Property::new(1);
        let mut p2 = Property::new(1);

        let e = expr::math::gte(&p1, &p2);

        assert_that(&e.get()).is_true();

        p2.set(2);
        assert_that(&e.get()).is_false();

        p1.set(3);
        assert_that(&e.get()).is_true();

        p2.set(3);
        assert_that(&e.get()).is_true();
    }

    #[test]
    fn gte_val_expr_works()
    {
        let mut p = Property::new(1);
        let e = expr::math::gte_val(&p, 2);

        assert_that(&e.get()).is_false();

        p.set(3);
        assert_that(&e.get()).is_true();

        p.set(2);
        assert_that(&e.get()).is_true();
    }

    #[test]
    fn lte_expr_works()
    {
        let mut p1 = Property::new(1);
        let mut p2 = Property::new(1);

        let e = expr::math::lte(&p1, &p2);

        assert_that(&e.get()).is_true();

        p2.set(2);
        assert_that(&e.get()).is_true();

        p1.set(3);
        assert_that(&e.get()).is_false();

        p2.set(3);
        assert_that(&e.get()).is_true();
    }

    #[test]
    fn lte_val_expr_works()
    {
        let mut p = Property::new(1);
        let e = expr::math::lte_val(&p, 2);

        assert_that(&e.get()).is_true();

        p.set(3);
        assert_that(&e.get()).is_false();

        p.set(2);
        assert_that(&e.get()).is_true();
    }
}

mod text {
    use spectral::prelude::*;
    use pebl::prelude::*;

    #[test]
    fn to_string_expr_works() {
        {
            let mut p_int = Property::new(10);

            let e = expr::text::to_string(&p_int);
            assert_that(&e.get()).is_equal_to(String::from("10"));

            p_int.set(-123);
            assert_that(&e.get()).is_equal_to(String::from("-123"));
        }
        {
            let mut p_bool = Property::new(true);

            let e = expr::text::to_string(&p_bool);
            assert_that(&e.get()).is_equal_to(String::from("true"));

            p_bool.set(false);
            assert_that(&e.get()).is_equal_to(String::from("false"));
        }
    }

    #[test]
    fn len_expr_works() {
        let mut p = Property::new(String::from(""));

        let e = expr::text::len(&p);
        assert_that(&e.get()).is_equal_to(&0);

        p.set(String::from("Hello"));
        assert_that(&e.get()).is_equal_to(&5);
    }
}
