#[macro_use]
extern crate hello_world_macro;

#[derive(HelloVanilla, HelloSyn, HelloVenial)]
struct Example;

#[derive(HelloVanilla, HelloSyn, HelloVenial)]
enum Pet {
    Cat,
}

#[derive(HelloVanilla, HelloSyn, HelloVenial)]
union Tag {
    x: i32,
    y: u16,
}

fn main() {
    let e = Example;
    e.hello_world_vanilla();
    e.hello_world_syn();
    e.hello_world_venial();

    let p = Pet::Cat;
    p.hello_world_vanilla();
    p.hello_world_syn();
    e.hello_world_venial();

    let t = Tag { x: -4 };
    t.hello_world_vanilla();
    t.hello_world_syn();
    e.hello_world_venial();
    println!("{:?}", unsafe { t.y });
}
