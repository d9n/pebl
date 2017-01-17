use super::Expression;
use ::property::ToObservablePtr;

pub fn and(targets: &[&ToObservablePtr<bool>]) -> Expression<bool, bool> {
    Expression::<bool, bool>::new(targets, Box::new(|targets| {
        if targets.len() == 0 {
            return false;
        }
        targets.iter().fold(true, |result, val| result && *val.get())
    }))
}
