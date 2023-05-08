<h1 align="center"><br>
    Type Safe Builder Pattern
<br></h1>

<h4 align="center">Infallible compile-time checked builders for your structs.</h4>

<p align="center">
  <a href="https://www.gnu.org/licenses/gpl-3.0">
    <img src="https://img.shields.io/badge/License-GPL%20v3-blue.svg" alt="License: GPL v3">
  </a>
  <a href="https://crates.io/crates/typesafe-builders">
    <img src="https://img.shields.io/crates/v/typesafe-builders"/>
  </a>
  <a href="https://docs.rs/crate/typesafe-builders">
  	<img src="https://img.shields.io/docsrs/typesafe-builders"/>
  </a>
  <img src="https://img.shields.io/badge/MSRV-1.65-green"/>
  <a href="https://github.com/ggwpez/typesafe-builders/actions/workflows/rust.yml">
  	<img src="https://github.com/ggwpez/typesafe-builders/actions/workflows/rust.yml/badge.svg"/>
  </a>
</p>

No more worrying whether the `build` call on your builder will return `Ok` or not. Maybe you forgot to set a field? `typesafe-builders` solves this by using the Rust type-system to ensure correct usage.

# Example

```rust
use typesafe_builders::prelude::*;

fn main() {
	#[derive(Builder)]
	struct Point {
		#[builder(constructor)]
		x: u8,
		y: u8,
		#[builder(optional)]
		z: Option<u8>,
	}

	// `builder` requires `x` since it is marked as `constructor`.
	let builder = Point::builder(1);
	// These do not compile:
	// partial.x(6); 		// `x` is already set
	// partial.build();		// `y` is not set

	// `build` is only available once all required fields are set:
	let result = builder.y(2).build();

	assert_eq!(result.x, 1);
	assert_eq!(result.y, 2);
	assert_eq!(result.z, None);
}
```


# Known Downside

I can recommend this only for *internal use*. It is best to not expose these builder types as an API of your crate, since they look extremely ugly and verbose. For example:

```rust
use typesafe_builders::prelude::*;

#[derive(Builder)]
struct Point {
	x: u8,
	y: u8,
	z: u8,
}

// Ugly type name here... and it only gets worse for const-generics etc.
fn preset() -> GenericPointBuilder<false, false, true> {
	Point::builder().z(0)
}

fn main() {
	let partial = preset();
	let point = partial.x(1).y(2).build();
}
```

Please open an MR/Issue if you know how to improve this.

## Field Attributes

Attributes can be combined. Ones that do not work together will throw an explicit error at compile time. Duplicates always error.

### Optional

A field can be set, but does not have to be. Requires the field type to be `Default`.

```rust
use typesafe_builders::prelude::*;

#[derive(Builder)]
pub struct Struct {
	#[builder(optional)]
	x: u8,
}

fn main() {
	// without x
	Struct::builder().build();
	 // with x
	Struct::builder().x(4).build();
}
```

### Constructor

Require a field to be set upon builder construction.

```rust
use typesafe_builders::prelude::*;

#[derive(Builder)]
pub struct Struct {
	#[builder(constructor)]
	x: u8,
}

fn main() {
	Struct::builder(4).build();
	// does not work:
	// Struct::builder(4).x(5).build();
}
```

### Decay

Decay the type to its first generic. Eases use for `Option`, `Box` etc. Requires that the decayed type can be `into`ed its original. Works on all types with one generic arg.

```rust
use typesafe_builders::prelude::*;

#[derive(Builder)]
pub struct Struct {
	#[builder(decay)]
	x: Option<u8>,
}

fn main() {
	// Use `4` of `Some(4)`
	Struct::builder().x(4).build();
}
```

# How does it work?

Const generic one-hot bitfields. What you get is similar to this:

```rust
pub struct Builder<const x_set: bool, const y_set: bool> {
	x: Option<u8>,
	y: Option<u8>,
}

impl<const y_set: bool> Builder<false, y_set> {
    fn set_x(self, x: u8) -> Builder<true, y_set,> {
        unimplemented!()
    }
}

impl<const x_set: bool> Builder<x_set, false> {
    fn set_y(self, y: u8) -> Builder<x_set, true> {
        unimplemented!()
    }
}

// The build function is only available once all fields are set:
impl Builder<true, true> {
    fn build() {

    }
}
```

# More Examples

### Lifetimes

They work as expected

```rust
use typesafe_builders::prelude::*;

#[derive(Builder)]
pub struct Struct<'a, 'b, 'c> {
	x: &'a Box<&'b Option<&'c str>>, // yikes
}

fn main() {
	Struct::builder().x(&Box::new(&Some("hi"))).build();
}
```

### Generics

Works as expected, but does not yet support defaults.

```rust
mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<T: Clone> {
		y: Option<T>,
	}
}

fn main() {
	other::Struct::<u8>::builder().y(Some(4)).build();
}
```

### Const Generics

Works as expected, but does not yet support defaults.

```rust
mod other {
	use typesafe_builders::prelude::*;

	#[derive(Builder)]
	pub struct Struct<const LEN: usize> {
		x: [u8; LEN],
	}
}

fn main() {
	other::Struct::<1>::builder().x([1]).build();
}
```

# TODOs

- [x] Lifetimes
- [x] Generics
  - [x] Bounds
  - [ ] With default
- [x] Const generics
  - [ ] With default
- [x] Add `optional` fields.
- [ ] Add `rename` field attribute.
- [x] Add `constructor` or something like this to have mandatory args directly in the `builder` function.
- [ ] Add `Into` or whatever to cast types.
- [ ] Add way to pass options as `Some` automatically.
- [ ] Cleanup
