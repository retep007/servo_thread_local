#[macro_use] extern crate lazy_static;
extern crate typemap;

use std::marker::PhantomData;
use typemap::{ShareMap, TypeMap};
use std::cell::RefCell;

thread_local!(static STACK: RefCell<Vec<ShareMap>> = RefCell::new(Vec::new()));

trait TypeHolder: 'static + Sync + Send {}

#[derive(Debug, PartialEq)]
struct StackEntry<TH: TypeHolder> {
    p: PhantomData<TH>
}

impl<T: TypeHolder> typemap::Key for StackEntry<T> {
     type Value = StackEntry<TH>;
}

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
        let mut map = TypeMap::custom();
        map.insert::<StackEntry<T>>(StackEntry{p: Default::default()});
        stack.borrow_mut().push(map);
    });
}

fn print_first<T: TypeHolder>() {
    STACK.with(|stack| {
        (*stack.borrow()[0].get::<StackEntry<TH>>().unwrap()).print();
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
