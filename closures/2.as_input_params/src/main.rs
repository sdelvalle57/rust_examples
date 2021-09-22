
fn apply<T>(t: T) where T: FnOnce() {
    t();
}

//with where
fn apply_to_3<T>(t: T) -> i32 where T: Fn(i32) -> i32 {
    t(3)
}

//or
fn apply_to_4<T: Fn(i32) -> i32>(t: T) -> i32 {
    t(4)
}

fn apply_to_6<T>( t: T) -> i32 where T: Fn(i32) -> i32 {
    t(3)
}


fn call_me<T: Fn()>(t: T) {
    t();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn main() {

    let greeting = "hello";

    let mut farewell = String::from("goodbye");

    let diary = || {
        println!("I said {}", greeting);
        farewell.push_str("!!!");
        println!("The I screamed {}", farewell);
        println!("sleep");
    };

    
    // cannot borrow `farewell` as immutable because it is also borrowed as mutable
    // immutable borrow occurs in apply(diary)
    // println!("The I screamed {}", farewell);

    //cant be called twice bcause diary farewell is already consumed
    // apply(diary);

    apply(diary);
    
    let double = |x| 2*x;
    let triple = |x: i32| 5*x; 


    println!("apply double to 3: {}", apply_to_3(double));
    println!("double of 2: {}", double(2));
    println!("double of 4: {}", apply_to_4(double));
    println!("triple of 5: {}", apply_to_6(triple));

    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);

}

