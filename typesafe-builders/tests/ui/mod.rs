/*
 * SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! UI tests but at the same time they also run the assertions.

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
