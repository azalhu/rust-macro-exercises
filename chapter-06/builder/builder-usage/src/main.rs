use builder_macro::Builder;

#[derive(Builder)]
pub struct Gleipnir {}

#[derive(Debug)]
enum Expr<'a> {
    Val(i32),
    Div(&'a Expr<'a>, &'a Expr<'a>),
}

impl Expr<'_> {
    fn eval(&self) -> Result<f64, i32> {
        match self {
            Self::Val(i) => Ok(*i as f64),
            Self::Div(_, Self::Val(d)) if *d == 0 => Err(-1),
            Self::Div(n, d) => Ok(n.eval()? / d.eval()?),
        }
    }
}

fn main() {
    let expr = Expr::Div(&Expr::Val(1), &Expr::Val(3));
    println!("{:?}", expr.eval());
    println!("Hello, world!");
    let e1: Expr<'_> = Expr::Val(1);
    let e2: Expr<'_> = e1.into();
    println!("{:?}", e2);
}

#[cfg(test)]
mod tests {
    use builder_macro::Builder;

    #[test]
    fn should_generate_builder_for_struct_with_no_properties() {
        #[derive(Builder)]
        struct ExampleStructNoFields {}

        let _: ExampleStructNoFields = ExampleStructNoFields::builder().build();
    }

    #[test]
    fn should_generate_builder_for_struct_with_one_property() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
        }

        let gleipnir = Gleipnir::builder()
            .roots_of("mountains".to_string())
            .build();

        assert_eq!(gleipnir.roots_of, "mountains");
    }

    #[test]
    fn should_generate_builder_for_struct_with_two_properties() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
            breath_of_a_fish: u8,
        }

        let gleipnir_builder = Gleipnir::builder()
            .roots_of("mountains".to_string())
            .breath_of_a_fish(1);
        let gleipnir = gleipnir_builder.build();

        assert_eq!(gleipnir.roots_of, "mountains");
        assert_eq!(gleipnir.breath_of_a_fish, 1);
    }

    #[test]
    fn should_generate_builder_for_struct_with_multiple_properties() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
            breath_of_a_fish: u8,
            other_necessities: Vec<String>,
        }

        let gleipnir_builder = Gleipnir::builder()
            .roots_of("mountains".to_string())
            .breath_of_a_fish(1)
            .other_necessities(vec![
                "sound of a cat's footsteps".to_string(),
                "beard of a woman".to_string(),
                "spittle of a bird".to_string(),
            ]);
        let gleipnir = gleipnir_builder.build();

        assert_eq!(gleipnir.roots_of, "mountains");
        assert_eq!(gleipnir.breath_of_a_fish, 1);
        assert_eq!(gleipnir.other_necessities.len(), 3);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_field_is_missing() {
        #[derive(Builder)]
        struct Gleipnir {
            _roots_of: String,
        }

        _ = Gleipnir::builder().build();
    }
}
