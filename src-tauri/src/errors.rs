use core::fmt::Debug as fDbg;

macro_rules! eprint_other {
    ($name:ident) => {
        fn eprint_or(self, msg: &str, default: T) -> T {
            self.eprint(msg);
            self.unwrap_or(default)
        }
    };
}

pub trait Eprintln<T> {
    fn eprint(&self, msg: &str);
    fn eprint_or(self, msg: &str, default: T) -> T;
}

#[allow(clippy::use_debug)]
impl<T, E: fDbg> Eprintln<T> for Result<T, E> {
    fn eprint(&self, msg: &str) {
        if let Err(ref err) = *self {
            eprintln!("{msg}: {err:?}");
        }
    }

    eprint_other!(Ok);
}

impl<T> Eprintln<T> for Option<T> {
    fn eprint(&self, msg: &str) {
        if self.is_none() {
            eprintln!("{msg}.");
        }
    }

    eprint_other!(Some);
}
