#[macro_export]
macro_rules! format {
    ($cap:expr, $($arg:tt)*) => {{
        let mut string: heapless::String<$cap> = heapless::String::new();
        core::fmt::write(&mut string, core::format_args!($($arg)*)).ok();
        string
    }};
}
