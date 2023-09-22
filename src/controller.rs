use crate::lib::{Journal, Model, Note, Trail};
use crate::view::*;

pub struct Controller {
    model: Model,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            model: Model::new(),
        }
    }
    pub fn execute(&mut self) {
        start_page();

        let is_create_journal: bool;

        match self.model.journal_page.date.len() {
            0 => is_create_journal = select_create_journal(),
            _ => is_create_journal = false,
        }

        match is_create_journal {
            true => self.model.journal_page = Journal::todays_journal(),
            false => {}
        }

        display_journal(&self.model.journal_page)
    }
}
