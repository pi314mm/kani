// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#[kani::ensures(|result| true)]
fn always() {}

#[kani::proof]
fn random_harness() {}
