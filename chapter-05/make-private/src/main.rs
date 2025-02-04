use make_private_macro::{local, make_private};

make_private! {
    struct Example {
        pub string_value: String,
        number_value: i32,
    }
}

fn main() {
    let e = Example::new("value".to_string(), 2);

    let sv = e.get_string_value();
    let nv = e.get_number_value();

    println!("{sv}: {nv}");

    local!();
    println!("{}", greeting);
}
