fn main() {
    println!("Hello, world!");
    let noice = "hello";
    println!("{noice}");
    test("noice".to_string(), "Woohoo".to_string());
}

fn test(hy: String, cool: String) {
    let superu: Vec<String> = vec!["Hello".to_string(), "bye".to_string()];
    println!("Hello {}, and {}", hy, cool);
}

fn syscall_test() {}
