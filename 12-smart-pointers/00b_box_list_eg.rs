#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn print_list_rec(list: &List) {
    match list {
        Cons(value, next) => {
            println!("{}", value);
            print_list_rec(&*next);
        }
        Nil => {}
    }
}

fn print_list_ite(list: &List) {
    let mut tmp = list;
    loop {
        match tmp {
            Cons(value, ref next) => {
                println!("{}", value);
                tmp = next;
            }
            Nil => {
                break;
            }
        }
    }
}

fn main() {
    let list = Cons(23, Box::new(Cons(24,Box::new(Cons(54, Box::new(Cons(43, Box::new(Nil))))))));
    print_list_rec(&list);
    print_list_ite(&list);
}
