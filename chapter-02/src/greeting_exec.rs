use crate::greeting::base_greeting_fn;

pub fn greeting_exec() {
    trace_macros!(true);
    let _greet = greeting!("Sam", "Heya");
    let _greet_with_default = greeting!("Sam");
    let _greet_with_default_test = greeting!(test "Sam");
    trace_macros!(false);
}
