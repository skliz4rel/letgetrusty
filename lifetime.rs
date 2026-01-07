fn main() {
    //A static lifetime would exits tru out the lifetime of the program
    let s: &'static str = "This is a static lifetime";

    let r: &i32;

    {
        let x: i32 = 5;
        r = &x;
    }

    /// println!("{}", r); //This is a bug.   ^^ borrowed value does not live long enough. Dangling references
    let string1: String = String::from("3werwerwe");
    {
        let string2: String = String::from("234234");

        let result: &str = longest(string1.as_str(), string2.as_str());
        println!("This is the longest string is {}", result); //smallest lifetime is still valid. So this would print.
    }

    //println!("This is the longest string is {}", result);  This would render an error cos the smallest lifetime of the parameter ends within the braces.
}

//The lifetime of our return value always has to be tied to one of the lifetime of our parameters.
//Becos if we pass back a reference from a parameter passed to a  function
//Also note you cant return a reference from a function. cos it the function it killed it becomes a dangling pointer
//THis generic lifetime annotation wont change the lifetime. It would only show a relation
//So this would ensure that the return value has the lifetime of the parameter to the smallett liftime.
//Reason for this is becos we are passing a reference from the parameter. So the lifetime of the return value is connected to the lifetime of the shortest living reference paramter
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//Also return a owned value instead of a reference. bcos if the function gets killed the reference no longer exits.
//This would cause a dangling pointer. pointers referencing invalid locations in memory.
// fn my_function() -> &str {
//     let result = String::from("This is my string");

//     return result.as_str();
// }

//This transfers ownership
fn my_function() -> String {
    let result = String::from("This is my string");

    return result;
}
