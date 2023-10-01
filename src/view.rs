extern crate termion;

use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, scroll, style, terminal_size};

use std::io::{stdin, stdout, Write};

use crate::lib::{Journal, Note, Trail};

pub fn start_page() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{clear}{cursor}{goto}{red}{bold}VANNEVAR{reset_color}{reset_style}",
        clear = clear::All,
        cursor = cursor::Hide,
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 3,
            terminal_size().unwrap().1 / 2 - 2
        ),
        red = color::Fg(color::Red),
        bold = style::Bold,
        reset_color = color::Fg(color::Reset),
        reset_style = style::Reset
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{white}Press any key to continue.{reset}",
        // Goto the cell.
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 13,
            terminal_size().unwrap().1 / 2 + 2
        ),
        white = color::Fg(color::White),
        reset = color::Fg(color::Reset)
    )
    .unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            _ => break,
        }
    }
}

pub enum JournalMessage {
    EditDescription,
    EditLinks,
    Menu,
    SelectLinks,
    Exit,
}

pub fn display_journal(page: &Journal) -> JournalMessage {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{clear}{cursor}{goto}{red}{bold}{date}{reset_color}{reset_style}",
        clear = clear::All,
        cursor = cursor::Hide,
        goto = cursor::Goto(1, 1),
        red = color::Fg(color::Red),
        bold = style::Bold,
        reset_color = color::Fg(color::Reset),
        reset_style = style::Reset,
        date = page.date
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{bold}Description: {reset_style}",
        goto = cursor::Goto(1, 3),
        bold = style::Bold,
        reset_style = style::Reset,
    )
    .unwrap();

    write!(
        stdout,
        " {goto}{text}",
        goto = cursor::Goto(1, 4),
        text = page.description
    )
    .unwrap();

    let pos: u16 = page.description.len() as u16 / terminal_size().unwrap().0;

    write!(
        stdout,
        "{goto}{bold}Pages created or edited today:{reset_style}{goto}",
        goto = cursor::Goto(1, 6 + pos),
        bold = style::Bold,
        reset_style = style::Reset,
    )
    .unwrap();

    for (i, s) in page.pages.iter().enumerate() {
        write!(
            stdout,
            "{goto}[{link}]",
            goto = cursor::Goto(1, 7 + pos + i as u16),
            link = s
        )
        .unwrap();
    }

    stdout.flush().unwrap();

    for k in stdin.keys() {
        match k.unwrap() {
            Key::Char(c) => match c {
                'd' | 'D' => return JournalMessage::EditDescription,
                'e' | 'E' => return JournalMessage::EditLinks,
                'm' | 'M' => return JournalMessage::Menu,
                'l' | 'L' => return JournalMessage::SelectLinks,
                'q' | 'Q' => return JournalMessage::Exit,
                _ => {}
            },
            _ => {}
        }
    }

    JournalMessage::Exit
}

pub fn select_create_journal() -> bool {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{clear}{goto}The journal page for today does not exist.",
        clear = clear::All,
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 21,
            terminal_size().unwrap().1 / 2
        )
    )
    .unwrap();

    write!(
        stdout,
        "{goto}Create it? (y/n)",
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 8,
            terminal_size().unwrap().1 / 2 + 1
        )
    )
    .unwrap();

    stdout.flush().unwrap();

    let choice: bool = false;

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char(c) => match c {
                'y' | 'Y' => return true,
                'n' | 'N' => return false,
                _ => {}
            },
            _ => {}
        }
    }

    choice
}

pub enum LinkMessage {
    Exit,
    Back,
    GotoLink(String),
}

pub fn link_menu(list: &Vec<String>) -> LinkMessage {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{clear}{cursor}{goto}{red}{bold} Select link: {reset_color}{reset_style}",
        clear = clear::All,
        cursor = cursor::Hide,
        goto = cursor::Goto(terminal_size().unwrap().0 / 2 - 6, 1),
        red = color::Fg(color::Red),
        bold = style::Bold,
        reset_color = color::Fg(color::Reset),
        reset_style = style::Reset,
    )
    .unwrap();

    for (i, x) in list.iter().enumerate() {
        write!(
            stdout,
            "{goto}{bold}{number}. {reset_style}{text}",
            goto = cursor::Goto(1, 2 + i as u16),
            number = i,
            bold = style::Bold,
            reset_style = style::Reset,
            text = x
        )
        .unwrap();
    }

    write!(
        stdout,
        "{goto}---",
        goto = cursor::Goto(1, terminal_size().unwrap().1 - 2)
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{bold}Selection: {reset_style}",
        goto = cursor::Goto(1, terminal_size().unwrap().1 - 1),
        bold = style::Bold,
        reset_style = style::Reset,
    )
    .unwrap();

    stdout.flush().unwrap();

    let mut str_buf = String::new();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char(c) => match c {
                '\n' => break,
                'q' | 'Q' => return LinkMessage::Exit,
                _ => {
                    str_buf.push(c);
                    write!(stdout, "{}", c).unwrap();
                    stdout.flush().unwrap();
                }
            },
            Key::Esc => return LinkMessage::Back,
            _ => {}
        }
    }

    LinkMessage::GotoLink(str_buf)
}

pub enum NoteMessage {
    Edit,
    SelectLinks,
    Menu,
    Exit,
    Back,
}

pub fn display_note(page: &Note) -> NoteMessage {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{clear}{cursor}{goto}{red}{bold}{title}{reset_color}{reset_style}",
        clear = clear::All,
        cursor = cursor::Hide,
        goto = cursor::Goto(1, 1),
        red = color::Fg(color::Red),
        bold = style::Bold,
        reset_color = color::Fg(color::Reset),
        reset_style = style::Reset,
        title = page.title
    )
    .unwrap();

    write!(
        stdout,
        " {goto}{text}",
        goto = cursor::Goto(1, 3),
        text = page.text
    )
    .unwrap();

    stdout.flush().unwrap();

    for k in stdin.keys() {
        match k.unwrap() {
            Key::Char(c) => match c {
                'e' | 'E' => return NoteMessage::Edit,
                'l' | 'L' => return NoteMessage::SelectLinks,
                'm' | 'M' => return NoteMessage::Menu,
                'q' | 'Q' => return NoteMessage::Exit,
                _ => {}
            },
            Key::Esc => return NoteMessage::Back,
            _ => {}
        }
    }

    NoteMessage::Back
}

pub enum TrailMessage {
    SelectLink,
    MainMenu,
    Quit,
    EditDescription,
    AddLink,
    RemoveLink,
}

pub fn display_trail(page: &Trail) -> TrailMessage {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{clear}{cursor}{goto}{red}{bold}{name}{reset_color}{reset_style}",
        clear = clear::All,
        cursor = cursor::Hide,
        goto = cursor::Goto(1, 1),
        red = color::Fg(color::Red),
        bold = style::Bold,
        reset_color = color::Fg(color::Reset),
        reset_style = style::Reset,
        name = page.name
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{bold}Description: {reset_style}",
        goto = cursor::Goto(1, 3),
        bold = style::Bold,
        reset_style = style::Reset,
    )
    .unwrap();

    write!(
        stdout,
        " {goto}{text}",
        goto = cursor::Goto(1, 4),
        text = page.description
    )
    .unwrap();

    stdout.flush().unwrap();

    let mut base_offset = 7 + page.description.chars().count() as u16 / terminal_size().unwrap().0;

    for (i, x) in page.hops.iter().enumerate() {
        let line_number = 3 + (10 + 5 + x.1.len() + x.0.len()) as u16 / terminal_size().unwrap().0;

        write!(
            stdout,
            "{goto}{bold}Hop {number}:{reset_style} {name}",
            goto = cursor::Goto(1, base_offset),
            bold = style::Bold,
            reset_style = style::Reset,
            number = i,
            name = x.0,
        )
        .unwrap();

        write!(
            stdout,
            "{goto}{bold}Description:{reset_style} {desc}",
            goto = cursor::Goto(
                1,
                base_offset + 1 + (x.0.len() as u16) / terminal_size().unwrap().0
            ),
            bold = style::Bold,
            reset_style = style::Reset,
            desc = x.1
        )
        .unwrap();

        base_offset += line_number;
    }

    stdout.flush().unwrap();

    for k in stdin.keys() {
        match k.unwrap() {
            Key::Char(c) => match c {
                'l' | 'L' => return TrailMessage::SelectLink,
                'm' | 'M' => return TrailMessage::MainMenu,
                'q' | 'Q' => return TrailMessage::Quit,
                'd' | 'D' => return TrailMessage::EditDescription,
                'e' | 'E' => return TrailMessage::AddLink,
                'r' | 'R' => return TrailMessage::RemoveLink,
                _ => {}
            },
            Key::Down => {
                write!(stdout, "{}", scroll::Down(1)).unwrap();
                //stdout.flush().unwrap();
            }
            Key::Up => {
                write!(stdout, "{}", scroll::Up(1)).unwrap();
                //stdout.flush().unwrap();
            }
            _ => {}
        }
    }

    TrailMessage::Quit
}

pub enum CreateTrailMessage {
    CreateTrail,
    LoadTrail,
    ReturnToJournal,
}

pub fn select_create_trail() -> CreateTrailMessage {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{clear}{goto}There is no trail currently loaded.",
        clear = clear::All,
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 18,
            terminal_size().unwrap().1 / 2
        )
    )
    .unwrap();

    write!(
        stdout,
        "{goto}(c) Create one.",
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 7,
            terminal_size().unwrap().1 / 2 + 1
        )
    )
    .unwrap();

    write!(
        stdout,
        "{goto}(l) Load one.",
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 6,
            terminal_size().unwrap().1 / 2 + 2
        )
    )
    .unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char(c) => match c {
                'c' | 'C' => return CreateTrailMessage::CreateTrail,
                'l' | 'L' => return CreateTrailMessage::LoadTrail,
                'j' | 'J' => return CreateTrailMessage::ReturnToJournal,
                _ => {}
            },
            _ => {}
        }
    }

    CreateTrailMessage::ReturnToJournal
}

pub enum MenuOption {
    Journal,
    LoadJournal,
    Notes,
    LoadCreateNote,
    Trails,
    LoadCreateTrail,
    Quit,
}

pub fn display_menu() -> MenuOption {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{clear}{cursor}{goto}{red}{bold}MAIN MENU{reset_color}{reset_style}",
        clear = clear::All,
        cursor = cursor::Hide,
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 5,
            terminal_size().unwrap().1 / 2 - 8
        ),
        red = color::Fg(color::Red),
        bold = style::Bold,
        reset_color = color::Fg(color::Reset),
        reset_style = style::Reset
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{white}Select an option and press enter.{reset}",
        // Goto the cell.
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 16,
            terminal_size().unwrap().1 / 2 - 6
        ),
        white = color::Fg(color::White),
        reset = color::Fg(color::Reset)
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{white}(j) Open today's journal.{reset}",
        // Goto the cell.
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 12,
            terminal_size().unwrap().1 / 2 - 4
        ),
        white = color::Fg(color::White),
        reset = color::Fg(color::Reset)
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{white}(o) Open old journal pages.{reset}",
        // Goto the cell.
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 12,
            terminal_size().unwrap().1 / 2 - 2
        ),
        white = color::Fg(color::White),
        reset = color::Fg(color::Reset)
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{white}(n) Open the current note.{reset}",
        // Goto the cell.
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 13,
            terminal_size().unwrap().1 / 2
        ),
        white = color::Fg(color::White),
        reset = color::Fg(color::Reset)
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{white}(N) Create or load a note.{reset}",
        // Goto the cell.
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 13,
            terminal_size().unwrap().1 / 2 + 2
        ),
        white = color::Fg(color::White),
        reset = color::Fg(color::Reset)
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{white}(t) Open the current trail.{reset}",
        // Goto the cell.
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 13,
            terminal_size().unwrap().1 / 2 + 4
        ),
        white = color::Fg(color::White),
        reset = color::Fg(color::Reset)
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{white}(T) Create or load a trail.{reset}",
        // Goto the cell.
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 13,
            terminal_size().unwrap().1 / 2 + 4
        ),
        white = color::Fg(color::White),
        reset = color::Fg(color::Reset)
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{white}(q) Quit.{reset}",
        // Goto the cell.
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 4,
            terminal_size().unwrap().1 / 2 + 6
        ),
        white = color::Fg(color::White),
        reset = color::Fg(color::Reset)
    )
    .unwrap();

    stdout.flush().unwrap();

    for k in stdin.keys() {
        match k.unwrap() {
            Key::Char(c) => match c {
                'j' => return MenuOption::Journal,
                'J' => return MenuOption::LoadJournal,
                'n' => return MenuOption::Notes,
                'N' => return MenuOption::LoadCreateNote,
                't' => return MenuOption::Trails,
                'T' => return MenuOption::LoadCreateTrail,
                'q' | 'Q' => return MenuOption::Quit,
                _ => {}
            },
            Key::Esc => return MenuOption::Quit,
            _ => {}
        }
    }

    MenuOption::Quit
}

pub fn text_editor(text: &String) -> String {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{goto}{clear}{show_cursor}{text}",
        goto = cursor::Goto(1, 1),
        clear = clear::All,
        text = text,
        show_cursor = cursor::Show
    )
    .unwrap();

    stdout.flush().unwrap();

    let mut new_text = text.clone();
    let mut pointer = new_text.len();

    let (mut _cur_x, cur_y) = stdout.cursor_pos().unwrap();

    for k in stdin.keys() {
        match k.unwrap() {
            Key::Char(c) => {
                if new_text.chars().count() == pointer {
                    new_text.push(c);
                } else {
                    let mut part1: String = new_text
                        .chars()
                        .enumerate()
                        .filter(|(i, _s)| *i < pointer)
                        .map(|(_i, s)| s)
                        .collect();
                    let part2: String = new_text
                        .chars()
                        .enumerate()
                        .filter(|(i, _s)| *i >= pointer)
                        .map(|(_i, s)| s)
                        .collect();
                    part1.push(c);
                    part1.push_str(&part2);
                    new_text = part1;
                }
                pointer += 1;
            }
            Key::Esc => break,
            Key::Backspace => {
                if pointer == new_text.chars().count() && new_text.chars().count() != 0 {
                    new_text.pop();
                    pointer -= 1;
                } else if new_text.chars().count() == 0 {
                    new_text.pop();
                } else {
                    new_text = new_text
                        .chars()
                        .enumerate()
                        .map(|(i, s)| if i != pointer { Some(s) } else { None })
                        .flatten()
                        .collect();
                    pointer -= 1;
                }
            }
            Key::Left => {
                if pointer > 0 {
                    pointer -= 1;
                }
            }
            Key::Right => {
                if pointer < new_text.chars().count() {
                    pointer += 1;
                }
            }
            _ => {}
        }
        write!(
            stdout,
            "{goto}{clear}{text}",
            goto = cursor::Goto(1, 1),
            clear = clear::All,
            text = new_text
        )
        .unwrap();

        write!(
            stdout,
            "{}",
            cursor::Goto(
                (pointer as u16 + 1) % terminal_size().unwrap().0,
                cur_y + pointer as u16 / terminal_size().unwrap().0
            )
        )
        .unwrap();
        stdout.flush().unwrap();
    }

    new_text
}

pub fn edit_journal_description(desc: &String) -> String {
    text_editor(desc)
}

pub fn edit_note(text: &String) -> String {
    text_editor(text)
}

pub fn save_error(text: &str) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{clear}{cursor}{goto}{red}{bold}ERROR{reset_color}{reset_style}",
        clear = clear::All,
        cursor = cursor::Hide,
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 2,
            terminal_size().unwrap().1 / 2 - 2
        ),
        red = color::Fg(color::Red),
        bold = style::Bold,
        reset_color = color::Fg(color::Reset),
        reset_style = style::Reset
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{white}Could not save {name}.{reset}",
        // Goto the cell.
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 13,
            terminal_size().unwrap().1 / 2 + 2
        ),
        white = color::Fg(color::White),
        name = text,
        reset = color::Fg(color::Reset)
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{white}Press any key to continue.{reset}",
        // Goto the cell.
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 13,
            terminal_size().unwrap().1 / 2 + 4
        ),
        white = color::Fg(color::White),
        reset = color::Fg(color::Reset)
    )
    .unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            _ => break,
        }
    }
}

pub fn add_journal_link() -> String {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{clear}{cursor}{goto}{red}{bold}CREATE NEW NOTE{reset_color}{reset_style}",
        clear = clear::All,
        cursor = cursor::Hide,
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 8,
            terminal_size().unwrap().1 / 2 - 2
        ),
        red = color::Fg(color::Red),
        bold = style::Bold,
        reset_color = color::Fg(color::Reset),
        reset_style = style::Reset
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{bold}Page name: {reset}",
        // Goto the cell.
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 13,
            terminal_size().unwrap().1 / 2 + 2
        ),
        bold = style::Bold,
        reset = style::Reset
    )
    .unwrap();

    stdout.flush().unwrap();

    let mut buf = String::new();

    for k in stdin.keys() {
        match k.unwrap() {
            Key::Char(c) => match c {
                '\n' => return buf,
                _ => buf.push(c),
            },
            Key::Backspace => {
                buf.pop();
            }
            Key::Esc => return String::new(),
            _ => {}
        }

        write!(
            stdout,
            "{goto}{bold}Page name: {reset}{name}",
            // Goto the cell.
            goto = cursor::Goto(
                terminal_size().unwrap().0 / 2 - 13,
                terminal_size().unwrap().1 / 2 + 2
            ),
            bold = style::Bold,
            reset = style::Reset,
            name = buf
        )
        .unwrap();

        write!(stdout, "{}", clear::AfterCursor).unwrap();

        stdout.flush().unwrap();
    }

    String::new()
}

pub fn select_create_note(title: &str) -> bool {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{clear}{goto}The note for {name} does not exist.",
        clear = clear::All,
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 21,
            terminal_size().unwrap().1 / 2
        ),
        name = title
    )
    .unwrap();

    write!(
        stdout,
        "{goto}Create it? (y/n)",
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 8,
            terminal_size().unwrap().1 / 2 + 1
        )
    )
    .unwrap();

    stdout.flush().unwrap();

    let choice: bool = false;

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char(c) => match c {
                'y' | 'Y' => return true,
                'n' | 'N' => return false,
                _ => {}
            },
            _ => {}
        }
    }

    choice
}

pub fn reset_cursor() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", cursor::Show).unwrap();
    stdout.flush().unwrap();
}

pub fn create_new_trail() -> String {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{clear}{cursor}{goto}{red}{bold}CREATE NEW TRAIL{reset_color}{reset_style}",
        clear = clear::All,
        cursor = cursor::Hide,
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 8,
            terminal_size().unwrap().1 / 2 - 2
        ),
        red = color::Fg(color::Red),
        bold = style::Bold,
        reset_color = color::Fg(color::Reset),
        reset_style = style::Reset
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{bold}Trail name: {reset}",
        // Goto the cell.
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 13,
            terminal_size().unwrap().1 / 2 + 2
        ),
        bold = style::Bold,
        reset = style::Reset
    )
    .unwrap();

    stdout.flush().unwrap();

    let mut buf = String::new();

    for k in stdin.keys() {
        match k.unwrap() {
            Key::Char(c) => match c {
                '\n' => return buf,
                _ => buf.push(c),
            },
            Key::Backspace => {
                buf.pop();
            }
            Key::Esc => return String::new(),
            _ => {}
        }

        write!(
            stdout,
            "{goto}{bold}Trail name: {reset}{name}",
            // Goto the cell.
            goto = cursor::Goto(
                terminal_size().unwrap().0 / 2 - 13,
                terminal_size().unwrap().1 / 2 + 2
            ),
            bold = style::Bold,
            reset = style::Reset,
            name = buf
        )
        .unwrap();

        write!(stdout, "{}", clear::AfterCursor).unwrap();

        stdout.flush().unwrap();
    }

    String::new()
}

pub fn edit_trail_description(desc: &String) -> String {
    text_editor(desc)
}

enum TrailHopState {
    Name,
    Description,
    End,
}

pub fn add_trail_hop() -> (String, String) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{clear}{cursor}{goto}{red}{bold}CREATE NEW HOP{reset_color}{reset_style}",
        clear = clear::All,
        cursor = cursor::Hide,
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 7,
            terminal_size().unwrap().1 / 2 - 2
        ),
        red = color::Fg(color::Red),
        bold = style::Bold,
        reset_color = color::Fg(color::Reset),
        reset_style = style::Reset
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{bold}Hop name: {reset}",
        // Goto the cell.
        goto = cursor::Goto(
            terminal_size().unwrap().0 / 2 - 13,
            terminal_size().unwrap().1 / 2 + 2
        ),
        bold = style::Bold,
        reset = style::Reset
    )
    .unwrap();

    stdout.flush().unwrap();

    let mut name_buf = String::new();
    let mut desc_buf = String::new();

    let keys = stdin.keys();
    let mut current_state = TrailHopState::Name;

    for k in keys.into_iter() {
        match k.unwrap() {
            Key::Char(c) => match c {
                '\n' => match current_state {
                    TrailHopState::Name => current_state = TrailHopState::Description,
                    TrailHopState::Description => current_state = TrailHopState::End,
                    TrailHopState::End => break,
                },
                _ => match current_state {
                    TrailHopState::Name => name_buf.push(c),
                    TrailHopState::Description => desc_buf.push(c),
                    TrailHopState::End => {}
                },
            },
            Key::Backspace => {
                name_buf.pop();
            }
            Key::Esc => return (String::new(), String::new()),
            _ => {}
        }

        match current_state {
            TrailHopState::Name => {
                write!(
                    stdout,
                    "{goto}{bold}Hop name: {reset}{name}",
                    // Goto the cell.
                    goto = cursor::Goto(
                        terminal_size().unwrap().0 / 2 - 13,
                        terminal_size().unwrap().1 / 2 + 2
                    ),
                    bold = style::Bold,
                    reset = style::Reset,
                    name = name_buf
                )
                .unwrap();

                write!(stdout, "{}", clear::AfterCursor).unwrap();
            }
            TrailHopState::Description => {
                write!(
                    stdout,
                    "{goto}{bold}Hop description: {reset}{description}",
                    // Goto the cell.
                    goto = cursor::Goto(
                        terminal_size().unwrap().0 / 2 - 13,
                        terminal_size().unwrap().1 / 2 + 2
                    ),
                    bold = style::Bold,
                    reset = style::Reset,
                    description = desc_buf
                )
                .unwrap();
            }
            _ => {}
        }

        stdout.flush().unwrap();
    }

    (name_buf, desc_buf)
}
