fn borrowfail() {
    let mut s = String::from("Borrow Checker");
    let r1 = &s;  // immutable borrow
    let r2 = &s;  // endnu en immutable borrow

    let r3 = &mut s;
    println!("{}, {}", r1, r2);
     // mutable borrow mens r1 og r2  har flere referencer
}

fn borrowwork() {
    let mut s = String::from("Borrow Checker");
    let r1 = &s;  // immutable borrow
    let r2 = &s;  // endnu en immutable borrow
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    
    // mutable borrow mens r1 og r2 ikke har flere referencer
}