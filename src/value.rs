/*!
# Value as simple graph

## Code

```rust
//
*/

#[macro_export]
macro_rules! scalar {
    ( $s:expr, $t:ty ) => {
        Scalar::<$t>::scalar($s)
    };
    ( $s:expr, is, $t:ty ) => {
        Scalar::<$t>::is_scalar($s)
    };
    ( $s:expr, to, $t:ty ) => {
        Scalar::<$t>::to_scalar($s)
    };
    ( $s:expr, as, $t:ty ) => {
        Scalar::<$t>::as_scalar($s)
    };
}

pub trait Scalar<T> {
    fn scalar(&self) -> Option<&T> {
        None
    }

    fn is_scalar(&self) -> bool {
        true
    }

    fn to_scalar(&self) -> Option<T> {
        None
    }

    fn as_scalar(&self) -> Option<T> {
        None
    }
}

pub trait ScalarValue: Scalar<i128> + Scalar<f64> + Scalar<String> + Scalar<Vec<u8>> {
    fn integer(&self) -> Option<i128> {
        scalar!(self, to, i128)
    }

    fn is_integer(&self) -> bool {
        scalar!(self, is, i128)
    }

    fn as_integer(&self) -> Option<i128> {
        scalar!(self, as, i128)
    }

    fn float(&self) -> Option<f64> {
        scalar!(self, to, f64)
    }

    fn is_float(&self) -> bool {
        scalar!(self, is, f64)
    }

    fn as_float(&self) -> Option<f64> {
        scalar!(self, as, f64)
    }

    fn text(&self) -> Option<&str> {
        scalar!(self, String).map(String::as_str)
    }

    fn is_text(&self) -> bool {
        scalar!(self, is, String)
    }

    fn as_text(&self) -> Option<String> {
        scalar!(self, as, String)
    }

    fn binary(&self) -> Option<&[u8]> {
        scalar!(self, Vec<u8>).map(Vec::as_ref)
    }

    fn is_binary(&self) -> bool {
        scalar!(self, is, Vec<u8>)
    }

    fn as_binary(&self) -> Option<Vec<u8>> {
        scalar!(self, as, Vec<u8>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum MyValue {
        Integer(i128),
        String(String),
    }

    impl Scalar<i128> for MyValue {
        fn scalar(&self) -> Option<&i128> {
            if let MyValue::Integer(n) = self {
                Some(n)
            } else {
                None
            }
        }
        fn to_scalar(&self) -> Option<i128> {
            if let MyValue::Integer(n) = self {
                Some(*n)
            } else {
                None
            }
        }
        fn as_scalar(&self) -> Option<i128> {
            match self {
                MyValue::Integer(n) => Some(*n),
                MyValue::String(s) => s.parse().ok(),
            }
        }
    }

    impl Scalar<String> for MyValue {
        fn scalar(&self) -> Option<&String> {
            if let MyValue::String(s) = self {
                Some(s)
            } else {
                None
            }
        }
        fn to_scalar(&self) -> Option<String> {
            if let MyValue::String(s) = self {
                Some(s.clone())
            } else {
                None
            }
        }

        fn as_scalar(&self) -> Option<String> {
            match self {
                MyValue::Integer(n) => Some(n.to_string()),
                MyValue::String(s) => Some(s.clone()),
            }
        }
    }

    impl Scalar<f64> for MyValue {}
    impl Scalar<Vec<u8>> for MyValue {}
    impl ScalarValue for MyValue {}

    #[test]
    fn types() {
        let v1 = MyValue::Integer(0);
        assert!(Scalar::<i128>::is_scalar(&v1));
        assert!(scalar!(&v1, is, i128));
        assert!(v1.is_integer());

        let v2 = MyValue::String("123".into());
        assert!(Scalar::<String>::is_scalar(&v2));
        assert!(scalar!(&v2, is, String));
        assert!(v2.is_text());

        assert_eq!(scalar!(&v1, i128), Some(&0));
        assert_eq!(scalar!(&v2, String), Some(&"123".to_string()));
        assert_eq!(scalar!(&v1, to, i128), Some(0));
        assert_eq!(v1.integer(), Some(0));
        assert_eq!(scalar!(&v2, to, String), Some("123".to_string()));
        assert_eq!(v2.text(), Some("123"));
        assert_eq!(scalar!(&v1, as, String), Some("0".to_string()));
        assert_eq!(v1.as_text(), Some("0".to_string()));
        assert_eq!(scalar!(&v2, as, i128), Some(123));
        assert_eq!(v2.as_integer(), Some(123));
    }
}

/*
```
*/
