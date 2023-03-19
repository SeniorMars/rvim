use nvim_oxi as oxi;
use oxi::{
    api::{self, Buffer, Window, types::Mode},
    Dictionary,
};
use std::{env, path::PathBuf};
#[allow(unused_imports)]
use std::path::Path;

// #[oxi::module]
// /// returns a useable status line
// fn status_line() -> oxi::Result<()> {
//     Ok(())
// }
// 

// static MODES: phf::Map<&'static str, [&'static str; 2]> = phf_map! {
//     "n" => ["NORMAL", "N"],
//     "no" => ["N·OPERATOR PENDING", "N·P"],
//     "v" => ["VISUAL", "V"],
//     "V" => ["V·LINE", "V·L"],
//     "" => ["V·BLOCK", "V·B"],
//     "s" => ["SELECT", "S"],
//     "S" => ["S·LINE", "S·L"],
//     "" => ["S·BLOCK", "S·B"],
//     "i" => ["INSERT", "I"],
//     "ic" => ["INSERT", "I"],
//     "ix" => ["INSERT", "R"],
//     "R" => ["REPLACE", "R"],
//     "Rv" => ["V·REPLACE", "V·R"],
//     "c" => ["COMMAND", "C"],
//     "cv" => ["VIM·EX", "V·E"],
//     "ce" => ["EX", "E"],
//     "r" => ["PROMPT", "P"],
//     "rm" => ["MORE", "M"],
//     "r?" => ["CONFIRM", "C"],
//     "!" => ["SHELL", "S"],
//     "t" => ["TERMINAL", "T"],
// };


fn current_mode() -> oxi::Result<String> {
    let width = Window::current().get_width()?;
    let small = width < 80;

    match (api::get_mode()?.mode, small) {
        (Mode::CmdLine, true) => todo!(),
        (Mode::CmdLine, false) => todo!(),
        (Mode::Insert, true) => todo!(),
        (Mode::Insert, false) => todo!(),
        (Mode::InsertCmdLine, true) => todo!(),
        (Mode::InsertCmdLine, false) => todo!(),
        (Mode::Langmap, true) => todo!(),
        (Mode::Langmap, false) => todo!(),
        (Mode::NormalVisualOperator, true) => todo!(),
        (Mode::NormalVisualOperator, false) => todo!(),
        (Mode::Normal, true) => todo!(),
        (Mode::Normal, false) => todo!(),
        (Mode::OperatorPending, true) => todo!(),
        (Mode::OperatorPending, false) => todo!(),
        (Mode::Select, true) => todo!(),
        (Mode::Select, false) => todo!(),
        (Mode::Terminal, true) => todo!(),
        (Mode::Terminal, false) => todo!(),
        (Mode::Visual, true) => todo!(),
        (Mode::Visual, false) => todo!(),
        (Mode::VisualSelect, true) => todo!(),
        (Mode::VisualSelect, false) => todo!(),
        (_, true) => todo!(),
        (_, false) => todo!(),
    }
}

/// toggle spell checking
fn spell_toggle() -> oxi::Result<()> {
    // another way would be to use the `nvim_get_option` and `nvim_set_option` functions
    // but those are globals, so it may be better to use the `Window` and `Buffer` methods
    let mut current_window = Window::current();
    let mut curr_buf = Buffer::current();
    let spell_on: bool = current_window.get_option("spell")?;

    if spell_on {
        current_window.set_option("spell", false)?;
        Ok(curr_buf.set_option("spelllang", "en")?)
    } else {
        current_window.set_option("spell", true)?;
        // this is kinda hacky, but it works
        // NOTE: it should be able to take an array of strings
        // curr_buf.set_option("spelllang", vec!["en_us", "de"].join(","))
        Ok(curr_buf.set_option("spelllang", "en_us,de")?)
    }
}

#[oxi::test]
fn test_spell_toggle() {
    let mut current_window = Window::current();
    let curr_buf = Buffer::current();
    current_window
        .set_option("spell", true)
        .expect("set_option failed");
    spell_toggle().expect("spell_toggle failed");
    assert!(!current_window
        .get_option::<bool>("spell")
        .expect("get_option failed"));
    spell_toggle().expect("spell_toggle failed");
    let what: String = curr_buf.get_option("spelllang").expect("get_option failed");
    assert_eq!(what, "en_us,de");
}

fn git_branch() -> oxi::Result<String> {
    let loaded_fugitive: bool = api::get_var("loaded_fugitive")?;

    if loaded_fugitive {
        let branch: String = api::call_function("FugitiveHead", vec![])?;
        if !branch.is_empty() {
            let width = Window::current().get_width()?;
            if width <= 80 {
                return Ok(format!(
                    " {}",
                    branch.to_uppercase().chars().take(2).collect::<String>()
                ));
            }
            return Ok(format!(" {}", branch.to_uppercase()));
        }
    }

    Ok(String::new())
}

#[oxi::test]
fn test_git_branch() {
    api::set_var("loaded_fugitive", false).expect("set_var failed");
    let branch = git_branch().expect("git_branch failed");
    assert_eq!(branch, "");
}

fn word_count() -> oxi::Result<String> {
    let words: Dictionary = api::call_function("wordcount", vec![])?;

    if let Some(visual_words) = words.get(&"visual_words".to_string()) {
        let visual_words = unsafe { visual_words.as_integer_unchecked() };
        Ok(format!("[{visual_words}]"))
    } else {
        let words = words.get(&"words".to_string()).expect("API has changed");
        let words = unsafe { words.as_integer_unchecked() };
        Ok(format!("[{words}]"))
    }
}

#[oxi::test]
fn test_word_count() {
    let words = word_count().expect("word_count failed");
    assert_eq!(words, "[0]");
}

fn human_file_size() -> oxi::Result<String> {
    let buff = Buffer::current();
    let file = buff.get_name()?;
    if !file.exists() {
        return Ok(String::new());
    }

    let size = file.metadata().expect("metadata failed").len();
    if size == 0 {
        return Ok(String::new());
    }

    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

    let format_file_size = |size: u64| -> String {
        let mut size = size;
        let mut i = 0;
        while size > 1024 {
            size /= 1024;
            i += 1;
        }
        format!("{size}{}", units[i])
    };

    Ok(format_file_size(size))
}

#[test]
fn test_format_file_size() {
    let file = Path::new("Cargo.toml");
    let size = file.metadata().expect("metadata failed").len();

    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

    let format_file_size = |size: u64| -> String {
        let mut size = size;
        let mut i = 0;
        while size > 1024 {
            size /= 1024;
            i += 1;
        }
        format!("{size}{}", units[i])
    };

    let result = format_file_size(size);
    assert_eq!(result, "271B");
}

fn smart_file_path() -> oxi::Result<String> {
    let buf_name = Buffer::current().get_name()?;
    if buf_name == Path::new("") {
        return Ok("[No Name]".to_string());
    }

    let home = env::var("HOME").expect("HOME not set");
    let mut file_dir: PathBuf;

    let mut is_term = false;
    if buf_name.starts_with("term://") {
        file_dir = env::current_dir().expect("current_dir failed");
        if file_dir == PathBuf::from(&home) {
            return Ok("$HOME ".to_string());
        }
        is_term = true;
    } else {
        file_dir = buf_name.parent().expect("parent failed").to_path_buf();
    }

    if let Ok(path) = file_dir.strip_prefix(&home) {
        file_dir = PathBuf::from("~").join(path);
    }

    let width = Window::current().get_width()?;

    if width <= 80 {
        file_dir = path_shorten(&file_dir, 1);
    }

    if is_term {
        Ok(format!("{} ", file_dir.display()))
    } else {
        let file_name = PathBuf::from(buf_name.file_name().expect("file_name failed"));
        let smart_path = file_dir.join(file_name);
        Ok(format!("{} ", smart_path.display()))
    }
}

// shortens a path to the first letter of each directory
// max_component_len should be a num between 1..usize::MAX
fn path_shorten(file_dir: &Path, max_component_len: usize) -> PathBuf {
    let max_component_len = if max_component_len == 0 {
        1
    } else {
        max_component_len
    };

    let mut shorten_path = String::new();
    let mut components = file_dir.components().peekable();
    while let Some(component) = components.next() {
        let component = component.as_os_str().to_str().expect("Not a valid UTF-8 str");
        if component.len() > max_component_len {
            // keep leading '.' for hidden files
            // keep leading '~' is kept to align with vim's shortenpath function
            if component.starts_with('.') || component.starts_with('~') {
                shorten_path.push_str(&component[..=max_component_len]);
            } else {
                shorten_path.push_str(&component[..max_component_len]);
            }
        } else {
            shorten_path.push_str(component);
        }

        if components.peek().is_none() {
            break;
        } 
        shorten_path.push('/');
    }

    PathBuf::from(shorten_path)
}

#[oxi::test]
fn test_path() {
    let mut buf_handle = api::create_buf(false, true).expect("create_buf failed");
    let path = buf_handle.get_name().expect("get_name failed");
    assert_eq!(path, Path::new(""));

    let cwd = env::current_dir().expect("current_dir failed");

    buf_handle
        .set_name(&cwd.join("Cargo.toml"))
        .expect("set_name failed");

    assert_eq!(
        buf_handle.get_name().expect("get_name failed"),
        cwd.join("Cargo.toml")
    );


    // buf_handle.set_name("Cargo.toml").expect("set_name failed");
    // assert_eq!(buf_handle.get_name().expect("get_name failed"), Path::new("./Cargo.toml"));
    // if let Ok(path) = buf_handle.get_name() {
    //     dbg!("no");
    // } else {
    //     dbg!("no path");
    // }
    // let path = buf_handle.get_name();
    // if let Ok(path) = path {
    //     dbg!("no");
    // } else {
    //     dbg!("no path");
    // }
}

#[test]
fn path_test() {
    let cwd = env::current_dir().expect("current_dir failed");
    let home = env::var("HOME").expect("HOME not set");
    let rest = cwd.strip_prefix(home).expect("strip_prefix failed");
    println!("{:?}", PathBuf::from("~").join(rest));

    let path = PathBuf::from("~/test/hello/world/here/test");
    let shorten_path = path_shorten(&path, 2);
    println!("{}", shorten_path.display());
}
