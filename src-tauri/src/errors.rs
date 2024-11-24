use crate::env::LOGS_PATH;
use core::fmt::Debug as fDbg;
use std::fs;
use std::io::Write as _;

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

impl<T> Eprintln<T> for Option<T> {
    fn eprint(&self, msg: &str) {
        if self.is_none() {
            eprintln!("{msg}.");
        }
    }

    eprint_other!(Some);
}

#[allow(clippy::use_debug)]
impl<T, E: fDbg> Eprintln<T> for Result<T, E> {
    fn eprint(&self, msg: &str) {
        if let Err(ref err) = *self {
            eprintln!("{msg}: {err:?}");
            #[cfg(feature = "logs")]
            if let Err(er) = fs::OpenOptions::new()
                .append(true)
                .open(LOGS_PATH)
                .and_then(|mut fd| writeln!(fd, "{msg}: {err:?}"))
            {
                eprint!("Failed to log errors in {LOGS_PATH} ! ({er:?})");
            }
        }
    }

    eprint_other!(Ok);
}
