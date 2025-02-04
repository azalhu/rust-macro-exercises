use std::ops::Add;

macro_rules! generate_get_value {
    ($struct_type:ident, String) => {
        generate_get_value!($struct_type, str);
    };
    ($struct_type:ident, $value_type:ty) => {
        impl $struct_type {
            #[must_use]
            pub fn get_value(&self) -> &$value_type {
                &self.value
            }
        }
    };
}

macro_rules! generate_impl_from {
    ($struct_type:ident, String) => {
        impl From<&str> for $struct_type {
            fn from(value: &str) -> Self {
                Self {
                    value: value.to_string(),
                }
            }
        }

        impl From<String> for $struct_type {
            fn from(value: String) -> Self {
                Self { value }
            }
        }
    };
    ($struct_type:ident, $value_type:ty) => {
        impl From<$value_type> for $struct_type {
            fn from(value: $value_type) -> Self {
                Self { value }
            }
        }
    };
}

macro_rules! generate_impl_try_from {
    ($struct_type:ident, String, $predicate:expr, $error_message:literal) => {
        impl TryFrom<&str> for $struct_type {
            type Error = String;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                if $predicate {
                    Ok(Self {
                        value: value.to_string(),
                    })
                } else {
                    Err($error_message)
                }
            }
        }

        impl TryFrom<String> for $struct_type {
            type Error = String;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if $predicate {
                    Ok(Self { value })
                } else {
                    Err($error_message)
                }
            }
        }
    };
    ($struct_type:ident, $value_type:ty, $predicate:expr, $error_message:literal) => {
        impl TryFrom<$value_type> for $struct_type {
            type Error = String;

            fn try_from(value: $value_type) -> Result<Self, Self::Error> {
                if $predicate(value) {
                    Ok(Self { value })
                } else {
                    Err($error_message.to_string())
                }
            }
        }
    };
}

macro_rules! __generate_newtypes_methods_internal {
    ($struct_type:ident, $value_type:tt) => {
        generate_get_value!($struct_type, $value_type);
    };
}

macro_rules! generate_newtypes_methods {
    ($struct_type:ident, $value_type:tt, $predicate:expr, $error_message:literal) => {
        generate_impl_try_from!($struct_type, $value_type, $predicate, $error_message);
        __generate_newtypes_methods_internal!($struct_type, $value_type);
    };
    ($struct_type:ident, $value_type:tt$(,)*) => {
        generate_impl_from!($struct_type, $value_type);
        __generate_newtypes_methods_internal!($struct_type, $value_type);
    };
}

macro_rules! generate_name {
    ($struct_type:ident, $default_value:expr) => {
        #[derive(Debug)]
        struct $struct_type {
            value: String,
        }

        impl $struct_type {
            #[must_use]
            pub fn new(name: &str) -> Result<Self, String> {
                if name.len() < 2 {
                    Err("Name should be at least two characters".to_string())
                } else {
                    Ok(Self::from(name))
                }
            }
        }

        impl Default for $struct_type {
            fn default() -> Self {
                Self {
                    value: $default_value.to_string(),
                }
            }
        }

        generate_newtypes_methods!($struct_type, String,);
    };
}

#[derive(Debug)]
struct Age {
    value: i32,
}

impl Age {
    pub fn new(age: i32) -> Result<Self, String> {
        Ok(age.try_into()?)
    }
}

impl Default for Age {
    fn default() -> Self {
        Self { value: 18 }
    }
}

#[derive(Debug)]
struct Pay {
    value: i32,
}

impl Pay {
    pub fn new(pay: i32) -> Result<Self, String> {
        Ok(pay.try_into()?)
    }
}

impl Default for Pay {
    fn default() -> Self {
        Self { value: 10 }
    }
}

impl Add<i32> for Pay {
    type Output = Result<Self, String>;

    fn add(self, other: i32) -> Self::Output {
        Self::new(self.value + other)
    }
}

generate_name!(FirstName, "Bob");
generate_name!(LastName, "Jylland");
generate_newtypes_methods!(
    Age,
    i32,
    |age| (0..150).contains(&age),
    "Age should be positive and less than 150y"
);
generate_newtypes_methods!(Pay, i32, |pay| pay >= 0, "Pay should be positive");

#[must_use]
fn calculate_raise(
    first_name: FirstName,
    _last_name: LastName,
    _age: Age,
    current_pay: Pay,
) -> Result<Pay, String> {
    let first_name = first_name.get_value();
    if first_name == "Sam" {
        current_pay + 1000
    } else {
        Ok(current_pay)
    }
}

macro_rules! new_or_default {
    ($struct_type:ident, $new_value:expr) => {
        $struct_type::new($new_value).unwrap_or_else(|e| {
            println!("{}: {e}", stringify!($struct_type));
            $struct_type::default()
        })
    };
}

macro_rules! calculate_raise {
    ($first_name:expr, $last_name:expr, $age:expr, $pay:expr) => {
        calculate_raise(
            new_or_default!(FirstName, $first_name),
            new_or_default!(LastName, $last_name),
            new_or_default!(Age, $age),
            new_or_default!(Pay, $pay),
        )
    };
}

pub fn pay_raise() {
    let first_raise = calculate_raise!("Smith", "Sam", 20, 1000);
    println!("{first_raise:?}");

    let second_raise = calculate_raise!("Sam", "Smith", 1000, 20);
    println!("{second_raise:?}");
}
