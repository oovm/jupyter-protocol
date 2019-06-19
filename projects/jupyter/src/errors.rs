use failure::Error;

/** Wrapped result type for this crate.

This is just a `failure::Error` error type, with generic `Ok` type.
*/
pub type Result<T> = ::std::result::Result<T, Error>;
