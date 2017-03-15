use std::cell::Cell;
use std::fmt;
use std::rc::Rc;
use obsv::{InvalidationHandler, ModifyInnerRef, Observable, ObservablePtr};
use expr::{CoreExpressions, Expression, IntoExpression};

/// Data for linking to some target `Expression<T>`. When the expression's value changes, this
/// struct's `dirty` value will be set to `true`.
struct Binding<T: PartialEq> {
    expr: Rc<Expression<T>>,
    #[allow(dead_code)] // Needed to keep weak ref alive
    handle: InvalidationHandler,
    dirty: Rc<Cell<bool>>,
}

/// A property is a value which can get set, queried, and chained.
///
/// Properties are particularly useful for setting up a listening pattern, where one struct wants to
/// expose a bunch of useful data that anyone can listen to. This is particularly good for
/// separating UI views and their data models, for example.
///
/// A property can be bound to a target `Expression<T>`, and a `&Property<T>` can be converted into
/// an `Expression<T>` as well, so that properties can bind to this one.
///
/// See the `Expression<T>` struct for more details.
///
/// # Examples
///
/// ```
/// use pebl::prelude::*;
///
/// let mut p_src = Property::new(10);
/// let p_dst = Property::bound_to(&p_src);
/// assert_eq!(10, *p_dst.get()); // Property.get() -> &T
/// p_src.set(20);
/// assert_eq!(20, *p_dst.get());
/// ```
///
/// Depending on the property's type, it also has access to a bunch of convenient chaining methods.
/// This are more completely documented in `CoreExpressions`.
///
/// ```
/// use pebl::prelude::*;
///
/// let mut i1 = Property::new(1);
/// let mut i2 = Property::new(10);
/// let mut i3 = Property::new(100);
/// let sum = i1.plus(i2.plus(&i3)); // Note: |sum| is an Expression
///
/// let input1 = Property::new(true);
/// let mut input2 = Property::new(false);
/// let and_output = input1.and(&input2); // Note: |and_output| is an Expression
///
/// assert_eq!(111, sum.get()); // Expression.get() -> T
/// assert_eq!(false, and_output.get());
///
/// i1.set(2);
/// i2.set(3);
/// i3.set(5);
///
/// input2.set(true);
///
/// assert_eq!(10, sum.get());
/// assert_eq!(true, and_output.get());
/// ```
pub struct Property<T: PartialEq> {
    value: Observable<T>,
    bound_to: Option<Binding<T>>,
}

impl<T: 'static + PartialEq> Property<T> {
    /// Create a new property, initialized with a target value
    pub fn new(value: T) -> Property<T> {
        Property { value: Observable::new(value), bound_to: None }
    }

    /// Create a new property, bound to a target expression and initialized with its value.
    pub fn bound_to<E: IntoExpression<T>>(target: E) -> Property<T> {
        let expr = target.into_expr();
        let mut p = Property::new(expr.get());
        p.bind_expr(expr);
        p
    }

    /// Return a reference to this property's value. If this property is bound, the value will be
    /// derived from the target expression.
    pub fn get(&self) -> &T {
        if let Some(ref binding) = self.bound_to {
            if binding.dirty.get() {
                let mut value_ptr = ObservablePtr::new(&self.value);
                value_ptr.deref_mut().set(binding.expr.get());
                binding.dirty.set(false);
            }
        }
        self.value.get()
    }

    /// Set the value of this property directly.
    pub fn set(&mut self, value: T) {
        self.value.set(value)
    }

    /// Modify the property's data value in place.
    ///
    /// Instead of calling `set` to overwrite this property's value, this method is useful if you
    /// want to modify the contents directly, such as a appending to a long string or setting a
    /// single value in a large vector.
    ///
    /// # Example
    /// ```
    /// use pebl::prelude::*;
    ///
    /// let mut name = Property::new(String::from("John"));
    /// name.modify_inner().push_str(" Doe");
    /// assert_eq!("John Doe", name.get());
    /// ```
    pub fn modify_inner(&mut self) -> ModifyInnerRef<T> {
        self.value.modify_inner()
    }

    /// Bind this property to some target expression.
    ///
    /// # Example
    /// ```
    /// use pebl::prelude::*;
    ///
    /// let is_citizen = Property::new(true);
    /// let age = Property::new(20);
    /// let mut can_vote = Property::<bool>::default();
    ///
    /// can_vote.bind(is_citizen.and(age.gte_val(18)));
    ///
    /// assert_eq!(true, *can_vote.get());
    /// ```
    pub fn bind<E: IntoExpression<T>>(&mut self, target: E) {
        self.bind_expr(target.into_expr());
    }

    /// Remove a binding previously established by `bind`. It is a no-op to call this method on an
    /// unbound property.
    pub fn unbind(&mut self) {
        self.bound_to = None;
    }

    /// Returns `true` if this property is currently bound to a target expression.
    pub fn is_bound(&self) -> bool {
        self.bound_to.is_some()
    }

    fn bind_expr(&mut self, expr: Rc<Expression<T>>) {
        let dirty = Rc::new(Cell::new(true));
        let dirty_clone = dirty.clone();
        let handle = InvalidationHandler::new(move || dirty_clone.set(true));
        let binding = Binding {
            expr: expr,
            handle: handle,
            dirty: dirty,
        };
        binding.expr.add_invalidation_handler(&binding.handle);
        self.bound_to = Some(binding);
    }
}

/// A simple expression which wraps a `Property<T>`s data and acts as a thin layer around it, simply
/// returning its value directly. This class ultimately allows one property to bind to another
/// (since properties bind to target expressions, not properties).
struct PropertyExpression<T: PartialEq> {
    src: ObservablePtr<T>,
}

impl<T: PartialEq> PropertyExpression<T> {
    pub fn new(src: &Observable<T>) -> Self {
        PropertyExpression { src: ObservablePtr::new(src) }
    }
}

impl<T: 'static + PartialEq + Clone> IntoExpression<T> for PropertyExpression<T> {
    fn into_expr(self) -> Rc<Expression<T>> {
        Rc::new(self)
    }
}

impl<T: 'static + PartialEq + Clone> Expression<T> for PropertyExpression<T> {
    fn try_get(&self) -> Option<T> {
        self.src.try_deref().map(|obsv| obsv.get().clone())
    }

    fn add_invalidation_handler(&self, handler: &InvalidationHandler) {
        if let Some(ref obsv) = self.src.try_deref() {
            obsv.add_invalidation_handler(handler);
        }
    }
}


impl<T: 'static + PartialEq + Default> Default for Property<T> {
    /// Convenience method for creating properties with default values.
    ///
    /// # Example
    ///
    /// ```
    /// use pebl::prelude::*;
    ///
    /// let p_int = Property::<i32>::default();
    /// let p_bool = Property::<bool>::default();
    /// let p_str = Property::<String>::default();
    /// let p_flt = Property::<f32>::default();
    ///
    /// assert_eq!(0, *p_int.get());
    /// assert_eq!(false, *p_bool.get());
    /// assert_eq!("", *p_str.get());
    /// assert_eq!(0f32, *p_flt.get());
    /// ```
    fn default() -> Self {
        Property::new(Default::default())
    }
}

impl<T: PartialEq + Default> Property<T> {
    /// Convenience method for clearing properties whose type supports the `Default` trait
    ///
    /// # Example
    ///
    /// ```
    /// use pebl::prelude::*;
    ///
    /// let mut val = Property::new(String::from("It was a dark and stormy night."));
    /// val.clear();
    /// assert_eq!("", *val.get());
    /// ```
    pub fn clear(&mut self) {
        self.value.clear();
    }
}

impl Property<bool> {
    /// Convenience method for toggling boolean properties
    ///
    /// # Example
    ///
    /// ```
    /// use pebl::prelude::*;
    ///
    /// let mut val = Property::new(false);
    /// val.invert();
    /// assert_eq!(true, *val.get());
    /// ```
    pub fn invert(&mut self) {
        let val = *self.get();
        self.set(!val);
    }
}

impl<'a, T: 'static + PartialEq + Clone> IntoExpression<T> for &'a Property<T> {
    fn into_expr(self) -> Rc<Expression<T>> {
        Rc::new(PropertyExpression::new(&self.value))
    }
}

impl<'a, T: 'static + PartialEq + Clone> CoreExpressions<T> for &'a Property<T> {
    // Default implementations are fine
}

impl<T: 'static + fmt::Debug + PartialEq> fmt::Debug for Property<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Property {{ {:?} }}", self.get())
    }
}
