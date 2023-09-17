mod lib;
fn main() {
    println!("Hello, world!");

    let test = lib::Journal::todays_journal();
    println!("{}", test.date);
}
