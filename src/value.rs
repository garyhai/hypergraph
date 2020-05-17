/*!
# Value as simple graph

## Code

```rust
//
*/

#[macro_export]
macro_rules! scalar {
    ( $s:expr, $t:ty ) => ( Scalar::<$t>::scalar($s) );
    ( $s:expr, is, $t:ty ) => ( Scalar::<$t>::is_scalar($s) );
    ( $s:expr, to, $t:ty ) => ( Scalar::<$t>::to_scalar($s) );
    ( $s:expr, as, $t:ty ) => ( Scalar::<$t>::as_scalar($s) );
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



#[cfg(test)]
mod tests {
    use super::*;

    enum MyValue {
        Integer(i32),
        String(String),
    }

    impl Scalar<i32> for MyValue {
        fn scalar(&self) -> Option<&i32> {
            if let MyValue::Integer(n) = self {
                Some(n)
            } else {
                None
            }
        }
        fn to_scalar(&self) -> Option<i32> {
            if let MyValue::Integer(n) = self {
                Some(*n)
            } else {
                None
            }
        }
    
        fn as_scalar(&self) -> Option<i32> {
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

    #[test]
    fn types() {
        let v1 = MyValue::Integer(0);
        assert!(Scalar::<i32>::is_scalar(&v1));
        assert!(scalar!(&v1, is, i32));
        let v2 = MyValue::String("123".into());
        assert!(Scalar::<String>::is_scalar(&v2));
        assert!(scalar!(&v2, is, String));
        assert_eq!(scalar!(&v1, i32), Some(&0));
        assert_eq!(scalar!(&v2, String), Some(&"123".to_string()));
        assert_eq!(scalar!(&v1, to, i32), Some(0));
        assert_eq!(scalar!(&v2, to, String), Some("123".to_string()));
        assert_eq!(scalar!(&v1, as, String), Some("0".to_string()));
        assert_eq!(scalar!(&v2, as, i32), Some(123));
    }
}

/*
```
*/
