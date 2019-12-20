//! An error type library for applications.
//!
//! This crate contains two primary items:
//! * [`Fail`](struct.Fail.html) represents failures. This **not** implements [`std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html).
//! * [`Error`](struct.Error.html) is a wrapper for `Fail` that implements `std::error::Error`.
//!
//! [`FailExt`](trait.FailExt.html) is supprting trait. It helps to handling `Result` and `Option`.

use std::error;
use std::fmt;
use std::string::ToString;

/// The failure type.
///
/// # Example
///
/// ```
/// use std::fs::File;
/// use std::io::{BufRead, BufReader};
///
/// use tiny_fail::Fail;
/// #
/// # if let Err(e) = foo() {
/// #     eprintln!("{}", e);
/// # }
///
/// fn foo() -> Result<(), Fail> {
///     let f = File::open("not_exists.txt")?;
///     let r = BufReader::new(f);
///
///     for line in r.lines() {
///         let num: f64 = line?.parse()?;
///         println!("{:.2}", num);
///     }
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Fail {
    msg: Option<String>,
    cause: Option<FailCause>,
}

#[derive(Debug)]
enum FailCause {
    Error(Box<dyn 'static + Send + Sync + error::Error>),
    Fail(Box<Fail>),
}

impl Fail {
    /// Create new `Fail` from message.
    pub fn new<S: ToString>(msg: S) -> Fail {
        Fail {
            msg: Some(msg.to_string()),
            cause: None,
        }
    }

    fn add_msg<S: ToString>(self, msg: S) -> Fail {
        match self {
            Fail { msg: None, cause } => Fail {
                msg: Some(msg.to_string()),
                cause,
            },
            fail => Fail {
                msg: Some(msg.to_string()),
                cause: Some(FailCause::Fail(Box::new(fail))),
            },
        }
    }
}

impl fmt::Display for Fail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let has_msg = if let Some(msg) = &self.msg {
            write!(f, "{}", msg)?;
            true
        } else {
            false
        };

        if let Some(cause) = &self.cause {
            if has_msg {
                write!(f, ": ")?;
            }

            match cause {
                FailCause::Error(e) => e.fmt(f)?,
                FailCause::Fail(fail) => fail.fmt(f)?,
            }
        }

        Ok(())
    }
}

impl<E: 'static + Send + Sync + error::Error> From<E> for Fail {
    fn from(err: E) -> Fail {
        Fail {
            msg: None,
            cause: Some(FailCause::Error(Box::new(err))),
        }
    }
}

/// A wrapper of `Fail` implements `std::error::Error`.
#[derive(Debug)]
pub struct Error(Fail);

impl Error {
    pub fn as_fail(&self) -> &Fail {
        &self.0
    }

    pub fn into_fail(self) -> Fail {
        self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for Error {}

/// A support trait for implementing rich error message.
///
/// This trait implemented for `Option` and `Result`.
/// You can
///
/// # Example
/// ```
/// use std::fs::File;
/// use std::io::{BufRead, BufReader};
///
/// use tiny_fail::{Fail, FailExt};
/// #
/// # if let Err(e) = foo() {
/// #     eprintln!("{}", e);
/// # }
///
/// fn foo() -> Result<(), Fail> {
///     let f = File::open("not_exists.txt").context("failed open not_exists.txt")?;
///     let r = BufReader::new(f);
///
///     for line in r.lines() {
///         let line = line.context("failed read line")?;
///         let num = line.parse::<f64>().context(format!("failed parse \"{}\" into floating number", line))?;
///         println!("{:.2}", num);
///     }
///     Ok(())
/// }
/// ```
pub trait FailExt<T> {
    fn context<S: ToString>(self, msg: S) -> Result<T, Fail>;
}

impl<T, E: 'static + Send + Sync + error::Error> FailExt<T> for Result<T, E> {
    fn context<S: ToString>(self, msg: S) -> Result<T, Fail> {
        self.map_err(|err| Fail {
            msg: Some(msg.to_string()),
            cause: Some(FailCause::Error(Box::new(err))),
        })
    }
}

impl<T> FailExt<T> for Option<T> {
    fn context<S: ToString>(self, msg: S) -> Result<T, Fail> {
        self.ok_or_else(|| Fail::new(msg))
    }
}

impl<T> FailExt<T> for Result<T, Fail> {
    fn context<S: ToString>(self, msg: S) -> Result<T, Fail> {
        self.map_err(|fail| fail.add_msg(msg))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    #[test]
    fn fail_is_send() {
        assert_send::<Fail>();
    }

    #[test]
    fn fail_is_sync() {
        assert_sync::<Fail>();
    }

    #[test]
    fn error_is_send() {
        assert_send::<Error>();
    }

    #[test]
    fn error_is_sync() {
        assert_sync::<Error>();
    }
}
