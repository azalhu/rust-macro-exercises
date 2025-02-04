use std::ops::Add;

macro_rules! curry {
    (|$first_arg:ident $(, $arg:ident)*| $function_body:expr) => (
        move |$first_arg| $(move |$arg|)* {
            $function_body
        }
    )
}

fn one() -> u8 {
    1
}

fn two() -> u8 {
    2
}

struct One;

struct Two;

impl From<One> for u8 {
    fn from(_: One) -> u8 {
        1
    }
}

impl From<Two> for u8 {
    fn from(_: Two) -> u8 {
        2
    }
}

impl Add<Two> for One {
    type Output = u8;

    fn add(self, other: Two) -> Self::Output {
        u8::from(self) + u8::from(other)
    }
}

impl Add<One> for Two {
    type Output = u8;

    fn add(self, other: One) -> Self::Output {
        u8::from(self) + u8::from(other)
    }
}

pub fn currying() {
    let curried = curry!(|a, b| a + b);
    let result = curried(1)(2);
    println!("{result}");
    let result = curried(one())(two());
    println!("{result}");
    let curried = curry!(|a, b| a + b);
    let result = curried(One)(Two);
    println!("{result}");
    let result = curry!(|a, b| a + b)(One)(Two);
    println!("{result}");
}
