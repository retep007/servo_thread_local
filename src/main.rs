#[macro_use] extern crate lazy_static;

use std::marker::PhantomData;
use std::any::Any;
use std::cell::RefCell;

thread_local!(static STACK: RefCell<Vec<Box<Any>>> = RefCell::new(Vec::new()));

trait TypeHolder: 'static + Sync + Send {}

#[derive(Debug, PartialEq)]
struct StackEntry<TH: TypeHolder> {
    p: PhantomData<TH>
}

// impl<T: TypeHolder> typemap::Key for StackEntry<T> {
//     type Value = StackEntry<TH>;
// }

impl<T: TypeHolder> StackEntry<T> {
    fn print(&self) {
        println!("hello from static");
    }
}

struct TH {
}

impl TypeHolder for TH{}

fn main() {
    let t = TH {};
    insert_new::<TH>();
    is_empty::<TH>();
    print_first::<TH>();
}

fn is_empty<T: TypeHolder>() -> bool {
   STACK.with(|stack| {
       stack.borrow().is_empty()
   })
}

fn insert_new<T: TypeHolder>() {
    STACK.with(|stack| {
        stack.borrow_mut().push(Box::new(StackEntry{
            p: Default::default(),
            } as StackEntry<TH>));
    });
}

fn print_first<T: TypeHolder>() {
    STACK.with(|stack| {
        let s = &stack.borrow()[0];
        let s = s.downcast_ref::<StackEntry<TH>>().unwrap();
        s.print();
        s.print();
    });
}

fn foo<T: TypeHolder>() {
    unimplemented!();
    /*lazy_static! {
        static ref INIT: Mutex<ShareMap> = Mutex::new(TypeMap::custom());
    }

    INIT.lock().unwrap().entry::<Key<T>>().or_insert_with(|| {
        println!("Called");
    });*/
}
