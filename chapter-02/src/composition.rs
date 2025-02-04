fn add_one(n: i32) -> i32 {
    n + 1
}

fn stringify(n: i32) -> String {
    n.to_string()
}

fn prefix_with(prefix: &str) -> impl Fn(String) -> String + '_ {
    move |x| format!("{prefix}{x}")
}

fn compose_two<FIRST, SECOND, THIRD, F, G>(f: F, g: G) -> impl Fn(FIRST) -> THIRD
where
    F: Fn(FIRST) -> SECOND,
    G: Fn(SECOND) -> THIRD,
{
    // move |x| g(f(x))
    move |first| {
        let second = f(first);
        let third = g(second);
        third
    }
}

macro_rules! compose {
    ($last:expr) => ( $last );
    ($head:expr => $($tail:expr)=>+) => (
        compose_two($head, compose!($($tail)=>+))
    );
}

pub fn composition() {
    let two_composed_function =
        compose_two(compose_two(add_one, stringify), prefix_with("Result: "));
    println!("{}", two_composed_function(9));
    let two_composed_function_alt =
        compose_two(add_one, compose_two(stringify, prefix_with("Result: ")));
    println!("{}", two_composed_function_alt(9));
    let composed = compose!(add_one => stringify => prefix_with("Result: "));
    println!("{}", composed(9))
}
