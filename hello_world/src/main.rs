mod how_you_data_for_rust {
    pub mod user_defined {
        pub fn name() {
            println!("User-defined name function");
        }
    }

    // Define other modules similarly
    pub mod primitive {
        pub mod compound {
            pub fn age() {
                println!("Compound age function");
            }
            pub fn tuple_example() {
                println!("Compound tuple example function");
            }
            pub fn array_example() {
                println!("Compound array example function");
            }
            pub fn scalar_example() {
                println!("Compound scalar example function");
            }
        }

        pub mod scalar {
            pub fn height() {
                println!("Scalar height function");
            }
            pub fn boolean() {
                println!("Scalar boolean function");
            }
            pub fn numerical_type() {
                println!("Scalar numerical type function");
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    how_you_data_for_rust::user_defined::name();
    how_you_data_for_rust::primitive::compound::age();
    how_you_data_for_rust::primitive::scalar::height();
    how_you_data_for_rust::primitive::scalar::boolean();
    how_you_data_for_rust::primitive::scalar::numerical_type();
    how_you_data_for_rust::primitive::compound::tuple_example();
    how_you_data_for_rust::primitive::compound::array_example();
    how_you_data_for_rust::primitive::compound::scalar_example();
}
