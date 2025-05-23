#[derive(Debug)]
enum NoLista {
    Cons(i32, Box<NoLista>),
    Nil,
}
#[derive(Debug)]
struct Lista {
    cab: Box<NoLista>,
}

impl NoLista {
    fn ins_cauda(self: Box<Self>, x: i32) -> Box<Self> {
        match *self {
            Self::Nil => Box::new(Self::Cons(x, Box::new(Self::Nil))),
            Self::Cons(v,t) => Box::new(Self::Cons(v, t.ins_cauda(x))),
        }
    }

    fn rem_cauda(self: Box<Self>) -> (Option<i32>, Box<Self>) {
        match *self {
            Self::Nil => (None, self),
            Self::Cons(x, t) => match *t {
                Self::Nil => (Some(x), Box::new(Self::Nil)),
                Self::Cons(x, t) => {
                    let (r, new_t) = t.rem_cauda();
                    (r, Box::new(Self::Cons(x, new_t)))
                }
            }
        }
    }
    fn ins_ordenado(self: Box<Self>, x: i32) -> Box<Self> {
        match *self {
            Self::Nil => Box::new(Self::Cons(x, Box::new(Self::Nil))),
            Self::Cons(v, t) if x <= v => {
                Box::new(Self::Cons(x, Box::new(Self::Cons(v, t))))
            },
            Self::Cons(v, t) => Box::new(Self::Cons(v, t.ins_ordenado(x)))
        }
    }
}
impl Lista {
    fn new() -> Self {
        Self {cab: Box::new(NoLista::Nil)}
    }
    fn ins_cauda(&mut self, x: i32) {
        let old_lista = std::mem::replace(&mut self.cab, Box::new(NoLista::Nil));
        self.cab = old_lista.ins_cauda(x);
    }
    fn rem_cauda(&mut self) -> Option<i32> {
        let old_lista = std::mem::replace(&mut self.cab, Box::new(NoLista::Nil));
        let (r, t) = old_lista.rem_cauda();
        self.cab = t;
        r
    }
    fn ins_ordenado(&mut self, x: i32) {
        let old_lista = std::mem::replace(&mut self.cab, Box::new(NoLista::Nil));
        self.cab = old_lista.ins_ordenado(x);
    }
}
fn main() {
    let mut lista = Lista::new();
    lista.ins_cauda(1);
    lista.ins_cauda(2);
    
    println!("Após inserir 1 e 2: {:?}", lista);

    lista.rem_cauda();

    println!("Após remover a cauda: {:?}", lista);

    lista.ins_ordenado(213);
    lista.ins_ordenado(215);
    lista.ins_ordenado(10);
    lista.ins_ordenado(0);

    println!("Após inserções ordenadas: {:?}", lista);
}
