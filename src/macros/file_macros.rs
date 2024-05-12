// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2023-2024 LibMake. All rights reserved.

/// Macro to simplify the match logic for file generation.
#[macro_export]
macro_rules! generate_file {
    ($file_type:expr, $value:expr, $generator:expr) => {
        if !$value.trim().is_empty() {
            if let Err(err) = $generator($value) {
                eprintln!(
                    "Error generating {} file: {}",
                    $file_type, err
                );
            }
        }
    };
}
