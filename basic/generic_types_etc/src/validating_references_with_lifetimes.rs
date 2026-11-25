// Dangling References
pub fn dangling_references() {
    /*
    let r;

        {
            let x = 5; // binding x declared here
            r = &x; // borrowed value does not live long enough because in scope {}
        }
        // x dropped (different scope) here while still borrowed by r

        println!("r: {r}"); // borrow later used here
    */
}
pub fn no_dangling_references() {
    let x = 5; // x never dropped because x have long lifetime in the scope
    let r = &x;
    println!("r: {r}");
}

// Generic Lifetime in Function
// This longest will error since it's expected named lifetime parameter
/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

pub fn generic_lifetime_fn() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
*/

// Lifetime Annotation Syntax
/*
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/

// In Function signatures
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
pub fn lifetime_fn_with_annotation() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}
