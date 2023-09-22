use crate::lib::{Journal, Model, Note, Trail, load_note};
use crate::view::*;

pub enum CurrentPage {
    StartPage,
    MainMenu,
    CreateNewJournal,
    JournalView,
    JournalEditDescription,
    JournalAddLink,
    SelectLink(Vec<String>),
    NoteView,
    NoteEdit,
    TrailView,
    TrailEditDescription,
    TrailEditHop
}
pub struct Controller {
    model: Model,
    current_page: CurrentPage
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            model: Model::new(),
            current_page: CurrentPage::StartPage
        }
    }
    pub fn execute(&mut self) {
        while true {
            match &self.current_page {
                CurrentPage::StartPage => {
                    start_page();
                    self.current_page = match self.model.journal_page.date.len() {
                        0 => CurrentPage::CreateNewJournal,
                        _ => CurrentPage::JournalView
                    }
                },
                CurrentPage::MainMenu => {
                    match display_menu() {
                        _ => {}
                    }
                },
                CurrentPage::CreateNewJournal => {
                    match select_create_journal() {
                        true => {
                            self.model.journal_page = Journal::todays_journal();
                            self.current_page = CurrentPage::JournalView;
                        },
                        false => return
                    }
                },
                CurrentPage::JournalView => {
                    match display_journal(&self.model.journal_page) {
                        Message::EditDescription => self.current_page = CurrentPage::JournalEditDescription,
                        Message::EditLinks => self.current_page = CurrentPage::JournalAddLink,
                        Message::Menu => self.current_page = CurrentPage::MainMenu,
                        Message::SelectLinks => self.current_page = CurrentPage::SelectLink(self.model.journal_page.pages.to_owned()),
                        Message::Exit => break,
                        _ => {}
                    }
                },
                CurrentPage::JournalEditDescription => {
                    self.model.journal_page.description = edit_journal_description(&self.model.journal_page.description);
                    self.current_page = CurrentPage::JournalView;
                },
                CurrentPage::JournalAddLink => {},
                CurrentPage::SelectLink(v) => {
                    match link_menu(v) {
                        Message::Exit => break,
                        Message::Back => self.current_page = CurrentPage::JournalView,
                        Message::GotoLink(l) => {
                            let index: usize = str::parse(&l).unwrap();
                            match load_note(v.get(index).unwrap()) {
                                Ok(n) => {
                                    self.model.note = n;
                                    self.current_page = CurrentPage::NoteView
                                }
                                Err(_) => todo!()
                            }
                        },
                        _ => {}
                    }
                },
                CurrentPage::NoteView => {},
                CurrentPage::NoteEdit => {
                    let note_text = edit_note(&self.model.note.text);
                    self.model.note = Note::from_str(&self.model.note.text, note_text);
                    self.model.note.save();
                    todo!();
                    // add to journal
                    // parse links?
                },
                CurrentPage::TrailView => {},
                CurrentPage::TrailEditDescription => {},
                CurrentPage::TrailEditHop => {}
            }
        }
        
    }
}
