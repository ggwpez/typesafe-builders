/*
 * SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>
 * SPDX-License-Identifier: GPL-3.0-only
 */

// Using two functions here to get it parallel.

#[test]
fn ui_pass() {
    let t = trybuild::TestCases::new();

	t.pass("tests/ui/pass/*.rs");
}

#[test]
fn ui_reject() {
    let t = trybuild::TestCases::new();

	t.compile_fail("tests/ui/reject/*.rs");
}
