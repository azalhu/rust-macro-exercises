use composing_macro::compose;

fn add_one(n: i32) -> i32 {
    n + 1
}

fn stringify(n: i32) -> String {
    n.to_string()
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let number: i32 = str::parse(args[1].as_str()).unwrap();
    let composed = compose!(add_one.add_one.stringify);
    println!("{:#?}", composed(number));
    println!("{:#?}", composed(2));
    println!("{:#?}", composed(1));
    let composed = compose!(add_one.add_one);
    println!("{:#?}", composed(1));
    let composed = compose!(add_one);
    println!("{:#?}", composed(1));
    let one = 1;
    let composed = compose!(one);
    println!("{:#?}", composed);
}
