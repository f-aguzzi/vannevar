use std::{fs, error::Error, fmt::Debug};

use chrono::Datelike;

struct Note {
    title: String,
    text: String,
}

impl Note {
    fn new(t: String) -> Note {
        Note {title: t, text: String::new() }
    }
}

pub struct Journal {
    pub date: String,
    description: String,
    pages: Vec<String>,
}

impl Journal {
    pub fn todays_journal() -> Journal {
        Journal {
            date: todays_date(),
            description: String::new(),
            pages: Vec::new() }
    }
}

fn todays_date() -> String {
    let date = chrono::Local::now();

    let date = format!("{}-{:02}-{:02}",
        date.year(),
        date.month(),
        date.day());

    date
}

pub struct Trail {
    description: String,
    hops: Vec<(String, String)>
}

pub struct Model {
    notes: Vec<Note>,
    journal_pages: Vec<Journal>,
    
}

impl Model {

    // FIX ERROR HANDLING
    pub fn new() -> Model {
        Model {
            notes: match load_database("../") {
                Ok(v) => v,
                Err(_) => Vec::new()
            },
            journal_pages: match load_journal("../journal/") {
                Ok(v) => v,
                Err(_) => Vec::new()
            }
        }
    }
}

enum FileError {
    ReadError,
    ProcessError,
    FormatError,
}

fn load_database(path: &str) -> Result<Vec<Note>, FileError> {

    let files_list = match fs::read_dir(path) {
        Err(e) => return Err(FileError::ReadError),
        Ok(dir) => dir
    };

    let files_list = files_list.map(|s| {
        match s {
            Err(e) => None,
            Ok(dir) => Some(dir)
        }
    })
    .flatten()
    .map(|s| {
        Some(s.path())
    })
    .flatten();

    let notes: Vec<_> = files_list.map(|p| {
        match fs::read(p.clone()) {
            Err(_) => None,
            Ok(file) => Some((file, p))
        }
    })
    .flatten()
    .map(|f| {
        let buf = std::str::from_utf8(f.0.as_slice());
        let name = f.1.to_str()?;

        if buf.is_ok() {
            let buf = String::from(buf.unwrap());
            let name = String::from(name);

            return Some(Note {title: name, text: buf})
        }

        None
    })
    .flatten()
    .collect();


    Ok( notes )
    
}

fn load_journal(path: &str) -> Result<Vec<Journal>, FileError> {

    let database = match load_database(path) {
        Ok(d) => d,
        Err(_) => return Err(FileError::ReadError)
    };

    let journal = database.into_iter().map(|n| -> Result<Journal, FileError> {
        
        let parts = n.text.split_once("---");

        match parts {
            Some((desc, list)) => {
                let string_vec: Vec<_> = list.split('\n')
                    .map(|s| {
                        let start_bytes = s.find("[").unwrap_or(0);             
                        let end_bytes = s.find("]").unwrap_or(s.len());

                        &s[start_bytes..end_bytes]
                    })
                    .map(|s| String::from(s))
                    .collect();

                Ok( Journal {
                    date: n.title,
                    description: String::from(desc),
                    pages: string_vec
                })
            }
            None => Err(FileError::FormatError)
        }
    }).collect();
    
    journal
}