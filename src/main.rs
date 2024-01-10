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

    let n: MyOption<u128> = MyNone;
    println!("{}", n.unwrap());
}
