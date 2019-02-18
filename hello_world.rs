fn main() {
    let a = String::from("Hello");
    let b = String::from("World");
    let a2 = &a;
    let b2 = &b;
    let c = myadd(a2,b2);
    println!("{}",a);
    println!("{}",c);
}

fn myadd(alpha:&str,beta:&str) -> String {
    String::from(alpha) + beta
}
