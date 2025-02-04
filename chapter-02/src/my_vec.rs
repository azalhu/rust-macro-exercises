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
    ($($x:expr),+) => (
        vec![$($x),+]
    );
}

pub fn my_vec() {
    //trace_macros!(true);
    let _empty: Vec<u8> = my_vec!();
    let _also_empty: Vec<i32> = my_vec!(make an empty vec);
    let _unit = my_vec!(unit);
    let _single: Vec<u8> = my_vec!(3);
    let _multiple: Vec<u8> = my_vec!(3, 5);
    let _bools = my_vec!(1 < 2, 1 > 2);
    //trace_macros!(false);
}
