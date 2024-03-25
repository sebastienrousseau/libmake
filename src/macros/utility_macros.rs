// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

/// Replaces placeholders in a given line with corresponding values from the provided parameters.
///
/// # Arguments
///
/// * `line` - The line containing placeholders to be replaced.
/// * `params` - The parameters containing values to replace the placeholders.
/// * `$($field:ident),+` - Identifiers representing the fields in `params` to be replaced.
///
/// # Returns
///
/// The line with placeholders replaced by their corresponding values.
///
#[macro_export]
macro_rules! macro_replace_placeholder {
    ($line:expr, $params:expr, $($field:ident),+) => {
        {
            let mut line = $line;
            $(
                line = line.replace(
                    concat!("{", stringify!($field), "}"),
                    &$params.$field.as_deref().unwrap_or(""),
                );
            )+
            line
        }
    };
}
