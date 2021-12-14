fn main() {
    println!("Hello, world!");
    let key = rcgen::generate_simple_self_signed(vec!["example.org".into()]).unwrap();
    println!("got it: {:x?}", key.get_key_identifier());
}
