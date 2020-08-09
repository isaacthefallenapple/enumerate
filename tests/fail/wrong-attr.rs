use enumerate::Enumerate;

fn main() {
    #[derive(Enumerate)]
    enum Enum {
        A,
        B,
        #[enumerate(wrong attr)]
        C,
    }
}
