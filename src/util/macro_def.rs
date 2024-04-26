#[macro_export]
macro_rules! str2vec {
    ($str1:expr, $str2:expr) => {
        [String::from($str1), String::from($str2)]
    };
}

#[macro_export]
macro_rules! str2tuple {
    ($str1:expr, $str2:expr) => {
        (String::from($str1), String::from($str2))
    };
}

#[macro_export]
macro_rules! var2tuple {
    ($str1:expr, $var:expr) => {
        (String::from($str1), $var)
    };
}
