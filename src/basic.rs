#[derive(Debug)]
enum MyOption<T> {
    MySome(T),
    MyNone,
}
use MyOption::*;

impl<T> MyOption<T> {
    fn unwrap(self) -> T {
        match self {
            MyNone => panic!("unwrap failed"),
            MySome(v) => v,
        }
    }
}

fn main() {
    let s = MySome(7u8);
    println!("{}", s.unwrap());

    let s = MySome("x");
    println!("{}", s.unwrap());

    // Panics:
    // let n: MyOption<u128> = MyNone;
    // println!("{}", n.unwrap());

    // Won't compile:
    // let a = [MySome(7u8), MySome(true)];

    #[derive(Debug)]
    enum A {
        U8(u8),
        Bool(bool),
    }
    let a = [MySome(A::U8(7u8)), MySome(A::Bool(true))];
    println!("{:?}", a);
}
