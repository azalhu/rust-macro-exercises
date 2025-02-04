#![feature(log_syntax)]
#![feature(trace_macros)]

#[macro_use]
pub mod greeting;
pub mod account_transfer;
pub mod composition;
pub mod currying;
pub mod exercises;
pub mod greeting_exec;
pub mod lazy_static;
pub mod my_vec;
pub mod pay_raise;

fn main() {
    my_vec::my_vec();
    greeting_exec::greeting_exec();
    pay_raise::pay_raise();
    account_transfer::account_transfer();
    composition::composition();
    currying::currying();
    exercises::exercises();
}
