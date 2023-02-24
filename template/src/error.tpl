use derive_more::Display;
use std::fmt::{self, Debug, Display, Formatter};

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Display)]
#[display(fmt = "rust-lib-template: {}")]
pub enum Error {
    /// Something wrong happened.
    #[display(fmt = "Something wrong happened: {}", _0)]
    SomethingWrong(BoxError),

    /// Something wrong happened again.
    #[display(fmt = "Something wrong happened again: {}", data)]
    SomethingWrongAgain { data: String },

    /// Some other wrong happened.
    #[display(fmt = "Some other wrong happened")]
    DifferentOne,

    #[doc(hidden)]
    __Nonexhaustive,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl std::error::Error for Error {}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl Eq for Error {}
