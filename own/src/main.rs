fn main() {
    let s = String::from("hello");
    let (s, l) = take_owner(s);
    println!("String: {} Size: {}", s, l);

    let ss = String::from("hello world");
    let l = take_ptr(&ss);

    println!("String: {} Size: {}", ss, l);

    let mut ss = String::from("hello world");
    change(&mut ss);
    let l = take_ptr(&ss);

    println!("String: {} Size: {}", ss, l);

    //Mut + immut
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}

fn change(s: &mut String) {
    s.push_str(" ,world");
}

fn take_owner(s: String) -> (String, usize) {
    let l = s.len();
    (s, l)
}

fn take_ptr(s: &String) -> usize {
    s.len()
}
