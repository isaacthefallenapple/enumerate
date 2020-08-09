use enumerate::Enumerate;

#[test]
fn test_macro() {
    #[derive(Enumerate, Clone, Copy, PartialEq, Debug)]
    enum Enum {
        A,
        B,
        C,
    }

    let mut iter = Enum::enumerate();
    assert_eq!(iter.next(), Some(&Enum::A));
    assert_eq!(iter.next(), Some(&Enum::B));
    assert_eq!(iter.next(), Some(&Enum::C));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_macro_skips_skipped_variant() {
    #[derive(Enumerate, Clone, Copy, PartialEq, Debug)]
    enum Enum {
        A,
        #[enumerate(skip)]
        _B,
        C,
    }

    let mut iter = Enum::enumerate();
    assert_eq!(iter.next(), Some(&Enum::A));
    assert_eq!(iter.next(), Some(&Enum::C));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_macro_skips_skipped_variants() {
    #[derive(Enumerate, Clone, Copy, PartialEq, Debug)]
    enum Enum {
        A,
        #[enumerate(skip)]
        _B,
        #[enumerate(skip)]
        _C,
    }

    let mut iter = Enum::enumerate();
    assert_eq!(iter.next(), Some(&Enum::A));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_macro_skips_skipped_variant_with_data() {
    #[derive(Enumerate, Clone, PartialEq, Debug)]
    enum Enum {
        A,
        #[enumerate(skip)]
        _B {
            id: usize,
            name: String,
        },
        C,
    }

    let mut iter = Enum::enumerate();
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next(), Some(&Enum::A));
    assert_eq!(iter.next(), Some(&Enum::C));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_macro_skips_skipped_variants_with_data() {
    #[derive(Enumerate, Clone, PartialEq, Debug)]
    enum Enum {
        #[enumerate(skip)]
        _A(u8),
        #[enumerate(skip)]
        _B {
            id: usize,
            name: String,
        },
        C,
    }

    let mut iter = Enum::enumerate();
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some(&Enum::C));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/*.rs");
}

#[test]
fn test_multiple_enumerators() {
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
}

#[test]
fn test_multiple_enumerators_start() {
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
}

#[test]
fn test_override_start() {
    #[derive(Enumerate, PartialEq, Debug)]
    enum Color {
        #[enumerate(start = rgb)]
        Red,
        Green,
        Blue,
        #[enumerate(skip)]
        _Orange,
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
}

#[test]
fn test_default() {
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
}
