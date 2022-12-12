//takes a reference to a String, returns nothing and prints
// whether the contents of the String is plural or singular.
pub fn inspect(arg: &String) {
    if arg.ends_with("s") {
        println!("The provided argument ends with an s.")
    } else {
        println!("The provided argument does not end with an s.")
    }
}

// takes a *mutable* reference to a String and adds an
// "s" to the String if it doesn't already end with "s".
pub fn change(arg: &mut String) {


    if !arg.ends_with("s") {
        arg.push_str("s")
    }
}
// accepts ownership of (consumes) a String and returns a bool indicating
// whether or not the String both starts with a "b" AND contains an "a".
pub fn eat(arg: String) -> bool{
    arg.starts_with("b") && arg.contains("a")
}
// Takes a mutable reference to a String and ignores what is in the string
// and replaces the contents of the string with the String "sparkly".
pub fn bedazzle(arg: &mut String) -> &mut String {
    *arg = arg.replace(&*arg, "sparkly");
    arg
}