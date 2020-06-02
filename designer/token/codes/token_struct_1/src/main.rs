trait Token {
    fn nup(&self);
    fn led(&self);
}

struct PlusToken {
}

impl Token for PlusToken {
    fn nup(&self) {
    }
    fn led(&self) {
    }
}

fn main() {
    Vec::<Box<dyn Token>>::new();
}
