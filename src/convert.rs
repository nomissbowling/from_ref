//! convert
//!

/// from_ref_unop without Copy derive
#[macro_export]
macro_rules! from_ref_unop {
  (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
    /// impl '$imp&lt;$u&gt;' for '$t'
    impl $imp<$u> for $t {
      /// '$t' from '$u'
      #[inline]
      fn $method(rhs: $u) -> Self {
        $imp::$method(&rhs)
      }
    }

    crate::convert::from_ref_mut_unop!{impl $imp, $method for $t, $u}
  };
}
pub use from_ref_unop;

/// from_ref_mut_unop without Copy derive
#[macro_export]
macro_rules! from_ref_mut_unop {
  (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
    /// impl '$imp&lt;&amp;mut $u&gt;' for '$t'
    impl<'a> $imp<&'a mut $u> for $t {
      /// '$t' from '&amp; mut $u'
      #[inline]
      fn $method(rhs: &'a mut $u) -> Self {
        // '&*self' means cast '&mut self' to '&self'
        // to avoid ***recursive call*** by using $imp::$method(self)
        $imp::$method(&*rhs)
      }
    }
  };
}
pub use from_ref_mut_unop;
