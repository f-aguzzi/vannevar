use crate::lib::parse_trails;

mod lib;
fn main() {
    println!("Hello, world!");

    let test = parse_trails(String::from("Vitellone"), "Vitellorzo\n---\n[cane]\n(descrizione di cane)\n->\n[gatto]\n(descrizione di gatto)\n->");
    println!("{:?}", test);
}
