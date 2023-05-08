/*
 * SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>
 * SPDX-License-Identifier: GPL-3.0-only
 */

#![doc = include_str!(env!("README_PATH"))]
#![deny(unsafe_code)]

/// Selection of imports that "just work".
pub mod prelude {
	pub use typesafe_builders_derive::Builder;
}
