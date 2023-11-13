pub(crate) struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub(crate) fn name() {
        println!("my name is charles");
    }
}

fn main() {
    // Create an instance of the `Person` struct
    let person1 = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    // Accessing struct fields
    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);

    // Create another instance of the `Person` struct
    let person2 = Person {
        name: "Bob".to_string(),
        age: 25,
    };

    // Accessing struct fields of the second person
    println!("Name: {}", person2.name);
    println!("Age: {}", person2.age);

    // Calling the function associated with the struct
    Person::name();
}
