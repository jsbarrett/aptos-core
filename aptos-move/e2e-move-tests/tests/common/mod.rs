// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

#[allow(unused)]
pub fn test_dir_path(s: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join(s)
}

#[allow(unused)]
pub fn framework_dir_path(s: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("framework")
        .join(s)
}
