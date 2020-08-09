use enumerate::Enumerate;

fn main() {
    #[derive(Enumerate)]
    enum Enum {
        A,
        B { id: usize, name: String },
        C,
    }
}
