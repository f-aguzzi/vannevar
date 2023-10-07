use chrono::Datelike;
use lazy_regex::regex;
use std::fs;

pub struct Note {
    pub title: String,
    pub text: String,
    pub links: Vec<String>,
}

impl Note {
    fn new() -> Note {
        Note {
            title: String::new(),
            text: String::new(),
            links: Vec::new(),
        }
    }
    pub fn from_str(name: &str, text: String) -> Note {
        let links_matcher = regex!(r#"\[.+?\]"#m);

        let matched_links: Vec<_> = links_matcher
            .find_iter(text.as_str())
            .map(|m| String::from(m.as_str()))
            .map(|s| {
                let mut q = s;
                q.remove(0);
                q.pop();
                q
            })
            .collect();

        Note {
            title: String::from(name),
            text: text,
            links: matched_links,
        }
    }
    pub fn parse_links(&mut self) {
        let link_matcher = regex!(r"\[(.+?)\]");
        let matches: Vec<_> = link_matcher
            .find_iter(&self.text)
            .map(|s| String::from(s.as_str()))
            .map(|s| {
                let mut q = s;
                q.remove(0);
                q.pop();
                q
            })
            .collect();
        self.links = matches;
    }
    pub fn save(&self) -> bool {
        match fs::write(&self.title, &self.text) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

#[derive(Clone)]
pub struct Journal {
    pub date: String,
    pub description: String,
    pub pages: Vec<String>,
}

impl Journal {
    pub fn new() -> Journal {
        Journal {
            date: String::new(),
            description: String::new(),
            pages: Vec::new(),
        }
    }
    pub fn todays_journal() -> Journal {
        Journal {
            date: todays_date(),
            description: String::new(),
            pages: Vec::new(),
        }
    }
    pub fn from_str(name: &str, text: &str) -> Result<Journal, FileError> {
        let parts = text.split_once("---");
        match parts {
            Some((desc, list)) => {
                let string_vec: Vec<_> = list
                    .split('\n')
                    .map(|s| {
                        let start_bytes = s.find("[").unwrap_or(0);
                        let end_bytes = s.find("]").unwrap_or(s.len());
                        &s[start_bytes..end_bytes]
                    })
                    .map(|s| String::from(s))
                    .collect();

                Ok(Journal {
                    date: String::from(name),
                    description: String::from(desc),
                    pages: string_vec,
                })
            }
            None => Err(FileError::FormatError),
        }
    }
    pub fn save(&self) -> bool {
        let mut stringified_body = format!("{}\n---\n", self.description);
        for l in &self.pages {
            stringified_body.push_str(&format!("[{}]\n", l));
        }
        let path = format!("journal/{}", &self.date);
        match fs::write(path, stringified_body) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

fn todays_date() -> String {
    let date = chrono::Local::now();

    let date = format!("{}-{:02}-{:02}", date.year(), date.month(), date.day());

    date
}

#[derive(Debug, PartialEq)]
pub struct Trail {
    pub name: String,
    pub description: String,
    pub hops: Vec<(String, String)>,
}

impl Trail {
    pub fn new() -> Trail {
        Trail {
            name: String::new(),
            description: String::new(),
            hops: Vec::new(),
        }
    }
    pub fn from_str(title: &str, trail: &str) -> Result<Trail, TrailError> {
        // Stop execution if file is empty
        match trail.len() {
            0 => return Err(TrailError::FileError(FileError::EmptyFileError)),
            _ => {}
        }

        // Precompiled regex for trail processing
        let trail_matcher = regex!(r"(.*?)\n---");
        let block_matcher = regex!(r#"\[(.*?)\]\n\((.*?)\)\n\->$"#m);
        let link_matcher = regex!(r"\[(.+?)\]");
        let description_matcher = regex!(r"\((.+?)\)");

        // Read description. If wrongly formatted, return error.
        let trail_description = match trail_matcher.find(trail) {
            // Remove trailing ---. If wrongly formatted, return error.
            Some(s) => {
                let buf = s.as_str().strip_suffix("\n---");
                match buf {
                    Some(s) => String::from(s),
                    None => return Err(TrailError::DescriptionError),
                }
            }
            None => return Err(TrailError::DescriptionError),
        };

        // Capture and process
        let s: Result<Vec<_>, TrailError> = block_matcher
            .find_iter(trail)
            .map(|m| m.as_str())
            .map(|x| -> Result<(String, String), TrailError> {
                let link = match link_matcher.find(x) {
                    Some(s) => s.as_str(),
                    None => return Err(TrailError::BodyFormatError),
                };
                let description = match description_matcher.find(x) {
                    Some(s) => s.as_str(),
                    None => return Err(TrailError::BodyFormatError),
                };
                let link = &link[1..link.len() - 1];
                let description = &description[1..description.len() - 1];
                Ok((String::from(link), String::from(description)))
            })
            .collect();

        match s {
            Ok(t) => Ok(Trail {
                name: String::from(title),
                description: trail_description,
                hops: t,
            }),
            Err(e) => Err(e),
        }
    }
    pub fn to_str(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str(&format!("{}\n---\n", self.description));

        let hops: String = self
            .hops
            .iter()
            .map(|(link, desc)| format!("[{}]\n({})\n->\n", link, desc))
            .collect();

        buffer.push_str(&hops);

        buffer
    }
    pub fn save(&self) -> bool {
        let path = format!("trails/{}", &self.name);
        match fs::write(path, &self.to_str()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

pub struct Model {
    pub current_date: String,
    pub note: Note,
    pub journal_page: Journal,
    pub trail: Trail,
}

impl Model {
    pub fn new() -> Model {
        let j = match load_journal_page(todays_date().as_str()) {
            Ok(r) => r,
            Err(_) => Journal::new(),
        };

        Model {
            current_date: todays_date(),
            note: Note::new(),
            journal_page: j,
            trail: Trail::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FileError {
    ReadError,
    EmptyFileError,
    FormatError,
}

#[derive(Debug, PartialEq)]
pub enum TrailError {
    DescriptionError,
    FileError(FileError),
    BodyFormatError,
}

// SINGLE PAGE LOADERS

pub fn load_note(path: &str) -> Result<Note, FileError> {
    let file: Vec<u8> = match fs::read(path) {
        Ok(f) => f,
        Err(_) => return Err(FileError::ReadError),
    };

    let file_string = match String::from_utf8(file) {
        Ok(f) => f,
        Err(_) => return Err(FileError::FormatError),
    };

    Ok(Note::from_str(path, file_string))
}

pub fn load_journal_page(path: &str) -> Result<Journal, FileError> {
    let file: Vec<u8> = match fs::read(path) {
        Ok(f) => f,
        Err(_) => return Err(FileError::ReadError),
    };

    let file_string = match String::from_utf8(file) {
        Ok(f) => f,
        Err(_) => return Err(FileError::FormatError),
    };

    Journal::from_str(path, &file_string)
}

// ADD EMPTY FILE ERROR TYPE
pub fn list_files(path: &str) -> Result<Vec<String>, FileError> {
    let files_list = match fs::read_dir(path) {
        Err(e) => return Err(FileError::ReadError),
        Ok(dir) => dir,
    };

    let file_strings: Result<Vec<_>, FileError> = files_list
        .map(|f| match f {
            Ok(s) => match s.file_name().to_str() {
                Some(str) => Ok(String::from(str)),
                None => Err(FileError::ReadError),
            },
            Err(_) => Err(FileError::ReadError),
        })
        .collect();

    file_strings
}

// FIX ERROR HANDLING
fn load_trail(path: &str) -> Result<Trail, TrailError> {
    let file: Vec<u8> = match fs::read(path) {
        Ok(f) => f,
        Err(_) => return Err(TrailError::FileError(FileError::ReadError)),
    };

    let file_string = match String::from_utf8(file) {
        Ok(f) => f,
        Err(_) => return Err(TrailError::FileError(FileError::FormatError)),
    };

    Trail::from_str(path, file_string.as_str())
}
