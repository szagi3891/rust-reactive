#![allow(non_snake_case)]

//https://crates.io/crates/simple-mutex


// struct Context {
// }

// impl Context {
//     pub fn new<T>() -> Value<T> {
//         todo!();
//     }

//     pub fn calculate<K>(fun: Box<dyn Fn() -> K>) -> Computed<K> {
//         todo!();
//     }
// }

// struct Value<T> {
//     value: T,
// }

// impl<T> Value<T> {
//     pub fn toComputed(&self) -> Computed<T> {
//         todo!();
//     }
// }

// struct Computed<T> {
//     get: Box<dyn Fn() -> T>,
// }

// impl<T> Computed<T> {
// }


/*
                                fn() ->             tylko czyste funkcje

Value --> T
Value --> K

fn(
    fn() -> T,          //subskrybcja podczas obliczania
    fn() -> K
) -> R



Value =====>    impl ValueTrait

    get() -> T                                      === pobiera oraz subskrybuje



let context: Context = Context::new();

let a: Value<u64> = context.value(3);
let b: Value<u32> = context.value(5);


lub bardziej mobxowo
Przewaga nad jsem nawet w tej wersji jest taka, że nie da się zrobić cykliczności pomiędzy zmiennymi

let c: Computed<u64> = context.combine2(Box::new(move fn(&mut context) -> u64 {

    let aValue = context.get(a);    //,.get();
    let bValue = context.get(b);    //.get();

    todo!();
}));


context.subscribe(c, fn(value: u64) -> {
    println!("wynik: {}", value);
});

context.set(a, 4);
context.set(b, 55);


Component
    render() {

    }

    Component
        render() {

        }

*/

use std::rc::Rc;
use std::cell::RefCell;

trait Subscriber {
    fn recalculate(&self);
}

trait Observer {
    fn call(&self) -> Vec<Box<dyn Subscriber>>;
}

struct Subscription {
    list: Vec<Box<dyn Observer>>,
}

impl Subscription {
    pub fn new() -> Subscription {
        Subscription {
            list: Vec::new()
        }
    }

    pub fn add(&mut self, observer: Box<dyn Observer>) {
        self.list.push(observer);
    }

    pub fn trigger(&self) -> Vec<Box<dyn Subscriber>> {
        let mut out: Vec<Box<dyn Subscriber>> = Vec::new();

        for item in self.list.iter() {
            let mut subList = item.call();
            out.append(&mut subList);
        }

        out
    }
}

struct Value<T: 'static> {
    value: Rc<T>,
    subscription: Subscription,
}

impl<T: 'static> Value<T> {
    pub fn new(value: T) -> Rc<Value<T>> {
        Rc::new(Value {
            value: Rc::new(value),
            subscription: Subscription::new(),
        })
    }

    pub fn setValue(&mut self, value: T) -> Vec<Box<dyn Subscriber>> {
        self.value = Rc::new(value);
        self.subscription.trigger()
    }

    pub fn getValue(&self) -> Rc<T> {
        self.value.clone()
    }

    pub fn toComputed(self: &Rc<Value<T>>) -> Rc<Computed<T>> {
        let selfClone = self.clone();

        let getValue = Box::new(move || {
            selfClone.getValue()
        });

        Computed::newRc(getValue)
    }
}

struct ComputedValue<T: 'static> {
    isFresh: bool,
    value: Rc<T>,
    subscription: Subscription
}

impl<T: 'static> ComputedValue<T> {
    pub fn new(value: Rc<T>) -> RefCell<ComputedValue<T>> {
        RefCell::new(ComputedValue {
            isFresh: true,
            value,
            subscription: Subscription::new(),
        })
    }
}

struct Computed<T: 'static> {
    getValue: Box<dyn Fn() -> Rc<T> + 'static>,
    refCell: RefCell<ComputedValue<T>>,
}

impl<T: 'static> Computed<T> {
    pub fn new<F: Fn() -> T + 'static>(getValue: Box<F>) -> Rc<Computed<T>> {
        let newGetValue = Box::new(move || {
            Rc::new(getValue())
        });

        let value = newGetValue();
        Rc::new(
            Computed {
                getValue: newGetValue,
                refCell: ComputedValue::new(value),
            }
        )
    }

    pub fn newRc<F: Fn() -> Rc<T> + 'static>(getValue: Box<F>) -> Rc<Computed<T>> {
        let value = getValue();
        Rc::new(
            Computed {
                getValue: getValue,
                refCell: ComputedValue::new(value),
            }
        )
    }

    pub fn from2<A, B, R>(
        a: Rc<Computed<A>>,
        b: Rc<Computed<B>>,
        calculate: fn(Rc<A>, Rc<B>) -> R
    ) -> Rc<Computed<R>> {
        let getValue = Box::new(move || {
            let aValue = a.getValue();
            let bValue = b.getValue();

            calculate(aValue, bValue)
        });

        Computed::new(getValue)
    }

    pub fn getValue(&self) -> Rc<T> {
        let mut inner = self.refCell.borrow_mut();

        if inner.isFresh == false {
            inner.value = self.getValue();
            inner.isFresh = true;
        }

        inner.value.clone()
    }

    pub fn setAsUnfresh(&self) -> Vec<Box<dyn Subscriber>> {
        let mut inner = self.refCell.borrow_mut();
        inner.isFresh = false;
        inner.subscription.trigger()
    }
}

impl<T> Observer for Rc<Computed<T>> {
    fn call(&self) -> Vec<Box<dyn Subscriber>> {
        self.setAsUnfresh()
    }
}

fn main() {
    println!("Hello, world!");

    let a = 3;
}
