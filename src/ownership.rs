fn ownership() {
    let s1 = String::from("Hej");
    let s2 = s1;
    // Fejl: s1 er ikke længere gyldig
    println!("{}", s1);
}
