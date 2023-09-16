use std::{fs, clone};

struct Note{
    title: String,
    text: String,
}

impl Note {
    fn new(t: String) -> Note {
        Note {title: t, text: String::new() }
    }
}

fn load_database() -> Result<Vec<Note>, std::io::Error> {

    let files_list = match fs::read_dir("..") {
        Err(e) => return Err(e),
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

    let notes = files_list.map(|p| {
        fs::read(p)
    })
    .map(|f| {
        match f {
            Err(e) => None,
            Ok(file) => Some(file)
        }
    })
    .flatten();

    Ok( Vec::new() )
}