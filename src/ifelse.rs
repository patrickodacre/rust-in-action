pub fn do_ifelse()
{
    // being an expression-based language
    // so all expressions return values
    // that can assigned to a variable
    let res = if matches(2, 2) {
        "They match!"
    } else {
        "Different"
    };

    println!("Res :: {:?}", res);
}

fn matches(a: u32, b: u32) -> bool
{
    return a == b;
}
