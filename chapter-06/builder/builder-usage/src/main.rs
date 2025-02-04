use builder_macro::Builder;

#[derive(Builder)]
pub struct Gleipnir {}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use builder_macro::Builder;

    #[test]
    fn should_generate_builder_for_struct_with_no_fields() {
        #[derive(Builder)]
        pub struct ExampleStructNoFields {}

        let _: ExampleStructNoFields = ExampleStructNoFields::builder().build();
    }
}
