use std::fs;
use lazy_regex::regex;
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

#[derive(Debug, PartialEq)]
pub struct Trail {
    name: String,
    description: String,
    hops: Vec<(String, String)>
}

pub struct Model {
    current_date: String,
    notes: Vec<Note>,
    journal_pages: Vec<Journal>,
    trails: Vec<Trail>   
}

impl Model {

    // FIX ERROR HANDLING
    pub fn new() -> Model {
        Model {
            current_date: todays_date(),
            notes: match load_database("../") {
                Ok(v) => v,
                Err(_) => Vec::new()
            },
            journal_pages: match load_journal("../journal/") {
                Ok(v) => v,
                Err(_) => Vec::new()
            },
            trails: match load_trails("../trails") {
                Ok(v) => v,
                Err(_) => Vec::new()
            }
        }
    }
}

#[derive(Debug)]
pub enum FileError {
    ReadError,
    EmptyFileError,
    FormatError,
}

#[derive(Debug, PartialEq)]
pub enum TrailError {
    DescriptionError,
    EmptyFileError,
    BodyFormatError,
}


// ADD EMPTY FILE ERROR TYPE
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

// FIX ERROR HANDLING, ADD EMPTY FILE CASE
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

pub fn parse_trails(title: String, trail: &str) -> Result<Trail, TrailError> {
    // Stop execution if file is empty
    match trail.len() {
        0 => return Err(TrailError::EmptyFileError),
        _ => { }
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
            let buf = s.as_str()
                .strip_suffix("\n---");
            match buf {
                Some(s) => String::from(s),
                None => return Err(TrailError::DescriptionError)
            }
        }
        None => return Err(TrailError::DescriptionError)
    };
   
    // Capture and process
    let s: Result<Vec<_>, TrailError> = block_matcher.find_iter(trail)
    .map(|m| m.as_str())
    .map(|x| -> Result<(String, String), TrailError> {

        let link = match link_matcher.find(x) {
            Some(s) => s.as_str(),
            None => return Err(TrailError::BodyFormatError)  
        };
        let description = match description_matcher.find(x) {
            Some(s) => s.as_str(),
            None => return Err(TrailError::BodyFormatError)
        };
        let link = &link[1..link.len() - 1];
        let description = &description[1..description.len() - 1];
        Ok( ( String::from(link), String::from(description) ) )
    })
    .collect();

    match s {
        Ok(t) => Ok( Trail { name: title, description: trail_description, hops: t } ),
        Err(e) => Err(e)
    }
    
}

// FIX ERROR HANDLING
fn load_trails(path: &str) -> Result<Vec<Trail>, FileError> {
    let database = match load_database(path) {
        Ok(d) => d,
        Err(_) => return Err(FileError::ReadError)
    };

    let trails = database.into_iter().map(|n| {
        parse_trails(n.title, &n.text)
    });

    Ok( Vec::new() )
}

#[test]
fn test_parse_trails() {
    // Correctly formatted string
    let title = String::from("Trail title");
    let description = String::from("Trail description.");
    let trail = "Trail description.\n---\n[link 1]\n(description 1)\n->\n[link 2]\n(description 2)\n->";

    let test = parse_trails(title.clone(), trail);

    let hop1 = (String::from("link 1"), String::from("description 1"));
    let hop2 = (String::from("link 2"), String::from("description 2"));
    let hops = vec![hop1, hop2];

    let check = Trail { name: title, description: description, hops: hops};

    println!("{:?}", test);

    assert_eq!(test.unwrap(), check);

    // Incorrectly formatted description: only two dashes
    let title = String::from("Trail title");
    let trail = "Trail description.\n--\n[link 1]\n(description 1)\n->\n[link 2]\n(description 2)\n->";

    let test = parse_trails(title, trail);

    let check = Err(TrailError::DescriptionError);

    println!("{:?}", test);

    assert_eq!(test, check);

    // Incorrectly formatted links and body: missing braces and arrows
    let title = String::from("Trail title");
    let trail = "Trail description.\n---\n[link 1]\n()\n->\n[link 2]\n(description 2)\n-";

    let test = parse_trails(title, trail);

    let check = Err(TrailError::BodyFormatError);

    println!("{:?}", test);

    assert_eq!(test, check);
}
