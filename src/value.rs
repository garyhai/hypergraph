/*!
# Value as simple graph

## Code

```rust
//
*/

use core::mem::replace;

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
        self.scalar().is_some()
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

pub trait Vector<K, V> {
    fn insert(&mut self, _: V) -> Option<K> {
        None
    }

    fn get(&self, _: &K) -> Option<&V> {
        None
    }

    fn get_mut(&mut self, _: &K) -> Option<&mut V> {
        None
    }

    fn put(&mut self, _: &K, _: V) -> Option<V> {
        None
    }

    fn remove(&mut self, _: &K) -> Option<V> {
        None
    }
}

impl<T, K, V> Vector<K, V> for T
where
    K: EdgeIndex<Self, Vertex = V>,
{
    fn insert(&mut self, v: V) -> Option<K> {
        let k = K::new(self);
        self.put(&k, v);
        Some(k)
    }

    fn get(&self, key: &K) -> Option<&V> {
        key.get_ref(self)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        key.get_mut(self)
    }

    fn put(&mut self, k: &K, v: V) -> Option<V> {
        k.get_mut(self).map(|x| replace(x, v))
    }

    fn remove(&mut self, k: &K) -> Option<V> {
        k.delete(self);
        None
    }
}

pub trait EdgeIndex<T: ?Sized> {
    type Vertex: ?Sized;
    fn new(_: &T) -> Self;
    fn delete(&self, _: &mut T) -> bool;
    fn get_ref<'a>(&self, _: &'a T) -> Option<&'a Self::Vertex>;
    fn get_mut<'a>(&self, _: &'a mut T) -> Option<&'a mut Self::Vertex>;
}

pub enum Value<'a, S, V: Vector<S, Self>> {
    Null,
    Scalar(S),
    Vector(V),
    Refer(&'a Self),
}

pub type SolidValue<T, V> = Value<'static, T, V>;

impl<T> EdgeIndex<Vec<T>> for usize {
    type Vertex = T;

    fn new(v: &Vec<T>) -> Self {
        v.len()
    }

    fn delete(&self, v: &mut Vec<T>) -> bool {
        if *self < v.len() {
            v.remove(*self);
            true
        } else {
            false
        }
    }

    fn get_ref<'a>(&self, v: &'a Vec<T>) -> Option<&'a Self::Vertex> {
        v.as_slice().get(*self)
    }
    fn get_mut<'a>(&self, v: &'a mut Vec<T>) -> Option<&'a mut Self::Vertex> {
        v.as_mut_slice().get_mut(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    enum MyScalar {
        Integer(i128),
        String(String),
    }

    impl Scalar<i128> for MyScalar {
        fn scalar(&self) -> Option<&i128> {
            if let MyScalar::Integer(n) = self {
                Some(n)
            } else {
                None
            }
        }
        fn to_scalar(&self) -> Option<i128> {
            if let MyScalar::Integer(n) = self {
                Some(*n)
            } else {
                None
            }
        }
        fn as_scalar(&self) -> Option<i128> {
            match self {
                MyScalar::Integer(n) => Some(*n),
                MyScalar::String(s) => s.parse().ok(),
            }
        }
    }

    impl Scalar<String> for MyScalar {
        fn scalar(&self) -> Option<&String> {
            if let MyScalar::String(s) = self {
                Some(s)
            } else {
                None
            }
        }
        fn to_scalar(&self) -> Option<String> {
            if let MyScalar::String(s) = self {
                Some(s.clone())
            } else {
                None
            }
        }

        fn as_scalar(&self) -> Option<String> {
            match self {
                MyScalar::Integer(n) => Some(n.to_string()),
                MyScalar::String(s) => Some(s.clone()),
            }
        }
    }

    impl Scalar<f64> for MyScalar {}
    impl Scalar<Vec<u8>> for MyScalar {}
    impl ScalarValue for MyScalar {}

    #[test]
    fn scalar_test() {
        let v1 = MyScalar::Integer(0);
        assert!(Scalar::<i128>::is_scalar(&v1));
        assert!(scalar!(&v1, is, i128));
        assert!(v1.is_integer());
        assert!(!v1.is_text());

        let v2 = MyScalar::String("123".into());
        assert!(Scalar::<String>::is_scalar(&v2));
        assert!(scalar!(&v2, is, String));
        assert!(v2.is_text());
        assert!(!v2.is_integer());

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

    #[test]
    fn value_test() {
        let mut v: Vec<MyScalar> = vec![MyScalar::Integer(0)];
        let n = Vector::<usize, MyScalar>::insert(&mut v, MyScalar::Integer(1));
        assert_eq!(n, Some(1));
        assert_eq!(v.get(&0), Some(&MyScalar::Integer(0)));
    }
}

/*
```
*/
