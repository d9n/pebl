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

mod logic {
    use spectral::prelude::*;
    use pebl::prelude::*;

    #[test]
    fn and_expr_works() {
        let mut p1 = Property::new(true);
        let mut p2 = Property::new(true);
        let a = expr::logic::and(&p1, &p2);

        assert_that(&a.get()).is_equal_to(&true);

        p1.set(false);
        assert_that(&a.get()).is_equal_to(&false);

        p2.set(false);
        assert_that(&a.get()).is_equal_to(&false);

        p1.set(true);
        assert_that(&a.get()).is_equal_to(&false);

        drop(p2);
        assert_that(&a.try_get()).is_none();
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
}

mod text {
    use spectral::prelude::*;
    use pebl::prelude::*;

    #[test]
    fn to_string_expr_works() {
        let mut p = Property::new(10);

        let s = expr::text::to_str(&p);
        assert_that(&s.get()).is_equal_to(String::from("10"));

        p.set(-123);
        assert_that(&s.get()).is_equal_to(String::from("-123"));

        drop(p);
        assert_that(&s.try_get()).is_none();
    }
}
