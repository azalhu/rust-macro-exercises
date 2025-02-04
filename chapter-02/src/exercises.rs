pub fn exercises() {
    exercise_1();
    exercise_2_3();
}

// exercise 1

macro_rules! hello_world {
    ($something:ident) => {
        impl $something {
            fn hello_world(&self) {
                println!("Hello world!")
            }
        }
    };
}

struct Example {}
hello_world!(Example);

fn exercise_1() {
    let e = Example {};
    e.hello_world(); // prints "Hello world!"
}

// exercise 2

#[deny(meta_variable_misuse)]
macro_rules! my_vec {
    () => (
        Vec::new()
    );
    (make an empty vec) => (
        vec![444]
    );
    (unit) => (
        ()
    );
    ($x:expr) => (
        vec![$x, $x * $x]
    );
    ($($x:expr),+ $(,)*) => (
        vec![$($x),+]
    );
}

fn exercise_2_3() {
    //trace_macros!(true);
    let _empty: Vec<u8> = my_vec!();
    let _also_empty: Vec<i32> = my_vec!(make an empty vec);
    let _unit = my_vec!(unit);
    let _single: Vec<u8> = my_vec!(3);
    let _multiple: Vec<u8> = my_vec!(3, 5);
    let _bools = my_vec!(1 < 2, 1 > 2);
    let bools = my_vec!(1 < 2, 1 > 2,,,,);
    println!("{bools:?}");
    let _comments: Vec<u8> = my_vec!(3 /*hello*/);
    //trace_macros!(false);
}
