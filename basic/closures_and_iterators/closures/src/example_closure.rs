pub fn ex_closure() {
    let example_closure = |x| x;
    let _s = example_closure(String::from("hello"));
    let _n = example_closure(5.to_string());
}
