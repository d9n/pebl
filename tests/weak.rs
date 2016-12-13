#[macro_use]
extern crate spectral;
extern crate pebl;

use std::rc::Rc;
use spectral::prelude::*;
use pebl::weak::*;


#[test]
fn weak_list_converted_to_strong_using_upgrade() {
    let mut list = WeakList::<i32>::new();
    let int1 = Rc::new(10);
    list.push(&int1);
    {
        let int2 = Rc::new(20);
        list.push(&int2);

        let u = &list.upgrade();
        assert_that(&u.len()).is_equal_to(&2);
        assert_that(&u[0]).is_equal_to(&int1);
        assert_that(&u[1]).is_equal_to(&int2);
    }
    assert_that(&list.upgrade().len()).is_equal_to(&1);
}

#[test]
fn can_iter_weak_list() {
    let mut list = WeakList::<i32>::new();
    let int1 = Rc::new(10);
    let int2 = Rc::new(20);
    let int3 = Rc::new(30);
    list.push(&int1);
    list.push(&int2);
    list.push(&int3);

    {
        let int4 = Rc::new(40);
        list.push(&int4);

        let mut sum = 0;
        for i in list.iter() {
            sum += *i;
        }
        assert_that(&sum).is_equal_to(&100);
    }

    let mut sum = 0;
    for i in list.iter() {
        sum += *i;
    }
    assert_that(&sum).is_equal_to(&60);
}

#[test]
fn can_create_weak_list_with_struct() {
    struct Pt {
        x: i32,
        y: i32,
    }

    let mut list = WeakList::<Pt>::new();
    let pt1 = Rc::new(Pt { x: 1, y: 10 });
    let pt2 = Rc::new(Pt { x: 2, y: 20 });
    let pt3 = Rc::new(Pt { x: 3, y: 30 });
    list.push(&pt1);
    list.push(&pt2);
    list.push(&pt3);

    let pt_sum = list.iter().fold(Pt { x: 0, y: 0 },
                                  |pt1, pt2| { Pt { x: pt1.x + pt2.x, y: pt1.y + pt2.y } });

    assert_that(&pt_sum.x).is_equal_to(&6);
    assert_that(&pt_sum.y).is_equal_to(&60);
}

