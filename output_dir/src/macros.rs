//! # Macros for the `test_lib` crate.
//!
//! This crate provides the following macros:
//!
//! - `test_lib`: The main macro for the `test_lib` crate.
//! - `test_lib_print`: Prints the arguments to the console.
//! - `test_lib_vec`: Creates a new vector of the given elements.
//! - `test_lib_map`: Creates a new map of the given key-value pairs.
//! - `test_lib_assert`: Checks if the given expression is true.
//! - `test_lib_min`: Returns the minimum of the given values.
//! - `test_lib_max`: Returns the maximum of the given values.
//! - `test_lib_split`: Splits a string into a vector of words.
//! - `test_lib_join`: Joins a vector of strings into a single string.
//! - `test_lib_print_vec`: Prints a vector of elements to the console.
//!

/// This macro takes any number of arguments and parses them into a
/// Rust value.
#[macro_export]
macro_rules! test_lib {
    ($($tt:tt)*) => {
        // Parse the arguments into a Rust value.
        $crate::parse!($($tt)*)
    };
}

/// This macro prints the arguments to the console.
#[macro_export]
macro_rules! test_lib_print {
    ($($arg:tt)*) => {
        println!("{}", format_args!("{}", $($arg)*));
    };
}

/// This macro creates a new vector of the given elements.
#[macro_export]
macro_rules! test_lib_vec {
    ($($elem:expr),*) => {{
        let mut v = Vec::new();
        $(v.push($elem);)*
        v
    }};
}

/// This macro creates a new map of the given key-value pairs.
#[macro_export]
macro_rules! test_lib_map {
    ($($key:expr => $value:expr),*) => {{
        use std::collections::HashMap;
        let mut m = HashMap::new();
        $(m.insert($key, $value);)*
        m
    }};
}

/// This macro checks if the given expression is true.
#[macro_export]
macro_rules! test_lib_assert {
    ($($arg:tt)*) => {
        if !$($arg)* {
            panic!("Assertion failed!");
        }
    };
}

/// This macro returns the minimum of the given values.
#[macro_export]
macro_rules! test_lib_min {
    ($($x:expr),*) => {{
        let mut min = $($x)*;
        $(if min > $x { min = $x; })*
        min
    }};
}

/// This macro returns the maximum of the given values.
#[macro_export]
macro_rules! test_lib_max {
    ($($x:expr),*) => {{
        let mut max = $($x)*;
        $(if max < $x { max = $x; })*
        max
    }};
}

/// This macro takes a string and splits it into a vector of words.
#[macro_export]
macro_rules! test_lib_split {
    ($s:expr) => {{
        let mut v = Vec::new();
        for w in $s.split_whitespace() {
            v.push(w.to_string());
        }
        v
    }};
}

/// This macro takes a vector of strings and joins them together into a
/// single string.
#[macro_export]
macro_rules! test_lib_join {
    ($($s:expr),*) => {{
        let mut s = String::new();
        $(
            s += &$s;
        )*
        s
    }};
}

/// This macro takes a vector of elements and prints them to the
/// console.
#[macro_export]
macro_rules! test_lib_print_vec {
    ($($v:expr),*) => {{
        for v in $($v),* {
            println!("{}", v);
        }
    }};
}
