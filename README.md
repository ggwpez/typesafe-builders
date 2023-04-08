<h1 align="center"><br>
    Type Safe Builder Patterns
<br></h1>

<h4 align="center">Infallible compile-time checked builders for your structs.</h4>

<p align="center">
  <a href="https://www.gnu.org/licenses/gpl-3.0">
    <img src="https://img.shields.io/badge/License-GPL%20v3-blue.svg" alt="License: GPL v3">
  </a>
  <a href="https://crates.io/crates/typesafe-builders">
    <img src="https://img.shields.io/crates/v/typesafe-builders"/>
  </a>
</p>

No more worrying whether the `build` call on your builder will return `Ok` or not. Maybe you forgot to set a field? `typesafe-builders` solves this by using the Rust type-system to ensure correct usage.

# Example

```rust
fn example() {
	#[derive(Builder)]
	struct Point {
		#[builder(constructor)]
		x: u8,
		y: u8,
		#[builder(optional)]
		z: Option<u8>,
	}

	// The `builder` function requires `x` since it is marked as `constructor`.
	let builder = Point::builder(1);
	// These do not compile:
	// partial.x(6); 		// `x` is already set
	// partial.build();		// `y` is not set

	// Set all required fields to enable the `build` function:
	let result = builder.y(2).build();

	assert_eq!(result.x, 1);
	assert_eq!(result.y, 2);
	assert_eq!(result.z, None);
}
```

## Field Attributes

All attributes must be wrapped in a `builder`, eg. `builder(optional)`.

- `optional` - A field can be set, but is not required to.
- `constructor` - A field must already be set in the `builder` function.

# How does it work?

Const generic one-hot bitfields. What you get is similar to this:

```rust
pub struct Builder<const x_set: bool, const y_set: bool> {
	x: Option<u8>,
	y: Option<u8>,
}

impl<const y_set: bool> Builder<false, y_set> {
    fn set_x(self, x: u8) -> Builder<true, y_set,> {
        …
    }
}

impl<const x_set: bool> Builder<x_set, false> {
    fn set_y(self, y: u8) -> Builder<x_set, true> {
        …
    }
}

// The build function is only available once all fields are set:
impl Builder<true, true> {
    fn build() {

    }
}
```

# TODOs

- [x] Add `optional` fields.
- [ ] Add `rename` field attribute.
- [x] Add `constructor` or something like this to have mandatory args directly in the `builder` function.
- [ ] Add `Into` or whatever to cast types.
- [ ] Add way to pass options as `Some` automatically.
- [ ] Cleanup
