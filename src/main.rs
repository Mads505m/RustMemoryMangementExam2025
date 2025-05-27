fn main() {
    let mut s = String::from("Borrow Checker");

    let r1 = &s;  // immutable borrow
    let r2 = &s;  // endnu en immutable borrow


    println!("{}, {}", r1, r2);
    let r3 = &mut s;  // mutable borrow mens r1 og r2 ikke har flere referencer

    println!("Main starter");
    a();
    println!("Main slutter");
    let result = make_string();
    println!("{}", result); // virker fint!


    let s1 = String::from("Hej");
    let s2 = s1;
    // Fejl: s1 er ikke længere gyldig
    println!("{}", s1);

    let mut x = 5;


fn a() {
    println!("a starter");
    b();
    println!("a slutter");
}

fn b() {
    println!("b starter");
    println!("b slutter");
}

fn make_string() -> String {
    let s = String::from("hello"); // heap-allokeret
    s // ejerskab flyttes ud af funktionen
}

fn Lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
    }