#[derive(Debug, Clone, Copy)]
struct SomeStruct {
    num: i32,
}

// for lifetime #2 example
#[derive(Debug)]
struct RefStruct<'a> {
    // error[E0106]: missing lifetime specifier
    num_ref: &'a i32 // ref to i32
}

fn print_some_struct(the_struct: SomeStruct) {
    println!("{:?}", the_struct);
}

fn print_some_struct_by_ref(the_struct: &SomeStruct) {
    println!("{:?}", the_struct);
}

fn mutate_struct(the_struct: &mut SomeStruct) {
    the_struct.num = 5;
}

// error[E0106]: missing lifetime specifier
// Add 'a lifetime to show return has same return.
fn max<'a>(a: &'a SomeStruct, b: &'a SomeStruct) -> &'a SomeStruct {
     if a.num > b.num { a } else { b }
}

fn main() {
// References
    let mut some_struct: SomeStruct = SomeStruct {num:3};
    // let struct_ref: &SomeStruct = &some_struct;
    print_some_struct(some_struct.clone());

    // Copy or changing ownership.
    print_some_struct(some_struct); // implicit Copy

    // error[E0502]: cannot borrow `some_struct` as mutable 
    // because it is also borrowed as immutable (line 34, 43)
    mutate_struct(&mut some_struct); // no immutable refs at this point!
    // print_some_struct_by_ref(struct_ref);
    print_some_struct_by_ref(&mut some_struct);

// LifeTimes
    let max_num: &SomeStruct;
    let other_struct: SomeStruct = SomeStruct { num: 7};
    { // This closure limits of lifetime of other_struct
        // error[E0597]: `other_struct` does not live long enough
        // let other_struct: SomeStruct = SomeStruct { num: 7};
        max_num = max( &some_struct, &other_struct);
    } // to here

    print_some_struct_by_ref(max_num);
    // lifetimes are only relevant for references
    // the don't cahnge the lifetimes of the parameters
    // they are often inferred

    let ref_struct: RefStruct;
    let num: i32 = 10;
    { // new closure
        // let num: i32 = 10;
        // error[E0597]: `num` does not live long enough

        ref_struct = RefStruct{num_ref: &num };
    } // because goes out of scope here!

    println!("Ref: {:?}", ref_struct);
}
