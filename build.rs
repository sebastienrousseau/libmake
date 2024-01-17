// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibMake. All rights reserved.

//! This is the main function for the build script.
//!
//! Currently, it only instructs Cargo to re-run this build script if `build.rs` is changed.
fn main() {
    // Avoid unnecessary re-building.
    println!("cargo:rerun-if-changed=build.rs");
}
