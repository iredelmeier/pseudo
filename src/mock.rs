use std::cell::RefCell;
use std::rc::Rc;

type OptionalRef<T> = Rc<RefCell<Option<T>>>;

/// Used for tracking function call arguments and specifying a predetermined
/// return value or mock function.
///
/// See the crate documentation for more substantial examples, including some
/// that demonstrate how to use `Mock` for methods that have multiple arguments
/// as well as methods with argument or return types that do not implement
/// `Clone`.
#[derive(Debug, Clone)]
pub struct Mock<C, R>
    where C: Clone,
          R: Clone
{
    return_value: Rc<RefCell<R>>,
    mock_fn: OptionalRef<fn(C) -> R>,
    calls: Rc<RefCell<Vec<C>>>,
}

impl<C, R> Mock<C, R>
    where C: Clone,
          R: Clone
{
    /// Creates a new `Mock` that will return `return_value`.
    pub fn new(return_value: R) -> Self {
        Mock {
            return_value: Rc::new(RefCell::new(return_value)),
            mock_fn: Rc::new(RefCell::new(None)),
            calls: Rc::new(RefCell::new(vec![])),
        }
    }

    /// Use the `Mock` to return a value, keeping track of the arguments used.
    ///
    /// Depending on what has most recently been called, this will return:
    /// - the return value specified at construction time
    /// - the return value specified via `Mock::return_value` or a derivative,
    /// such as `Mock::return_some`
    /// - the output of the function set via `Mock::use_fn` with the current arguments
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<&str, _>::new("return value");
    /// assert_eq!(mock.call("something"), "return value");
    ///
    /// mock.return_value("different value");
    /// assert_eq!(mock.call("something"), "different value");
    ///
    /// mock.use_fn(str::trim);
    /// assert_eq!(mock.call("  test  "), "test");
    /// ```
    pub fn call(&self, args: C) -> R {
        let mock_fn = *self.mock_fn.borrow();
        self.calls.borrow_mut().push(args.clone());

        if let Some(mock_fn) = mock_fn {
            mock_fn(args)
        } else {
            self.return_value.borrow().clone()
        }
    }

    /// Override the initial return value.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<&str, _>::new("original value");
    /// mock.return_value("new value");
    ///
    /// assert_eq!(mock.call("something"), "new value");
    /// ```
    pub fn return_value(&self, return_value: R) {
        self.set_return_value(return_value)
    }

    /// Specify a function to determine the `Mock`'s return value based on
    /// the arguments provided to `Mock::call`.
    ///
    /// Arguments of `Mock::call` are still tracked.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// fn add_two(x: i64) -> i64 {
    ///     x + 2
    /// }
    ///
    /// let mock = Mock::<i64, i64>::new(10);
    /// mock.use_fn(add_two);
    ///
    /// assert_eq!(mock.call(1), 3);
    /// assert_eq!(mock.call(10), 12);
    /// ```
    ///
    /// For functions with multiple arguments, use a tuple:
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// fn add((x, y, z): (i64, i64, i64)) -> i64 {
    ///     x + y + z
    /// }
    ///
    /// let mock = Mock::<(i64, i64, i64), i64>::default();
    /// mock.use_fn(add);
    ///
    /// assert_eq!(mock.call((1, 1, 1)), 3);
    /// assert_eq!(mock.call((1, 2, 3,)), 6);
    /// ```
    pub fn use_fn(&self, mock_fn: fn(C) -> R) {
        self.set_mock_fn(mock_fn)
    }

    /// Returns true if `Mock::call` has been called.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<i64, ()>::default();
    ///
    /// assert!(!mock.called());
    ///
    /// mock.call(10);
    ///
    /// assert!(mock.called());
    /// ```
    pub fn called(&self) -> bool {
        !self.calls.borrow().is_empty()
    }

    /// Returns the number of times `Mock::call` has been called.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<i64, i64>::new(0);
    ///
    /// assert_eq!(mock.num_calls(), 0);
    /// mock.call(5);
    /// assert_eq!(mock.num_calls(), 1);
    /// mock.call(10);
    /// assert_eq!(mock.num_calls(), 2);
    /// ```
    pub fn num_calls(&self) -> usize {
        self.calls.borrow().len()
    }

    /// Returns the arguments to `Mock::call` in order from first to last.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<&str, _>::new("");
    ///
    /// mock.call("first");
    /// mock.call("second");
    /// mock.call("third");
    ///
    /// assert_eq!(mock.calls().as_slice(), ["first", "second", "third"]);
    /// ```
    pub fn calls(&self) -> Vec<C> {
        self.calls.borrow().clone()
    }

    /// Reset the call history for the `Mock`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<&str, &str>::default();
    ///
    /// mock.call("first");
    /// mock.call("second");
    ///
    /// assert!(mock.called());
    /// assert_eq!(mock.num_calls(), 2);
    /// assert!(mock.called_with("first"));
    /// assert!(mock.called_with("second"));
    ///
    /// mock.reset_calls();
    ///
    /// assert!(!mock.called());
    /// assert_eq!(mock.num_calls(), 0);
    /// assert!(!mock.called_with("first"));
    /// assert!(!mock.called_with("second"));
    /// ```
    pub fn reset_calls(&self) {
        self.calls.borrow_mut().clear()
    }

    fn set_return_value(&self, return_value: R) {
        *self.return_value.borrow_mut() = return_value
    }

    fn set_mock_fn(&self, mock_fn: fn(C) -> R) {
        *self.mock_fn.borrow_mut() = Some(mock_fn)
    }
}

impl<C, R> Default for Mock<C, R>
    where C: Clone,
          R: Clone + Default
{
    /// Use `R::default()` as the initial return value.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<i64, i64>::default();
    /// assert_eq!(mock.call(10), 0);
    ///
    /// let mock = Mock::<(), String>::default();
    /// assert_eq!(&mock.call(()), "");
    ///
    /// let mock = Mock::<(i64, &str), Option<bool>>::default();
    /// assert_eq!(mock.call((10, "test")), None);
    /// ```
    fn default() -> Self {
        Self::new(R::default())
    }
}

impl<C, R> Mock<C, R>
    where C: Clone + PartialEq,
          R: Clone
{
    /// Returns true if the specified argument has been used for `Mock::call`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<&str, ()>::new(());
    /// mock.call("foo");
    /// mock.call("bar");
    ///
    /// assert!(mock.called_with("foo"));
    /// assert!(mock.called_with("bar"));
    /// assert!(!mock.called_with("baz"));
    /// ```
    pub fn called_with<T: Into<C>>(&self, args: T) -> bool {
        self.calls.borrow().contains(&args.into())
    }
}

impl<C, S> Mock<C, Option<S>>
    where C: Clone,
          S: Clone
{
    /// Return `Some(return_value)` from `Mock::call`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<(), Option<i64>>::new(None);
    /// mock.return_some(10);
    ///
    /// assert_eq!(mock.call(()), Some(10));
    /// ```
    pub fn return_some(&self, return_value: S) {
        self.set_return_value(Some(return_value))
    }

    /// Return `None` from `Mock::call`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<(), Option<i64>>::new(Some(42));
    /// mock.return_none();
    ///
    /// assert_eq!(mock.call(()), None);
    /// ```
    pub fn return_none(&self) {
        self.set_return_value(None)
    }
}

impl<C, O, E> Mock<C, Result<O, E>>
    where C: Clone,
          O: Clone,
          E: Clone
{
    /// Return `Ok(return_value)` from `Mock::call`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<(), Result<&str, &str>>::new(Err("oh no"));
    /// mock.return_ok("success");
    ///
    /// assert_eq!(mock.call(()), Ok("success"));
    /// ```
    pub fn return_ok(&self, return_value: O) {
        self.set_return_value(Ok(return_value))
    }

    /// Return `Err(return_value)` from `Mock::call`.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<(), Result<&str, &str>>::new(Ok("success"));
    /// mock.return_err("oh no");
    ///
    /// assert_eq!(mock.call(()), Err("oh no"));
    /// ```
    pub fn return_err(&self, return_value: E) {
        self.set_return_value(Err(return_value))
    }
}