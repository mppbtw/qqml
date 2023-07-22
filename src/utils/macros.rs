#[macro_export]
macro_rules! cow_map{
    ( $($key:tt : $val:expr),* $(,)? ) =>{{
        #[allow(unused_mut)]
        let mut map = ::std::collections::HashMap::new();
        $(
            #[allow(unused_parens)]
            let _ = map.insert(Cow::Borrowed($key), $val);
         )*
        map
    }};
}
