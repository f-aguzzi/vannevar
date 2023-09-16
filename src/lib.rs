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