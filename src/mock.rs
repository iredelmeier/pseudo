use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, RwLock};

type OptionalRef<T> = Arc<RwLock<Option<T>>>;

/// Used for tracking function call arguments and specifying a predetermined
/// return value or mock function.
///
/// See the crate documentation for more substantial examples, including some
/// that demonstrate how to use `Mock` for methods that have multiple arguments
/// as well as methods with argument or return types that do not implement
/// `Clone`.
#[derive(Clone)]
pub struct Mock<C, R>
    where C: Clone,
          R: Clone
{
    return_value: Arc<RwLock<R>>,
    mock_fn: OptionalRef<fn(C) -> R>,
    mock_closure: OptionalRef<Box<Fn(C) -> R>>,
    calls: Arc<RwLock<Vec<C>>>,
}

impl<C, R> Mock<C, R>
    where C: Clone,
          R: Clone
{
    /// Creates a new `Mock` that will return `return_value`.
    pub fn new<T: Into<R>>(return_value: T) -> Self {
        Mock {
            return_value: Arc::new(RwLock::new(return_value.into())),
            mock_fn: Arc::new(RwLock::new(None)),
            mock_closure: Arc::new(RwLock::new(None)),
            calls: Arc::new(RwLock::new(vec![])),
        }
    }

    /// Use the `Mock` to return a value, keeping track of the arguments used.
    ///
    /// Depending on what has most recently been called, this will return:
    /// - the return value specified at construction time
    /// - the return value specified via `Mock::return_value` or a derivative,
    /// such as `Mock::return_some`
    /// - the output of the function set via `Mock::use_fn` with the current arguments
    /// - the output of the closure set via `Mock::use_closure` with the current arguments
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
    ///
    /// mock.use_closure(Box::new(|x| x.trim_left()));
    /// assert_eq!(mock.call("  test  "), "test  ");
    ///
    /// mock.use_fn(str::trim);
    /// assert_eq!(mock.call("  test  "), "test");
    /// ```
    pub fn call(&self, args: C) -> R {
        self.calls.write().unwrap().push(args.clone());

        if let Some(ref mock_fn) = *self.mock_fn.read().unwrap() {
            return mock_fn(args);
        }

        if let Some(ref mock_closure) = *self.mock_closure.read().unwrap() {
            return mock_closure(args);
        }

        self.return_value.read().unwrap().clone()
    }

    /// Override the initial return value.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<&str, &str>::new("original value");
    /// mock.return_value("new value");
    ///
    /// assert_eq!(mock.call("something"), "new value");
    /// ```
    pub fn return_value<T: Into<R>>(&self, return_value: T) {
        let mut value = self.return_value.write().unwrap();
        *value = return_value.into()
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
        let mut closure_value = self.mock_closure.write().unwrap();
        *closure_value = None;

        let mut fn_value = self.mock_fn.write().unwrap();
        *fn_value = Some(mock_fn)
    }

    /// Specify a closure to determine the `Mock`'s return value based on
    /// the arguments provided to `Mock::call`.
    ///
    /// Arguments of `Mock::call` are still tracked.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<i64, i64>::new(10);
    /// let add_two = |x| x + 2;
    /// mock.use_closure(Box::new(add_two));
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
    /// let mock = Mock::<(i64, i64, i64), i64>::default();
    /// let add = |(x, y, z)| x + y + z;
    /// mock.use_closure(Box::new(add));
    ///
    /// assert_eq!(mock.call((1, 1, 1)), 3);
    /// assert_eq!(mock.call((1, 2, 3,)), 6);
    /// ```
    pub fn use_closure(&self, mock_fn: Box<Fn(C) -> R>) {
        let mut fn_value = self.mock_fn.write().unwrap();
        *fn_value = None;

        let mut closure_value = self.mock_closure.write().unwrap();
        *closure_value = Some(mock_fn)
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
    /// // mock.call(10);
    ///
    /// // assert!(mock.called());
    /// ```
    pub fn called(&self) -> bool {
        let calls = (*self.calls).read().unwrap();
        !calls.is_empty()
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
        self.calls.read().unwrap().len()
    }

    /// Returns the arguments to `Mock::call` in order from first to last.
    ///
    /// # Examples
    ///
    /// ```
    /// use pseudo::Mock;
    ///
    /// let mock = Mock::<&str, &str>::new("");
    ///
    /// mock.call("first");
    /// mock.call("second");
    /// mock.call("third");
    ///
    /// assert_eq!(mock.calls().as_slice(), ["first", "second", "third"]);
    /// ```
    pub fn calls(&self) -> Vec<C> {
        self.calls.read().unwrap().clone()
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
        self.calls.write().unwrap().clear()
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
        self.calls.read().unwrap().contains(&args.into())
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
    pub fn return_some<T: Into<S>>(&self, return_value: T) {
        self.return_value(Some(return_value.into()))
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
        self.return_value(None)
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
    pub fn return_ok<T: Into<O>>(&self, return_value: T) {
        self.return_value(Ok(return_value.into()))
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
    pub fn return_err<T: Into<E>>(&self, return_value: T) {
        self.return_value(Err(return_value.into()))
    }
}

impl<C, R> Debug for Mock<C, R>
    where C: Clone + Debug,
          R: Clone + Debug
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Mock")
            .field("return_value", &self.return_value)
            .field("calls", &self.calls)
            .finish()
    }
}
