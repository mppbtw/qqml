#[macro_export]
macro_rules! hashmap{
    ( $($key:tt : $val:expr),* $(,)? ) =>{{
        #[allow(unused_mut)]
        let mut map = ::std::collections::HashMap::with_capacity(hashmap!(@count $($key),* ));
        $(
            #[allow(unused_parens)]
            let _ = map.insert($key, $val);
         )*
            map
    }};
    (@replace $_t:tt $e:expr ) => { $e };
    (@count $($t:tt)*) => { <[()]>::len(&[$( hashmap!(@replace $t ()) ),*]) }
}
