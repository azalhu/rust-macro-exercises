use make_public_macro::{delete, make_public, public_alt_parsed_cursor};

#[derive(Debug)]
#[public_alt_parsed_cursor]
struct Example1 {}

#[derive(Debug)]
#[public_alt_parsed_cursor]
struct Example2 {
    first: String,
    pub second: u32,
    //third: (), // () not an Ident (see public_alt_parsed)
    third: i8,
}

#[delete]
struct EmptyStruct {}

#[make_public]
struct Ex1_1;

#[make_public]
pub struct Ex1_2;

#[make_public]
struct Ex2_1 {}

#[make_public]
pub struct Ex2_2 {}

#[make_public]
struct Ex2_3 {
    f: u8,
    pub s: i8,
}

#[make_public]
struct Ex3_1();

#[make_public]
pub struct Ex3_2();

#[make_public]
struct Ex3_3(u8, pub i8);

#[make_public]
enum Ex4_1 {}

#[make_public]
pub enum Ex4_2 {}

#[make_public]
enum Ex4_3 {
    A,
    B(u8),
}

#[make_public]
union Ex5_1 {
    a: u8,
    pub b: i8,
}

#[make_public]
pub union Ex5_2 {
    x: u8,
    pub y: i8,
}

#[make_public(HEY)]
#[derive(Debug)]
#[allow(dead_code)]
struct Ex6_1;

impl Ex6_1 {
    fn boom(&self) -> () {
        ()
    }
}

fn main() {
    let e1 = Example1 {};
    let e2 = Example2 {
        first: "GGGG".to_string(),
        second: 4,
        //third: (), // () not an Ident (see public_alt_parsed)
        third: -2,
    };
    println!("Hello, world! {:?} / {:?}", e1, e2);
}
