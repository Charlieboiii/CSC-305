pub(crate) fn height(){
    println!("i am 5'11");
}
pub(crate) fn boolean(){
   let sexy = true;
   if sexy==true{
    println!("dejo is sexy");
   }
   else{
    println!("charles is not sexy");
   }
}
pub(crate)fn numerical_type(){
    //there are two types pf numerical integer and floats
    let age: i32= 12;//defining an integer
    let height: f64=3.45;//defining a float
    //this code checks for the correct age for a person to enter an amusement park 
    if age>10 && height>5.5{
        println!("your are good to get on this mf ride")
    }
    else {
        println!(" fuck outta here dawg")
    }
}
pub(crate) fn textual(){
    //there are two textuals which are strings and char
    let name: &str="charlie"; //initializing string
    let blood_type: char= 'A'; //initializing char
    println!("my name is {} and my bloodtype is {}",name, blood_type);
}
pub(crate) fn never(){
    fn foo() -> ! {
        panic!("This call never returns.");
    }
    
}