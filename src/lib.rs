#[macro_export]
macro_rules! dbgf {
    ($fmt: tt $(,)?) => {
        ::std::eprintln!("[{}:{}]", ::std::file!(), ::std::line!())
    };
    ($fmt: tt, $val:expr $(,)?) => {
        match $val {
            tmp => {
                ::std::eprintln!(concat!("[{}:{}] {} = {:", $fmt, "}"),
                    ::std::file!(), ::std::line!(), ::std::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($fmt: tt $(, $val:expr)+ $(,)?) => {
        ($($crate::dbgf!($fmt, $val)),+,)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        #[derive(Debug, Clone)]
        struct S {
            i: Vec<f32>,
        }

        let s = S {
            i: vec![11.0 / 3.0; 10],
        };
        dbg!(&s, &s.i);
        dbgf!("5.3?", &s, &s.i);
    }
}
