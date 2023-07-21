#[macro_export]
macro_rules! hashmap {
    ($($($index:expr => $val:expr)+$(,)?)*) => ({
        let mut hm = ::std::collections::HashMap::new();

        $($(
            hm.insert($index, $val);
        )+)*

        hm
    })
}
