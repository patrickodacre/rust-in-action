pub fn run()
{
    do_string_types();
    // do_quote_iteration();
}

fn do_string_types()
{
    // static string
    let s = "Testing 123";
    // borrow to make a string slice (&str)
    print_string(&s);

    // owned string
    let s = String::from("Testing 456");
    // you can also borrow these to make &str
    print_string(&s);
}

fn print_string(s: &str)
{
    println!("{:?}", s);
}

fn do_quote_iteration()
{
    let search_term = "picture";
    let quote = "\
    Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what? 
It is the same with books. What do we seek through millions of pages?";
    let mut line_num: usize = 1;

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}: {}", line_num, line);
        }
        line_num += 1;
    }

    // enumerate gives us an index (i) that increments after each line
    for (i, line) in quote.lines().enumerate() {
        // if line.contains(search_term) {
        let line_num = i + 1;
        println!("{}: {}", line_num, line);
        // }
    }
}
