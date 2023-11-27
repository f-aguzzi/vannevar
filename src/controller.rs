/// # `controller` module
///
/// This program is built on an MVC pattern, and this module hosts the structs
/// and functions that make up its controller.

use crate::lib::{
    list_files, load_journal_page, load_note, FileError, Journal, Model, Note, Trail, load_trail, TrailError
};
use crate::view::*;

/// ## CurrentPage
///
///  This `enum` is a list of all possible pages that can be displayed to the
///  user. Since the view can be seen as a state machine, these are all of its
///  possible states.


#[derive(Clone)]
pub enum CurrentPage {
    StartPage,  // Initial page of the application
    MainMenu,   // Main menu
    CreateNewJournal,   // Menu to choose whether or not to create a new journal page
    JournalView,    // View mode for the journal page of the current day
    JournalViewReadOnly,    // View mode (read-only) for old journal pages
    JournalEditDescription, // Editor for the journal description
    JournalAddLink, // Interface to create a new note and add it to the journal
    SelectLink(Vec<String>),    // Menu to select a linked page from a list
    NoteView,   // View mode for note pages
    NoteEdit,   // Editor for the text in notes
    SelectCreateTrail,  // Trail menu: select whether to create a new one or open an old one
    CreateNewTrail, // Interface to create a new trail and give it a name
    LoadTrail,  // Menu to select a loadable trail from a list
    TrailView,  // View mode for trail pages
    TrailEditDescription,   // Editor for the description of the currently opened trail
    TrailAddHop,    // Interface to add a link to a note inside of a trail
    SaveError(Box<CurrentPage>),    // Display an error message for failed saving procedures
    UnexpectedError(String),    // Display an error message
}
pub struct Controller {
    model: Model,
    current_page: CurrentPage,
}

/// ## Controller
///
///  This is the state machine that controls the application. It runs on a loop
///  and controls the branching between the possible states, described by the
///  [CurrentPage] `enum`.

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
                CurrentPage::MainMenu => {
                    match display_menu() {
                        MenuOption::Journal => match self.model.journal_page.date.len() {
                            0 => self.current_page = CurrentPage::CreateNewJournal,
                            _ => self.current_page = CurrentPage::JournalView,
                        },
                        MenuOption::LoadJournal => match list_files("../journal") {
                            Ok(l) => match link_menu(&l) {
                                LinkMessage::Exit => break,
                                LinkMessage::Back => self.current_page = CurrentPage::JournalView,
                                LinkMessage::GotoLink(link) => {
                                    match str::parse::<usize>(&link) {
                                            Ok(i) => match l.get(i) {
                                                Some(path) => match load_journal_page(path) {
                                                    Ok(j) => {
                                                        self.model.journal_page = j;
                                                        self.current_page = CurrentPage::JournalViewReadOnly
                                                    }
                                                    Err(e) => match e {
                                                        FileError::ReadError => {
                                                            self.current_page = CurrentPage::UnexpectedError("The selected journal page does not exist.".to_string())
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
                            },
                            Err(_) => {
                                self.current_page = CurrentPage::UnexpectedError(
                                    "Could not find any journal pages.".to_string(),
                                )
                            }
                        },
                        MenuOption::Notes => {
                            todo!()
                        }
                        MenuOption::LoadCreateNote => {
                            todo!()
                        }
                        MenuOption::Trails => {
                            self.current_page = CurrentPage::TrailView;
                        }
                        MenuOption::LoadCreateTrail => {
                            todo!()
                        }
                        MenuOption::Quit => return,
                    }
                }
                CurrentPage::CreateNewJournal => match select_create_journal() {
                    true => {
                        self.model.journal_page = Journal::todays_journal();
                        self.current_page = CurrentPage::JournalView;
                    }
                    false => self.current_page = CurrentPage::MainMenu,
                },
                CurrentPage::JournalView => {
                    match display_journal(&self.model.journal_page) {
                        JournalMessage::EditDescription => {
                            self.current_page = CurrentPage::JournalEditDescription
                        }
                        JournalMessage::EditLinks => {
                            self.current_page = CurrentPage::JournalAddLink
                        }
                        JournalMessage::Menu => self.current_page = CurrentPage::MainMenu,
                        JournalMessage::SelectLinks => {
                            self.current_page =
                                CurrentPage::SelectLink(self.model.journal_page.pages.to_owned())
                        }
                        JournalMessage::Exit => break,
                    };
                    match self.model.journal_page.save() {
                        true => {}
                        false => {
                            self.current_page =
                                CurrentPage::SaveError(Box::new(self.current_page.clone()))
                        }
                    }
                }
                CurrentPage::JournalViewReadOnly => {
                    match display_journal(&self.model.journal_page) {
                        JournalMessage::EditDescription => {}
                        JournalMessage::EditLinks => {}
                        JournalMessage::Menu => match load_journal_page(&self.model.current_date) {
                            Ok(j) => {
                                self.model.journal_page = j;
                                self.current_page = CurrentPage::MainMenu;
                            }
                            Err(_) => self.current_page = CurrentPage::CreateNewJournal,
                        },
                        JournalMessage::SelectLinks => {
                            match load_journal_page(&self.model.current_date) {
                                Ok(j) => {
                                    self.model.journal_page = j;
                                    self.current_page = CurrentPage::SelectLink(
                                        self.model.journal_page.pages.to_owned(),
                                    );
                                }
                                Err(_) => self.current_page = CurrentPage::CreateNewJournal,
                            }
                        }
                        JournalMessage::Exit => break,
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
                    LinkMessage::Exit => break,
                    LinkMessage::Back => self.current_page = CurrentPage::JournalView,
                    LinkMessage::GotoLink(l) => {
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
                },
                CurrentPage::NoteView => {
                    match display_note(&self.model.note) {
                        NoteMessage::Edit => self.current_page = CurrentPage::NoteEdit,
                        NoteMessage::SelectLinks => {
                            self.current_page =
                                CurrentPage::SelectLink(self.model.note.links.to_owned())
                        }
                        NoteMessage::Menu => self.current_page = CurrentPage::MainMenu,
                        NoteMessage::Back => self.current_page = CurrentPage::JournalView,
                        NoteMessage::Exit => {
                            self.current_page = CurrentPage::MainMenu;
                            return;
                        }
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
                CurrentPage::LoadTrail => {
                    match list_files("../journal") {
                        Ok(list) => {
                            match link_menu(&list) {
                                LinkMessage::Exit => {},
                                LinkMessage::Back => {},
                                LinkMessage::GotoLink(link) => {
                                    match str::parse::<usize>(&link) {
                                        Ok(i) => match list.get(i) {
                                            Some(path) => match load_trail(path) {
                                                Ok(t) => {
                                                    self.model.trail = t;
                                                    self.current_page = CurrentPage::TrailView
                                                }
                                                Err(e) => match e {
                                                    TrailError::BodyFormatError => {
                                                        self.current_page = CurrentPage::UnexpectedError(String::from("The body of the selected trail is formatted incorrectly."))
                                                    },
                                                    TrailError::DescriptionError => {
                                                        self.current_page = CurrentPage::UnexpectedError(String::from("The description of the selected trail is formatted incorrectly."))
                                                    },
                                                    TrailError::FileError(fe) => match fe {
                                                        FileError::EmptyFileError => {
                                                            self.model.trail = Trail::new();
                                                            self.model.trail.name = String::from(path);
                                                        },
                                                        FileError::ReadError => {
                                                            self.current_page = CurrentPage::UnexpectedError(String::from("Could not load trail from memory."))
                                                        },
                                                        FileError::FormatError => {
                                                            self.current_page = CurrentPage::UnexpectedError(String::from("The trail file is corrupted."))
                                                        }
                                                    },
                                                },
                                            },
                                            None => {
                                                self.current_page = CurrentPage::UnexpectedError(String::from("The number you entered does not correspond to any valid option."))
                                            }
                                        },
                                        Err(_) => {
                                            self.current_page = CurrentPage::UnexpectedError(String::from("The input you entered is not valid."))
                                        }
                                    };
                                }
                            }
                        },
                        Err(_) => { self.current_page = CurrentPage::UnexpectedError(String::from("Trail loading error."))},
                    }
                }
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
                            TrailMessage::Quit => {
                                break;
                            }
                            TrailMessage::MainMenu => self.current_page = CurrentPage::MainMenu,
                            TrailMessage::RemoveLink => {
                                todo!()
                            }
                            TrailMessage::EditDescription => {
                                self.current_page = CurrentPage::TrailEditDescription
                            }
                        }
                        match self.model.trail.save() {
                            true => {}
                            false => {
                                let boxed_page = Box::new(self.current_page.clone());
                                self.current_page =
                                    CurrentPage::SaveError(boxed_page)
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
                CurrentPage::UnexpectedError(s) => match display_error(s) {
                    DisplayErrorMessage::Menu => self.current_page = CurrentPage::MainMenu,
                    DisplayErrorMessage::Exit => break,
                },
            }
        }

        reset_cursor();
    }
}
