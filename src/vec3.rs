#![allow(dead_code)]

use std::ops::{Add, Sub, Mul, Div};
use std::cmp::PartialEq;
pub use num::{Float, Zero, One};
use super::eq::{almost_equal, equal};

/// Vec3 is a generic three-component (2D) vector type.
///
/// The equality operator (==) on a vector is defined differently depending on
/// the type. If the type T is a floating-point number type; equality is defined
/// as near-equality via the almost_equal function. Otherwise if the type is a
/// signed or unsigned integer type, equality is defined as a binary (i.e. 1:1
/// identical) comparison.
#[derive(Copy, Clone)]
pub struct Vec3<T>{
  x: T,
  y: T,
  z: T
}

impl<T> Vec3<T>{
  /// new returns a new vector with the given parameters.
  ///
  /// # Examples
  ///
  /// ```
  /// let x = fiz_math::Vec3::new(4.0f32, 8.0f32, 2.0f32);
  /// ```
  ///
  /// ```
  /// let x = fiz_math::Vec3::new(1u8, 5u8, 2u8);
  /// ```
  pub fn new(x: T, y: T, z: T) -> Vec3<T> {
    Vec3{x: x, y: y, z: z}
  }
}

impl<T: One> One for Vec3<T>{
  /// one returns the one value for a vector whose component type implements the
  /// num::One trait.
  ///
  /// # Examples
  ///
  /// ```
  /// use fiz_math::One;
  ///
  /// let x = fiz_math::Vec3::<f32>::one();
  /// ```
  ///
  /// ```
  /// use fiz_math::One;
  ///
  /// let x = fiz_math::Vec3::<i64>::one();
  /// ```
  fn one() -> Self {
    Vec3{x: T::one(), y: T::one(), z: T::one()}
  }
}

impl<T: Float> Vec3<T>{
  /// almost_equal tells if this vector is equal to the other given an absolute
  /// tolerence value (see the almost_equal function for more details).
  ///
  /// # Examples
  ///
  /// ```
  /// use fiz_math::Vec3;
  ///
  /// let a = Vec3::<f32>::new(1.0, 1.0, 1.0);
  /// let b = Vec3::<f32>::new(0.9, 0.9, 0.9);
  /// assert!(a.almost_equal(b, 0.1000001));
  /// assert!(!a.almost_equal(b, 0.1));
  /// ```
  pub fn almost_equal(self, other: Vec3<T>, abs_tol: T) -> bool {
    almost_equal(self.x, other.x, abs_tol) &&
      almost_equal(self.y, other.y, abs_tol) &&
      almost_equal(self.z, other.z, abs_tol)
  }
}

impl<T: Add<Output = T>> Add for Vec3<T>{
  type Output = Vec3<T>;

  /// add performs component-wise addition of two vectors.
  ///
  /// # Examples
  ///
  /// ```
  /// use fiz_math::Vec3;
  ///
  /// let a = Vec3::new(1, 2, 3);
  /// let b = Vec3::new(4, 5, 6);
  /// assert!(a + b == Vec3::new(5, 7, 9));
  /// ```
  fn add(self, _rhs: Vec3<T>) -> Vec3<T> {
    Vec3{
      x: self.x + _rhs.x,
      y: self.y + _rhs.y,
      z: self.z + _rhs.z,
    }
  }
}

impl<T: Sub<Output = T>> Sub for Vec3<T>{
  type Output = Vec3<T>;

  /// sub performs component-wise subtraction of two vectors.
  ///
  /// # Examples
  ///
  /// ```
  /// use fiz_math::Vec3;
  ///
  /// let a = Vec3::new(1, 2, 3);
  /// let b = Vec3::new(4, 5, 6);
  /// assert!(a - b == Vec3::new(-3, -3, -3));
  /// ```
  fn sub(self, _rhs: Vec3<T>) -> Vec3<T> {
    Vec3{
      x: self.x - _rhs.x,
      y: self.y - _rhs.y,
      z: self.z - _rhs.z,
    }
  }
}

impl<T: Mul<Output = T>> Mul for Vec3<T>{
  type Output = Vec3<T>;

  /// mul performs component-wise multiplication of two vectors.
  ///
  /// # Examples
  ///
  /// ```
  /// use fiz_math::Vec3;
  ///
  /// let a = Vec3::new(1, 2, 3);
  /// let b = Vec3::new(4, 5, 6);
  /// assert!(a * b == Vec3::new(4, 10, 18));
  /// ```
  fn mul(self, _rhs: Vec3<T>) -> Vec3<T> {
    Vec3{
      x: self.x * _rhs.x,
      y: self.y * _rhs.y,
      z: self.z * _rhs.z,
    }
  }
}

impl<T: Div<Output = T>> Div for Vec3<T>{
  type Output = Vec3<T>;

  /// div performs component-wise division of two vectors.
  ///
  /// # Examples
  ///
  /// ```
  /// use fiz_math::Vec3;
  ///
  /// let a = Vec3::new(4, 5, 9);
  /// let b = Vec3::new(1, 2, 3);
  /// assert!(a / b == Vec3::new(4, 2, 3));
  /// ```
  fn div(self, _rhs: Vec3<T>) -> Vec3<T> {
    Vec3{
      x: self.x / _rhs.x,
      y: self.y / _rhs.y,
      z: self.z / _rhs.z,
    }
  }
}

// Different implementations are needed for PartialEq for float (relative
// equality) vs integer types (binary equality). For this reason we must use a
// macro for the separate implementations for each concrete type.

macro_rules! impl_floats {
    ($($ty:ty),*) => ($(
        impl PartialEq for Vec3<$ty> {
            /// eq tests for component-wise equality of two vectors, using the
            /// equal function (i.e. almost_equal with the EPSILON constant).
            ///
            /// # Examples
            ///
            /// ```
            /// use fiz_math::Vec3;
            ///
            /// let a = Vec3::new(4.0, 5.0, 9.0);
            /// let b = Vec3::new(4.0, 5.0, 9.00000000000000000000001);
            /// assert!(a == b);
            /// ```
            fn eq(&self, _rhs: &Vec3<$ty>) -> bool {
                equal(self.x, _rhs.x) &&
                  equal(self.y, _rhs.y) &&
                  equal(self.z, _rhs.z)
            }
        }

        impl Zero for Vec3<$ty>{
          /// zero returns the zero-value for the vector.
          ///
          /// # Examples
          ///
          /// ```
          /// use fiz_math::Zero;
          ///
          /// let x = fiz_math::Vec3::<f32>::zero();
          /// ```
          ///
          /// ```
          /// use fiz_math::Zero;
          ///
          /// let x = fiz_math::Vec3::<f64>::zero();
          /// ```
          fn zero() -> Self {
            Vec3{x: 0.0, y: 0.0, z: 0.0}
          }

          /// is_zero tests if the vector is equal to zero.
          ///
          /// # Examples
          ///
          /// ```
          /// use fiz_math::Zero;
          ///
          /// let x = fiz_math::Vec3::new(1.0, 0.0, 0.0);
          /// assert!(!x.is_zero())
          /// ```
          fn is_zero(&self) -> bool {
            *self == Vec3::<$ty>::zero()
          }
        }
    )*);
}

macro_rules! impl_ints {
    ($($ty:ty),*) => ($(
        impl PartialEq for Vec3<$ty> {
            /// eq tests for component-wise binary equality of two vectors.
            ///
            /// # Examples
            ///
            /// ```
            /// use fiz_math::Vec3;
            ///
            /// let a = Vec3::new(4, 5, 9);
            /// let b = Vec3::new(4, 5, 9);
            /// assert!(a == b);
            /// ```
            ///
            fn eq(&self, _rhs: &Vec3<$ty>) -> bool {
                self.x == _rhs.x &&
                  self.y == _rhs.y &&
                  self.z == _rhs.z
            }
        }

        impl Zero for Vec3<$ty>{
          /// zero returns the zero-value for the vector.
          ///
          /// # Examples
          ///
          /// ```
          /// use fiz_math::Zero;
          ///
          /// let x = fiz_math::Vec3::<u8>::zero();
          /// ```
          ///
          /// ```
          /// use fiz_math::Zero;
          ///
          /// let x = fiz_math::Vec3::<i64>::zero();
          /// ```
          fn zero() -> Self {
            Vec3{x: 0, y: 0, z: 0}
          }

          /// is_zero tests if the vector is equal to zero.
          ///
          /// # Examples
          ///
          /// ```
          /// use fiz_math::Zero;
          ///
          /// let x = fiz_math::Vec3::new(1, 0, 0);
          /// assert!(!x.is_zero())
          /// ```
          fn is_zero(&self) -> bool {
            *self == Vec3::<$ty>::zero()
          }
        }
    )*);
}

impl_floats! { f32, f64 }
impl_ints! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize }