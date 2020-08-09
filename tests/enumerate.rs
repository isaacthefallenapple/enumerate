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
