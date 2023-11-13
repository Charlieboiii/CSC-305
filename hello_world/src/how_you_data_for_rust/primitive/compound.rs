pub(crate) fn age(){
    println!("i am 18 years old");
}
pub(crate)fn tuple_example(){
    fn main() {
        // Creating a tuple
        let person: (String, i32, f64) = ("Alice".to_string(), 30, 5.8);
    
        // Accessing elements of the tuple
        let name = person.0;
        let age = person.1;
        let height = person.2;}
}
pub(crate) fn array_example(){
    // Creating an array of integers
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Accessing elements of the array
    let first_number = numbers[0];
    let second_number = numbers[1];

    // Printing the array elements
    println!("First number: {}", first_number);
    println!("Second number: {}", second_number);

    // Iterating through the array
    for number in numbers.iter() {
        println!("Number: {}", number);
    }

    // Initializing an array with the same value
    let zeros = [0; 3]; // Creates an array of three zeros: [0, 0, 0]

    // Printing the array of zeros
    println!("Zeros: {:?}", zeros);
}
pub(crate) fn scalar_example() {
    // Create an array of integers
    let numbers = [1, 2, 3, 4, 5];

    // Create a slice of the array
    let slice: &[i32] = &numbers[1..4]; // This creates a slice of elements at indices 1, 2, and 3.

    // Accessing elements of the slice
    let first_element = slice[0];
    let second_element = slice[1];
    let third_element = slice[2];

    // Printing the slice elements
    println!("First element: {}", first_element);
    println!("Second element: {}", second_element);
    println!("Third element: {}", third_element);

    // Iterating through the slice
    for element in slice.iter() {
        println!("Element: {}", element);
    }
}
    
