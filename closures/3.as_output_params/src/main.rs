fn do_twice<F>(mut f: F) where F: FnMut() {
    f();
    f();
}

fn consume_with_relish<F: FnOnce() -> String>(f: F) -> String {
    // `func` consumes its captured variables, so it cannot be run more
    // than once.
    let res = f();
    res
}

fn call_with_one<F>(f: F) -> usize where F: Fn(usize) -> usize {
    f(1)
}

fn main() {

    let mut x = 1;
    let add_two_to_x = || x+= 2;
    do_twice(add_two_to_x);
    println!("add 2 twice: {}", x);

    let mut x = String::from("x");
    let consume_and_return_x =  || {
        x.push_str("oee");
        x
    };
    let consumed = consume_with_relish(consume_and_return_x);
    println!("Consumed: {}", consumed);

    let double = |x| x*2;

    //Instances of Fn can be called repeatedly without mutating state.
    println!("double is: {}", call_with_one(double));
    println!("double is: {}", call_with_one(double));

}
