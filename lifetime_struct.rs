//So the generic lifetime annotation, ensure that the struct is killed immediately after the referense string is killed.
//This is to avoid a dangling pointer. Referencing a variable that does not exits.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&'a self, announcement: &'a str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel: String = String::from("Call me Ishmael, Some years ago.....");

    let first_sentence: &str = novel.split('.').next().expect("Could not find .");
    let i: ImportantExcerpt = ImportantExcerpt {
        part: first_sentence,
    };

    // novel = String::from("asdfasdfasf"); This would throw an error cos novel has already been borrowed
    // println!("{}", i.part);
}

//life time of argument passed are called input lifetimes.
//lifetime of return values are called output lifetimes.

/*
Life time Rules
1.	Each parameter that is a reference gets is own lifetime parameter
2.	If there is exactly one input lifetime parameter, that lifetime is assigned to all the output lifetime paramters.
3.	If there are multiple input lifetime parameters, but the one of the is &self or &mut self the lifetime of self is assigned to all  the output lifetime parameter

*/
