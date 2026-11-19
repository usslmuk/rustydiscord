use core::{fmt, iter::FusedIterator};

macro_rules! iterator {
    (
        $(#[$meta:meta])*
        $name:ident<$($lifetime:lifetime,)? K, V> => $inner:ident, $item:ty $(,)?
    ) => {
        $(#[$meta])*
        pub struct $name<$($lifetime,)? K, V>(pub(crate) indexmap::map::$inner<$($lifetime,)? K, V>);

        impl<$($lifetime,)? K, V> Iterator for $name<$($lifetime,)? K, V> {
            type Item = $item;

            fn next(&mut self) -> Option<Self::Item> {
                self.0.next()
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }

            fn count(self) -> usize {
                self.0.count()
            }

            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                self.0.nth(n)
            }

            fn last(self) -> Option<Self::Item> {
                self.0.last()
            }
        }

        impl<$($lifetime,)? K, V> DoubleEndedIterator for $name<$($lifetime,)? K, V> {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.0.next_back()
            }

            fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
                self.0.nth_back(n)
            }
        }

        impl<$($lifetime,)? K, V> ExactSizeIterator for $name<$($lifetime,)? K, V> {
            fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl<$($lifetime,)? K, V> FusedIterator for $name<$($lifetime,)? K, V> {}

        impl<$($lifetime,)? K: fmt::Debug, V: fmt::Debug> fmt::Debug for $name<$($lifetime,)? K, V> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.0).finish()
            }
        }
    };
}

macro_rules! clone {
    ($($name:ident),*) => {
        $(
            impl<K, V> Clone for $name<'_, K, V> {
                fn clone(&self) -> Self {
                    Self(self.0.clone())
                }
            }
        )*
    };
}

iterator! {


    Iter<'a, K, V> => Iter, (&'a K, &'a V),
}

iterator! {


    IterMut<'a, K, V> => IterMut, (&'a K, &'a mut V),
}

iterator! {


    IntoIter<K, V> => IntoIter, (K, V),
}

iterator! {

    Keys<'a, K, V> => Keys, &'a K,
}

iterator! {

    Values<'a, K, V> => Values, &'a V,
}

iterator! {


    ValuesMut<'a, K, V> => ValuesMut, &'a mut V,
}

iterator! {


    IntoKeys<K, V> => IntoKeys, K,
}

iterator! {


    IntoValues<K, V> => IntoValues, V,
}

clone!(Iter, Keys, Values);
