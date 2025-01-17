use super::{deserialize, serialize, Deserialize, Serialize};
use serde;
use std::{
	any, borrow::{Borrow, BorrowMut}, boxed, error, fmt, marker, ops::{self, Deref, DerefMut}, rc, sync
};

/// Convenience wrapper around [std::boxed::Box<T>](std::boxed::Box) that automatically uses serde_traitobject for (de)serialization.
#[derive(Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Box<T: ?Sized>(boxed::Box<T>);
impl<T> Box<T> {
	/// Create a new Box wrapper
	pub fn new(t: T) -> Self {
		Self(boxed::Box::new(t))
	}
}
impl<T: ?Sized> Box<T> {
	/// Convert to a regular `std::Boxed::Box<T>`. Coherence rules prevent currently prevent `impl Into<std::boxed::Box<T>> for Box<T>`.
	pub fn into_box(self) -> boxed::Box<T> {
		self.0
	}
}
impl Box<Any> {
	/// Convert into a `std::boxed::Box<dyn std::any::Any>`.
	pub fn into_any(self) -> boxed::Box<any::Any> {
		self.0.into_any()
	}
}
impl Box<Any + Send> {
	/// Convert into a `std::boxed::Box<dyn std::any::Any + Send>`.
	pub fn into_any_send(self) -> boxed::Box<any::Any + Send> {
		self.0.into_any_send()
	}
}
impl Box<Any + Sync> {
	/// Convert into a `std::boxed::Box<dyn std::any::Any + Sync>`.
	pub fn into_any_sync(self) -> boxed::Box<any::Any + Sync> {
		self.0.into_any_sync()
	}
}
impl Box<Any + Send + Sync> {
	/// Convert into a `std::boxed::Box<dyn std::any::Any + Send + Sync>`.
	pub fn into_any_send_sync(self) -> boxed::Box<any::Any + Send + Sync> {
		self.0.into_any_send_sync()
	}
}
impl<
		T: ?Sized + marker::Unsize<U>,
		U: ?Sized,
	> ops::CoerceUnsized<Box<U>> for Box<T>
{
}
impl<T: ?Sized> Deref for Box<T> {
	type Target = boxed::Box<T>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<T: ?Sized> DerefMut for Box<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
impl<T: ?Sized> AsRef<boxed::Box<T>> for Box<T> {
	fn as_ref(&self) -> &boxed::Box<T> {
		&self.0
	}
}
impl<T: ?Sized> AsMut<boxed::Box<T>> for Box<T> {
	fn as_mut(&mut self) -> &mut boxed::Box<T> {
		&mut self.0
	}
}
impl<T: ?Sized> AsRef<T> for Box<T> {
	fn as_ref(&self) -> &T {
		&*self.0
	}
}
impl<T: ?Sized> AsMut<T> for Box<T> {
	fn as_mut(&mut self) -> &mut T {
		&mut *self.0
	}
}
impl<T: ?Sized> Borrow<T> for Box<T> {
	fn borrow(&self) -> &T {
		&*self.0
	}
}
impl<T: ?Sized> BorrowMut<T> for Box<T> {
	fn borrow_mut(&mut self) -> &mut T {
		&mut *self.0
	}
}
impl<T: ?Sized> From<boxed::Box<T>> for Box<T> {
	fn from(t: boxed::Box<T>) -> Self {
		Self(t)
	}
}
// impl<T: Serialize + Deserialize + ?Sized> Into<boxed::Box<T>> for Box<T> {
// 	fn into(self) -> boxed::Box<T> {
// 		self.0
// 	}
// }
impl<T: > From<T> for Box<T> {
	fn from(t: T) -> Self {
		Self(boxed::Box::new(t))
	}
}
impl<T: fmt::Debug + ?Sized> fmt::Debug for Box<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		self.0.fmt(f)
	}
}
impl<T: fmt::Display + ?Sized> fmt::Display for Box<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		self.0.fmt(f)
	}
}
impl<'a, A, R> ops::FnOnce<A> for Box<dyn FnOnce<A, Output = R> + 'a> {
	type Output = R;
	extern "rust-call" fn call_once(self, args: A) -> R {
		self.0.call_once(args)
	}
}
impl<'a, A, R> ops::FnOnce<A> for Box<dyn FnOnce<A, Output = R> + Send + 'a> {
	type Output = R;
	extern "rust-call" fn call_once(self, args: A) -> R {
		self.0.call_once(args)
	}
}
impl<T: Serialize + Deserialize + ?Sized + 'static> serde::ser::Serialize for Box<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serialize(&self.0, serializer)
	}
}
impl<'de, T: Serialize + Deserialize + ?Sized + 'static> serde::de::Deserialize<'de> for Box<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserialize(deserializer).map(Self)
	}
}

/// Convenience wrapper around [std::rc::Rc<T>](std::rc::Rc) that automatically uses serde_traitobject for (de)serialization.
#[derive(Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Rc<T: ?Sized>(rc::Rc<T>);
impl<T> Rc<T> {
	/// Create a new Rc wrapper
	pub fn new(t: T) -> Self {
		Self(rc::Rc::new(t))
	}
}
impl<
		T: ?Sized + marker::Unsize<U>,
		U: ?Sized,
	> ops::CoerceUnsized<Rc<U>> for Rc<T>
{
}
impl<T: ?Sized> Deref for Rc<T> {
	type Target = rc::Rc<T>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<T: ?Sized> DerefMut for Rc<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
impl<T: ?Sized> AsRef<rc::Rc<T>> for Rc<T> {
	fn as_ref(&self) -> &rc::Rc<T> {
		&self.0
	}
}
impl<T: ?Sized> AsMut<rc::Rc<T>> for Rc<T> {
	fn as_mut(&mut self) -> &mut rc::Rc<T> {
		&mut self.0
	}
}
impl<T: ?Sized> AsRef<T> for Rc<T> {
	fn as_ref(&self) -> &T {
		&*self.0
	}
}
impl<T: ?Sized> Borrow<T> for Rc<T> {
	fn borrow(&self) -> &T {
		&*self.0
	}
}
impl<T: ?Sized> From<rc::Rc<T>> for Rc<T> {
	fn from(t: rc::Rc<T>) -> Self {
		Self(t)
	}
}
impl<T: ?Sized> Into<rc::Rc<T>> for Rc<T> {
	fn into(self) -> rc::Rc<T> {
		self.0
	}
}
impl<T> From<T> for Rc<T> {
	fn from(t: T) -> Self {
		Self(rc::Rc::new(t))
	}
}
impl<T: ?Sized> Clone for Rc<T> {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}
impl<T: fmt::Debug + ?Sized> fmt::Debug for Rc<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		self.0.fmt(f)
	}
}
impl<T: fmt::Display + ?Sized> fmt::Display for Rc<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		self.0.fmt(f)
	}
}
impl<T: Serialize + Deserialize + ?Sized + 'static> serde::ser::Serialize for Rc<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serialize(&self.0, serializer)
	}
}
impl<'de, T: Serialize + Deserialize + ?Sized + 'static> serde::de::Deserialize<'de> for Rc<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserialize(deserializer).map(Self)
	}
}

/// Convenience wrapper around [std::sync::Arc<T>](std::sync::Arc) that automatically uses serde_traitobject for (de)serialization.
#[derive(Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Arc<T: ?Sized>(sync::Arc<T>);
impl<T> Arc<T> {
	/// Create a new Arc wrapper
	pub fn new(t: T) -> Self {
		Self(sync::Arc::new(t))
	}
}
impl<
		T: ?Sized + marker::Unsize<U>,
		U: ?Sized,
	> ops::CoerceUnsized<Arc<U>> for Arc<T>
{
}
impl<T: ?Sized> Deref for Arc<T> {
	type Target = sync::Arc<T>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<T: ?Sized> DerefMut for Arc<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
impl<T: ?Sized> AsRef<sync::Arc<T>> for Arc<T> {
	fn as_ref(&self) -> &sync::Arc<T> {
		&self.0
	}
}
impl<T: ?Sized> AsMut<sync::Arc<T>> for Arc<T> {
	fn as_mut(&mut self) -> &mut sync::Arc<T> {
		&mut self.0
	}
}
impl<T: ?Sized> AsRef<T> for Arc<T> {
	fn as_ref(&self) -> &T {
		&*self.0
	}
}
impl<T: ?Sized> Borrow<T> for Arc<T> {
	fn borrow(&self) -> &T {
		&*self.0
	}
}
impl<T: ?Sized> From<sync::Arc<T>> for Arc<T> {
	fn from(t: sync::Arc<T>) -> Self {
		Self(t)
	}
}
impl<T: ?Sized> Into<sync::Arc<T>> for Arc<T> {
	fn into(self) -> sync::Arc<T> {
		self.0
	}
}
impl<T> From<T> for Arc<T> {
	fn from(t: T) -> Self {
		Self(sync::Arc::new(t))
	}
}
impl<T: ?Sized> Clone for Arc<T> {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}
impl<T: fmt::Debug + ?Sized> fmt::Debug for Arc<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		self.0.fmt(f)
	}
}
impl<T: fmt::Display + ?Sized> fmt::Display for Arc<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		self.0.fmt(f)
	}
}
impl<T: Serialize + Deserialize + ?Sized + 'static> serde::ser::Serialize for Arc<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serialize(&self.0, serializer)
	}
}
impl<'de, T: Serialize + Deserialize + ?Sized + 'static> serde::de::Deserialize<'de> for Arc<T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserialize(deserializer).map(Self)
	}
}

/// A convenience trait implemented on all (de)serializable implementors of [std::any::Any].
///
/// It can be made into a trait object which is then (de)serializable.
///
/// # Example
/// ```
/// extern crate serde_json;
/// extern crate serde_traitobject as s;
///
/// use std::any::Any;
///
/// let erased: s::Box<dyn s::Any> = s::Box::new(String::from("hi there"));
///
/// let serialized = serde_json::to_string(&erased).unwrap();
/// let deserialized: s::Box<dyn s::Any> = serde_json::from_str(&serialized).unwrap();
///
/// let downcast: Box<String> = Box::<dyn Any>::downcast(deserialized.into_any()).unwrap();
///
/// println!("{}!", downcast);
/// # assert_eq!(format!("{}!", downcast), "hi there!");
/// // hi there!
/// ```
pub trait Any: Serialize + Deserialize + any::Any {
	/// Convert to a `&std::any::Any`.
	fn as_any(&self) -> &any::Any;
	/// Convert to a `&mut std::any::Any`.
	fn as_any_mut(&mut self) -> &mut any::Any;
	/// Convert to a `std::boxed::Box<dyn std::any::Any>`.
	fn into_any(self: boxed::Box<Self>) -> boxed::Box<any::Any>;
	/// Convert to a `std::boxed::Box<dyn std::any::Any + Send>`.
	fn into_any_send(self: boxed::Box<Self>) -> boxed::Box<any::Any + Send>
	where
		Self: Send;
	/// Convert to a `std::boxed::Box<dyn std::any::Any + Sync>`.
	fn into_any_sync(self: boxed::Box<Self>) -> boxed::Box<any::Any + Sync>
	where
		Self: Sync;
	/// Convert to a `std::boxed::Box<dyn std::any::Any + Send + Sync>`.
	fn into_any_send_sync(self: boxed::Box<Self>) -> boxed::Box<any::Any + Send + Sync>
	where
		Self: Send + Sync;
}
impl<T: Serialize + Deserialize + any::Any> Any for T {
	fn as_any(&self) -> &any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut any::Any {
		self
	}
	fn into_any(self: boxed::Box<Self>) -> boxed::Box<any::Any> {
		self
	}
	fn into_any_send(self: boxed::Box<Self>) -> boxed::Box<any::Any + Send>
	where
		Self: Send,
	{
		self
	}
	fn into_any_sync(self: boxed::Box<Self>) -> boxed::Box<any::Any + Sync>
	where
		Self: Sync,
	{
		self
	}
	fn into_any_send_sync(self: boxed::Box<Self>) -> boxed::Box<any::Any + Send + Sync>
	where
		Self: Send + Sync,
	{
		self
	}
}

/// A convenience trait implemented on all (de)serializable implementors of [std::error::Error].
///
/// It can be made into a trait object which is then (de)serializable.
///
/// # Example
/// ```
/// # extern crate serde;
/// #[macro_use] extern crate serde_derive;
/// extern crate serde_json;
/// extern crate serde_traitobject as s;
///
/// use std::fmt;
///
/// #[derive(Serialize,Deserialize,Debug)]
/// struct MyError(String);
/// impl fmt::Display for MyError {
/// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
/// 		write!(f, "{}", self.0)
/// 	}
/// }
/// impl std::error::Error for MyError {}
///
/// fn fallible() -> Result<(),s::Box<dyn s::Error>> {
/// 	Err(Box::new(MyError(String::from("boxed error"))) as Box<dyn s::Error>)?
/// }
///
/// let serialized = serde_json::to_string(&fallible()).unwrap();
/// let deserialized: Result<(),s::Box<dyn s::Error>> = serde_json::from_str(&serialized).unwrap();
///
/// println!("{:?}", deserialized);
/// # assert_eq!(format!("{:?}", deserialized), "Err(MyError(\"boxed error\"))");
/// // Err(MyError("boxed error"))
/// ```
pub trait Error: error::Error + Serialize + Deserialize {}
impl<T: ?Sized> Error for T where T: error::Error + Serialize + Deserialize {}

/// A convenience trait implemented on all (de)serializable implementors of [std::fmt::Display].
///
/// It can be made into a trait object which is then (de)serializable.
///
/// # Example
/// ```
/// # extern crate serde;
/// #[macro_use] extern crate serde_derive;
/// extern crate serde_json;
/// extern crate serde_traitobject as s;
///
/// fn message() -> s::Box<dyn s::Display> {
/// 	s::Box::new(String::from("boxed displayable"))
/// }
///
/// let serialized = serde_json::to_string(&message()).unwrap();
/// let deserialized: s::Box<dyn s::Display> = serde_json::from_str(&serialized).unwrap();
///
/// println!("{}", deserialized);
/// # assert_eq!(format!("{}", deserialized), "boxed displayable");
/// // boxed displayable
/// ```
pub trait Display: fmt::Display + Serialize + Deserialize {}
impl<T: ?Sized> Display for T where T: fmt::Display + Serialize + Deserialize {}

/// A convenience trait implemented on all (de)serializable implementors of [std::fmt::Debug].
///
/// It can be made into a trait object which is then (de)serializable.
///
/// # Example
/// ```
/// # extern crate serde;
/// #[macro_use] extern crate serde_derive;
/// extern crate serde_json;
/// extern crate serde_traitobject as s;
///
/// fn debug() -> s::Box<dyn s::Debug> {
/// 	s::Box::new(String::from("boxed debuggable"))
/// }
///
/// let serialized = serde_json::to_string(&debug()).unwrap();
/// let deserialized: s::Box<dyn s::Debug> = serde_json::from_str(&serialized).unwrap();
///
/// println!("{:?}", deserialized);
/// # assert_eq!(format!("{:?}", deserialized), "\"boxed debuggable\"");
/// // "boxed debuggable"
/// ```
pub trait Debug: fmt::Debug + Serialize + Deserialize {}
impl<T: ?Sized> Debug for T where T: fmt::Debug + Serialize + Deserialize {}

/// A convenience trait implemented on all (de)serializable implementors of [std::ops::FnOnce].
///
/// It can be made into a trait object which is then (de)serializable.
pub trait FnOnce<Args>: ops::FnOnce<Args> + Serialize + Deserialize {}
impl<T: ?Sized, Args> FnOnce<Args> for T where T: ops::FnOnce<Args> + Serialize + Deserialize {}

/// A convenience trait implemented on all (de)serializable implementors of [std::ops::FnMut].
///
/// It can be made into a trait object which is then (de)serializable.
pub trait FnMut<Args>: ops::FnMut<Args> + Serialize + Deserialize {}
impl<T: ?Sized, Args> FnMut<Args> for T where T: ops::FnMut<Args> + Serialize + Deserialize {}

/// A convenience trait implemented on all (de)serializable implementors of [std::ops::Fn].
///
/// It can be made into a trait object which is then (de)serializable.
pub trait Fn<Args>: ops::Fn<Args> + Serialize + Deserialize {}
impl<T: ?Sized, Args> Fn<Args> for T where T: ops::Fn<Args> + Serialize + Deserialize {}
