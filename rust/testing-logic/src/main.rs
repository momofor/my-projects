fn main() {
    // two statements
    let p = false;
    let q = true;
    // exclusive or (xor)
    if (p | q) & !(p & q) {
        println!("xor");
    }
    // and
    if p & q {
        println!("and");
    }
    // or
    if p | q {
        println!("or");
    }
    // not
    if !p {
        println!("not p");
    }
    if !q {
        println!("not q");
    }

    println!("Hello, world!");
}
