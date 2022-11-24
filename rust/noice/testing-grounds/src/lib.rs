pub fn big_test() {
    let mut noice = "hello".to_string();
    println!("hello {}", noice);
    get_stuff(&mut noice);
    get_stuff(&mut noice);
}

pub fn get_stuff<'a>(n: &'a mut String) {
    hoho(n);
}

pub fn hoho<'a>(n: &'a mut String) {
    *n = "cool".to_string();
    drop(n);
}

#[cfg(test)]
mod tests {}
