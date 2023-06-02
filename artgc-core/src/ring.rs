use std::fmt::Debug;
use std::ops::{Add, Mul};

pub trait Ring:
    'static
    + Sized
    + Eq
    + Copy
    + Clone
    + Send
    + Sync
    + Debug
    + Add<Output = Self>
    + Mul<Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
{
}
