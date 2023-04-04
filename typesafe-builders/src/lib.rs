/*
 * SPDX-FileCopyrightText: Oliver Tale-Yazdi <oliver@tasty.limo>
 * SPDX-License-Identifier: GPL-3.0-only
 */

/// Selection of imports for the `typesafe-builders` crate that "just work".
pub mod prelude {
	pub use typesafe_builders_core::BuilderFactory;
	pub use typesafe_builders_derive::Builder;
}
