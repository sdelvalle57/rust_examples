#[derive(Debug, Clone, Copy)]
struct Oee {
    sisas: i32,
    epa: bool
}

#[derive(Debug)]
struct OeeNoCopy {
    sisas: i32,
    epa: bool
}

fn main() {

    //types such as u8, u16, ...., , bool, f8, ..., char, tuples like (i32, f16) not (i32, String) 
    //implement Copy Trait by default
    let j = 56;
    print_u32(j);
    println!("u32 in main {}", j);


    //for structs a copy trait has to be implemented
    let x = Oee {
        sisas: 32,
        epa: true
    };
    print_struct(x);
    println!("struct in main{:?}", x);

    //this struct does not implement copy trait, 
    //so it has to be borrowed id it is going to be used 
    //in the future
    let y = OeeNoCopy {
        sisas: 64,
        epa: false
    };
    print_struct_borrow(&y);
    println!("struct borrow in main{:?}", y);

    //When we use the reference ( & ) operator, 
    //we get the value’s memory address. To get the actual value at the address, 
    //we use the dereferencing operator ( * ).
    let mut z = OeeNoCopy {
        sisas: 8,
        epa: true
    };
    z.sisas = 9;
    print_struct_borrow_mut(&mut z);
    println!("struct borrow mut after mut {:?}", z);

    //slices
    let mut num = [10, 20, 30, 40, 50];
    slice_num(&mut num);
    println!("Slice after: {:?}", num);


    //dereferencing
    let a = 5;
    let b = Box::new(a);

    if a == *b {
        println!("Equal");
    } else {
        println!("Not equal");
    }


}

fn print_struct_borrow(y: &OeeNoCopy) {
    println!("struct borrow {:?}", y);
}

fn print_struct(x: Oee) {
    println!("struct {:?}", x);
}

fn print_u32(val: u32) {
    println!("u32 {}", val);
}

//In the function definition, we use the dereferencing operator to get the 
//value at the memory address of the parameter and mutate it
fn print_struct_borrow_mut(z: &mut OeeNoCopy) {
    println!("struct borrow mut before mut {:?}", *z);
    (*z).epa = false;
}

fn slice_num(slice:&mut [i32]) {

    // slice before mutation
    println!("Slice before: {:?}", slice);

    // mutate a slice value
    slice[0] = 5;

    // slice after mutation
}