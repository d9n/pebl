#[macro_use]
extern crate spectral;
extern crate pebl;

use spectral::prelude::*;
use pebl::prelude::*;

#[test]
fn expr_can_build_on_other_expr() {
    let p1 = Property::new(1);
    let p2 = Property::new(2);
    let sum1 = expr::math::sum(&[&p1, &p2]);

    let p3 = Property::new(10);
    let p4 = Property::new(20);
    let sum2 = expr::math::sum(&[&p3, &p4]);

    let p5 = Property::new(300);

    let sum3 = expr::math::sum(&[&sum1, &sum2, &p5]);

    assert_that(sum3.get()).is_equal_to(&333);
}

#[test]
fn expression_implements_debug() {
    let p1 = Property::new(10);
    let p2 = Property::new(20);
    let s = expr::math::sum(&[&p1, &p2]);

    let s_string = format!("{:?}", s);

    assert_that(&s_string.len()).is_greater_than(&0);
}

mod logic {
    use spectral::prelude::*;
    use pebl::prelude::*;

    #[test]
    fn and_expr_works() {
        let mut p1 = Property::new(true);
        let mut p2 = Property::new(true);
        let a = expr::logic::and(&[&p1, &p2]);

        assert_that(a.get()).is_equal_to(&true);

        p1.set(false);
        assert_that(a.get()).is_equal_to(&false);

        p2.set(false);
        assert_that(a.get()).is_equal_to(&false);

        p1.set(true);
        assert_that(a.get()).is_equal_to(&false);

        drop(p2);
        assert_that(a.get()).is_equal_to(&true);
    }
}

mod math {
    use spectral::prelude::*;
    use pebl::prelude::*;

    #[test]
    fn sum_expr_works_with_int() {
        let p1 = Property::new(10);
        let mut p2 = Property::new(20);
        let p3 = Property::new(30);

        let s = expr::math::sum(&[&p1, &p2, &p3]);
        assert_that(s.get()).is_equal_to(&60);

        p2.set(100);
        assert_that(s.get()).is_equal_to(&140);
    }

    #[test]
    fn sum_expr_works_with_float() {
        let p1 = Property::new(10.0);
        let mut p2 = Property::new(20.0);
        let p3 = Property::new(30.0);

        let s = expr::math::sum(&[&p1, &p2, &p3]);
        assert_that(s.get()).is_equal_to(&60.0);

        p2.set(100.0);
        assert_that(s.get()).is_equal_to(&140.0);
    }
}

mod text {
    use spectral::prelude::*;
    use pebl::prelude::*;

    #[test]
    fn to_string_expr_works() {
        let mut p = Property::new(10);

        let s = expr::text::to_string(&p);
        assert_that(s.get()).is_equal_to(String::from("10"));

        p.set(-123);
        assert_that(s.get()).is_equal_to(String::from("-123"));

        drop(p);
        assert_that(s.get()).is_equal_to(String::from(""));
    }
}
