extern crate termion;

use termion::cursor::DetectCursorPos;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, scroll, style, terminal_size};

use std::io::{stdin, stdout, Write};

use crate::lib::{Journal, Note, Trail};

pub enum Message {
    Save(String),
    EditDescription,
    AddPage,
    SelectLinks,
    EditLinks,
    GotoLink(String),
    Menu,
    Exit,
    Back,
    Edit,
}

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

pub fn display_journal(page: &Journal) -> Message {
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
        "{goto}{bold}Pages created today:{reset_style}{goto}",
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
                'd' | 'D' => return Message::EditDescription,
                'e' | 'E' => return Message::EditLinks,
                'm' | 'M' => return Message::Menu,
                'l' | 'L' => return Message::SelectLinks,
                'q' | 'Q' => return Message::Exit,
                _ => {}
            },
            _ => {}
        }
    }

    Message::Exit
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

pub fn link_menu(list: &Vec<String>) -> Message {
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

    let mut StrBuf = String::new();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char(c) => match c {
                '\n' => break,
                'q' | 'Q' => return Message::Exit,
                _ => {
                    StrBuf.push(c);
                    write!(stdout, "{}", c).unwrap();
                    stdout.flush().unwrap();
                }
            },
            Key::Esc => return Message::Back,
            _ => {}
        }
    }

    Message::GotoLink(StrBuf)
}

pub fn display_note(page: &Note) -> Message {
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
                'e' | 'E' => return Message::Edit,
                'l' | 'L' => return Message::SelectLinks,
                'm' | 'M' => return Message::Menu,
                'q' | 'Q' => return Message::Exit,
                _ => {}
            },
            Key::Esc => return Message::Back,
            _ => {}
        }
    }

    Message::Back
}

pub fn display_trail(page: &Trail) {
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

    for (i, x) in page.hops.iter().enumerate() {
        let line_number = (x.1.len() + x.0.len()) as u16 / terminal_size().unwrap().0;

        write!(
            stdout,
            "{goto}{bold}Hop {number}:{reset_style} {name}",
            goto = cursor::Goto(1, 3 + i as u16),
            bold = style::Bold,
            reset_style = style::Reset,
            number = i,
            name = x.0,
        )
        .unwrap();

        write!(
            stdout,
            "{goto}{bold}Description: {desc}{reset_style}",
            goto = cursor::Goto(1, 5 + i as u16),
            bold = style::Bold,
            reset_style = style::Reset,
            desc = x.1
        )
        .unwrap();
    }
}

pub enum MenuOption {
    Journal,
    Notes,
    Trails,
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
            terminal_size().unwrap().1 / 2 - 4
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
            terminal_size().unwrap().1 / 2 - 2
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
            terminal_size().unwrap().1 / 2
        ),
        white = color::Fg(color::White),
        reset = color::Fg(color::Reset)
    )
    .unwrap();

    write!(
        stdout,
        "{goto}{white}(n) Open or write a note.{reset}",
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
        "{goto}{white}(t) Open or write a trail.{reset}",
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
                'j' | 'J' => return MenuOption::Journal,
                'n' | 'N' => return MenuOption::Notes,
                't' | 'T' => return MenuOption::Trails,
                'q' | 'Q' => return MenuOption::Quit,
                _ => {}
            },
            Key::Esc => return MenuOption::Quit,
            _ => {}
        }
    }

    MenuOption::Quit
}

pub fn edit_journal_description(desc: &String) -> String {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();

    write!(
        stdout,
        "{goto}{clear}{show_cursor}{desc}",
        goto = cursor::Goto(1, 1),
        clear = clear::All,
        desc = desc,
        show_cursor = cursor::Show
    )
    .unwrap();

    stdout.flush().unwrap();

    let mut new_desc = desc.clone();
    let mut pointer = new_desc.len();

    let (mut _cur_x, cur_y) = stdout.cursor_pos().unwrap();

    for k in stdin.keys() {
        match k.unwrap() {
            Key::Char(c) => {
                if new_desc.chars().count() == pointer {
                    new_desc.push(c);
                } else {
                    let mut part1: String = new_desc
                        .chars()
                        .enumerate()
                        .filter(|(i, s)| *i < pointer)
                        .map(|(_i, s)| s)
                        .collect();
                    let part2: String = new_desc
                        .chars()
                        .enumerate()
                        .filter(|(i, s)| *i >= pointer)
                        .map(|(_i, s)| s)
                        .collect();
                    part1.push(c);
                    part1.push_str(&part2);
                    new_desc = part1;
                }
                pointer += 1;
            }
            Key::Esc => break,
            Key::Backspace => {
                if pointer == new_desc.chars().count() && new_desc.chars().count() != 0 {
                    new_desc.pop();
                    pointer -= 1;
                } else if new_desc.chars().count() == 0 {
                    new_desc.pop();
                } else {
                    new_desc = new_desc
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
                if pointer < new_desc.chars().count() {
                    pointer += 1;
                }
            }
            _ => {}
        }
        write!(
            stdout,
            "{goto}{clear}{desc}",
            goto = cursor::Goto(1, 1),
            clear = clear::All,
            desc = new_desc
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

    new_desc
}

pub fn edit_note(text: &String) -> String {
    edit_journal_description(text)
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
