# Enumerate

*Enumerate* let's you it*erate* over your *enum*s!

```rust
use enumerate::Enumerate;

#[derive(Enumerate, PartialEq, Debug)]
enum Musician {
    JohnColtrane,
    MilesDavies,
    EllaFitzgerald,
    DukeEllington,
    NinaSimone,
}

let mut musicians = Musician::enumerate();
assert_eq!(musicians.next(), Some(&Musician::JohnColtrane));
assert_eq!(musicians.next(), Some(&Musician::MilesDavies));
assert_eq!(musicians.next(), Some(&Musician::EllaFitzgerald));
assert_eq!(musicians.next(), Some(&Musician::DukeEllington));
assert_eq!(musicians.next(), Some(&Musician::NinaSimone));
assert_eq!(musicians.next(), None);
```

## Details

Enumerate implements a static method on your type that returns an iterator over its variants
in the order they are declared. This doesn't work for enums with tuple or struct variants.

Variants you don't want in your iteration can be skipped with the `enumerate(skip)` attribute.
This also works for tuple and struct variants.

## Examples

Using `skip` to filter unwanted elements out of iteration:
```rust
#[derive(Enumerate, PartialEq, Debug)]
enum Rgb {
    Red,
    #[enumerate(skip)]
    Orange, // you don't belong here
    Green,
    Blue,
}

let mut colors = Color::enumerate();
assert_eq!(colors.next(), Some(&Color::Red));
assert_eq!(colors.next(), Some(&Color::Green));
assert_eq!(colors.next(), Some(&Color::Blue));
assert_eq!(colors.next(), None);
```

Using `skip` to exclude illegal variants:
```rust
#[derive(Enumerate, PartialEq, Debug)]
enum IllegalRgb {
    Custom(u8, u8, u8), // compile error
    Red,
    Green,
    Blue,
}

#[derive(Enumerate)]
enum LegalRgb {
    #[enumerate(skip)] // now it'll work
    Custom(u8, u8, u8),
    Red,
    Green,
    Blue,
}


let mut colors = LegalRgb::enumerate();
assert_eq!(colors.next(), Some(&Color::Red));
assert_eq!(colors.next(), Some(&Color::Green));
assert_eq!(colors.next(), Some(&Color::Blue));
assert_eq!(colors.next(), None);
```

## Multiple enumerators

If you have one enum but want multiple iterators for it, *Enumerate* has got you covered! You can
specify custom enumerators for your variants using the `enumerate(my_enumerator)` attribute. The
custom enumerator can then be accessed through the `MyEnum::enumerate_my_enumerator` method.

## Examples

Creating custom enumerators for different variants:

```rust
#[derive(Enumerate, PartialEq, Debug)]
enum Color {
    #[enumerate(rgb)]
    Red,
    #[enumerate(cym)]
    Cyan,
    #[enumerate(rgb)]
    Green,
    #[enumerate(cym)]
    Yellow,
    #[enumerate(rgb)]
    Blue,
    #[enumerate(cym)]
    Magenta,
}

let mut rgb = Color::enumerate_rgb();
assert_eq!(rgb.next(), Some(&Color::Red));
assert_eq!(rgb.next(), Some(&Color::Green));
assert_eq!(rgb.next(), Some(&Color::Blue));
assert_eq!(rgb.next(), None);

let mut cym = Color::enumerate_cym();
assert_eq!(cym.next(), Some(&Color::Cyan));
assert_eq!(cym.next(), Some(&Color::Yellow));
assert_eq!(cym.next(), Some(&Color::Magenta));
assert_eq!(cym.next(), None);
```

This is pretty cumbersome, though! You can use the `enumerate(start = my_enumerator)` attribute to
assign a custom enumerator to each variant following the attribute:

```rust
#[derive(Enumerate, PartialEq, Debug)]
enum Color {
    #[enumerate(start = rgb)]
    Red,
    Green,
    Blue,
    #[enumerate(start = cym)]
    Cyan,
    Yellow,
    Magenta,
}

let mut rgb = Color::enumerate_rgb();
assert_eq!(rgb.next(), Some(&Color::Red));
assert_eq!(rgb.next(), Some(&Color::Green));
assert_eq!(rgb.next(), Some(&Color::Blue));
assert_eq!(rgb.next(), None);

let mut cym = Color::enumerate_cym();
assert_eq!(cym.next(), Some(&Color::Cyan));
assert_eq!(cym.next(), Some(&Color::Yellow));
assert_eq!(cym.next(), Some(&Color::Magenta));
assert_eq!(cym.next(), None);
```

Skipping still works as expected:

```rust
#[derive(Enumerate, PartialEq, Debug)]
enum Color {
    #[enumerate(start = rgb)]
    Red,
    Green,
    Blue,
    #[enumerate(skip)]
    Orange, // you still don't belong here
    #[enumerate(start = cym)]
    Cyan,
    Yellow,
    Magenta,
}

let mut rgb = Color::enumerate_rgb();
assert_eq!(rgb.next(), Some(&Color::Red));
assert_eq!(rgb.next(), Some(&Color::Green));
assert_eq!(rgb.next(), Some(&Color::Blue));
assert_eq!(rgb.next(), None);

let mut cym = Color::enumerate_cym();
assert_eq!(cym.next(), Some(&Color::Cyan));
assert_eq!(cym.next(), Some(&Color::Yellow));
assert_eq!(cym.next(), Some(&Color::Magenta));
assert_eq!(cym.next(), None);
```

If you have started an enumerator but want to exempt one variant which should appear in the
standard enumerator, use the `enumerate(default)` attribute to override the started enumerator:

```rust
#[derive(Enumerate, PartialEq, Debug)]
enum Color {
    #[enumerate(start = rgb)]
    Red,
    Green,
    Blue,
    #[enumerate(default)]
    Orange,
    #[enumerate(start = cym)]
    Cyan,
    Yellow,
    Magenta,
}

let mut rgb = Color::enumerate_rgb();
assert_eq!(rgb.next(), Some(&Color::Red));
assert_eq!(rgb.next(), Some(&Color::Green));
assert_eq!(rgb.next(), Some(&Color::Blue));
assert_eq!(rgb.next(), None);

let mut cym = Color::enumerate_cym();
assert_eq!(cym.next(), Some(&Color::Cyan));
assert_eq!(cym.next(), Some(&Color::Yellow));
assert_eq!(cym.next(), Some(&Color::Magenta));
assert_eq!(cym.next(), None);

let mut default = Color::enumerate();
assert_eq!(default.next(), Some(&Color::Orange));
assert_eq!(default.next(), None);
```
