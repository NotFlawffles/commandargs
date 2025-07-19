use std::{env, fmt::Display};

/// A wrapper around arguments being passed to `CommandPattern`. It is used to determine which
/// source of arguments to use.
pub enum Args {
    /// Tells the `CommandPattern` to use command line arguments (`std::env::Args` by default).
    CommandLineArgs,

    /// Tells the `CommandPattern` to use a custom user-defined `Vec` of arguments that implement
    /// `Display` trait. It is usually used for debugging or parsing purposes.
    ///
    /// ### Example
    /// ```rust
    /// Args::Vec(vec!["new", "exe", "big_cat"]),
    /// ```
    Vec(Vec<&'static dyn Display>),
}

impl Args {
    pub fn to_vec(&self) -> Vec<String> {
        match self {
            Self::CommandLineArgs => env::args().collect(),
            Self::Vec(vec) => vec.iter().map(|arg| arg.to_string()).collect(),
        }
    }
}
