use controller::Controller;
use vannevar::Journal;
use view::{link_menu, select_create_journal, start_page};

mod controller;
mod view;

mod lib;
fn main() {
    let mut c = Controller::new();

    c.execute();
}

/* DISPLAY_JOURNAL
let pages = vec![String::from("mucca"), String::from("toro"), String::from("bufalo")];
let j = Journal {
    date: String::from("2023-04-11"),
    description: String::from("Questo è un vitellone di prova. Questo è un vitellone di prova. Questo è un vitellone di prova. Questo è un vitellone di prova. Questo è un vitellone di prova. Questo è un vitellone di prova. Questo è un vitellone di prova. Questo è un vitellone di prova. Questo è un vitellone di prova. Questo è un vitellone di prova. "),
    pages: pages
};
view::display_journal(j);

while 1 == 1 {

    }
*/

/* LINK_MENU
let list = vec![String::from("cane"), String::from("gatto"), String::from("topo")];
let selection = link_menu(list);
println!("{}", selection);
*/
