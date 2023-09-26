use crate::lib::{load_note, FileError, Journal, Model, Note, Trail};
use crate::view::*;

#[derive(Clone)]
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
    SelectCreateTrail,
    CreateNewTrail,
    LoadTrail,
    TrailView,
    TrailEditDescription,
    TrailAddHop,
    SaveError(Box<CurrentPage>),
}
pub struct Controller {
    model: Model,
    current_page: CurrentPage,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            model: Model::new(),
            current_page: CurrentPage::StartPage,
        }
    }
    pub fn execute(&mut self) {
        loop {
            match &self.current_page {
                CurrentPage::StartPage => {
                    start_page();
                    self.current_page = match self.model.journal_page.date.len() {
                        0 => CurrentPage::CreateNewJournal,
                        _ => CurrentPage::JournalView,
                    }
                }
                CurrentPage::MainMenu => match display_menu() {
                    MenuOption::Journal => match self.model.journal_page.date.len() {
                        0 => self.current_page = CurrentPage::CreateNewJournal,
                        _ => self.current_page = CurrentPage::JournalView,
                    },
                    MenuOption::Notes => {}
                    MenuOption::Trails => {
                        self.current_page = CurrentPage::TrailView;
                    }
                    MenuOption::Quit => return,
                },
                CurrentPage::CreateNewJournal => match select_create_journal() {
                    true => {
                        self.model.journal_page = Journal::todays_journal();
                        self.current_page = CurrentPage::JournalView;
                    }
                    false => self.current_page = CurrentPage::MainMenu,
                },
                CurrentPage::JournalView => {
                    match display_journal(&self.model.journal_page) {
                        Message::EditDescription => {
                            self.current_page = CurrentPage::JournalEditDescription
                        }
                        Message::EditLinks => self.current_page = CurrentPage::JournalAddLink,
                        Message::Menu => self.current_page = CurrentPage::MainMenu,
                        Message::SelectLinks => {
                            self.current_page =
                                CurrentPage::SelectLink(self.model.journal_page.pages.to_owned())
                        }
                        Message::Exit => break,
                        _ => {}
                    };
                    match self.model.journal_page.save() {
                        true => {}
                        false => {
                            self.current_page =
                                CurrentPage::SaveError(Box::new(self.current_page.clone()))
                        }
                    }
                }
                CurrentPage::JournalEditDescription => {
                    self.model.journal_page.description =
                        edit_journal_description(&self.model.journal_page.description);
                    self.current_page = CurrentPage::JournalView;
                }
                CurrentPage::JournalAddLink => {
                    let s = add_journal_link();
                    match s.len() {
                        0 => {}
                        _ => match self.model.journal_page.pages.binary_search(&s) {
                            Ok(_) => {}
                            Err(_) => {
                                self.model.journal_page.pages.push(s);
                            }
                        },
                    }
                    self.current_page = CurrentPage::JournalView
                }
                CurrentPage::SelectLink(v) => match link_menu(v) {
                    Message::Exit => break,
                    Message::Back => self.current_page = CurrentPage::JournalView,
                    Message::GotoLink(l) => {
                        match str::parse::<usize>(&l) {
                            Ok(i) => match v.get(i) {
                                Some(path) => match load_note(path) {
                                    Ok(n) => {
                                        self.model.note = n;
                                        self.current_page = CurrentPage::NoteView
                                    }
                                    Err(e) => match e {
                                        FileError::ReadError => match select_create_note(path) {
                                            true => {
                                                self.model.note =
                                                    Note::from_str(path, String::new());
                                                self.current_page = CurrentPage::NoteView;
                                            }
                                            false => self.current_page = CurrentPage::JournalView,
                                        },
                                        FileError::FormatError => {}
                                        FileError::EmptyFileError => {}
                                    },
                                },
                                None => {}
                            },
                            Err(_) => {}
                        };
                    }
                    _ => {}
                },
                CurrentPage::NoteView => {
                    match display_note(&self.model.note) {
                        Message::Edit => self.current_page = CurrentPage::NoteEdit,
                        Message::SelectLinks => {
                            self.current_page =
                                CurrentPage::SelectLink(self.model.note.links.to_owned())
                        }
                        Message::Menu => self.current_page = CurrentPage::MainMenu,
                        Message::Back => self.current_page = CurrentPage::JournalView,
                        Message::Exit => {
                            self.current_page = CurrentPage::MainMenu;
                            return;
                        }
                        _ => {}
                    }
                    match self.model.note.save() {
                        true => {
                            let title = self.model.note.title.clone();
                            match self.model.journal_page.pages.binary_search(&title) {
                                Ok(_) => {}
                                Err(_) => {
                                    self.model.journal_page.pages.push(title);
                                }
                            }
                            self.model.note.parse_links();
                        }
                        false => {
                            self.current_page =
                                CurrentPage::SaveError(Box::new(self.current_page.clone()))
                        }
                    }
                }
                CurrentPage::NoteEdit => {
                    let note_text = edit_note(&self.model.note.text);
                    self.model.note = Note::from_str(&self.model.note.title, note_text);
                    self.current_page = CurrentPage::NoteView;
                }
                CurrentPage::SelectCreateTrail => match select_create_trail() {
                    CreateTrailMessage::CreateTrail => {
                        self.current_page = CurrentPage::CreateNewTrail
                    }
                    CreateTrailMessage::LoadTrail => self.current_page = CurrentPage::LoadTrail,
                    CreateTrailMessage::ReturnToJournal => {
                        self.current_page = CurrentPage::JournalView
                    }
                },
                CurrentPage::CreateNewTrail => {
                    let s = create_new_trail();
                    match s.len() {
                        0 => {}
                        _ => {
                            self.model.trail = Trail::new();
                            self.model.trail.name = s;
                            self.current_page = CurrentPage::TrailView;
                        }
                    }
                }
                CurrentPage::LoadTrail => {}
                CurrentPage::TrailView => match self.model.trail.name.len() {
                    0 => self.current_page = CurrentPage::SelectCreateTrail,
                    _ => {
                        match display_trail(&self.model.trail) {
                            TrailMessage::AddLink => self.current_page = CurrentPage::TrailAddHop,
                            TrailMessage::SelectLink => {
                                let names = self
                                    .model
                                    .trail
                                    .hops
                                    .clone()
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect();
                                self.current_page = CurrentPage::SelectLink(names)
                            }
                            TrailMessage::Quit => break,
                            TrailMessage::MainMenu => self.current_page = CurrentPage::MainMenu,
                            TrailMessage::RemoveLink => {
                                todo!()
                            }
                            TrailMessage::EditDescription => {
                                self.current_page = CurrentPage::TrailEditDescription
                            }
                        }
                    }
                },
                CurrentPage::TrailEditDescription => {
                    self.model.trail.description =
                        edit_trail_description(&self.model.trail.description);
                    self.current_page = CurrentPage::TrailView;
                }
                CurrentPage::TrailAddHop => {
                    let (name, desc) = add_trail_hop();
                    match name.len() {
                        0 => {}
                        _ => match self
                            .model
                            .trail
                            .hops
                            .binary_search(&(name.clone(), desc.clone()))
                        {
                            Ok(_) => {}
                            Err(_) => {
                                self.model.trail.hops.push((name, desc));
                            }
                        },
                    }
                    self.current_page = CurrentPage::TrailView;
                }
                CurrentPage::SaveError(cp) => {
                    let err = match **cp {
                        CurrentPage::NoteView => "note",
                        CurrentPage::JournalView => "journal page",
                        CurrentPage::TrailView => "trail",
                        _ => " ",
                    };
                    save_error(err);
                    self.current_page = *cp.clone();
                }
            }
        }

        reset_cursor();
    }
}
