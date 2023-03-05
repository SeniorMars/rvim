use std::path::Path;

use nvim_oxi as oxi;
use oxi::api::{self, Buffer, Window};

#[oxi::module]
/// returns a useable status line
fn status_line() -> oxi::Result<()> {
    Ok(())
}

/// toggle spell checking
fn spell_toggle() -> oxi::Result<()> {
    let mut current_window = Window::current();
    let mut curr_buf = Buffer::current();
    let spell_on: bool = current_window.get_option("spell")?;

    if spell_on {
        current_window.set_option("spell", false)?;
        curr_buf.set_option("spelllang", "en")
    } else {
        current_window.set_option("spell", true)?;
        // this is kinda hacky, but it works
        // NOTE: it should be able to take an array of strings
        // curr_buf.set_option("spelllang", vec!["en_us", "de"].join(","))
        curr_buf.set_option("spelllang", "en_us,de")
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
        // let branch = "Main";
        // NOTE: I'm not sure how to test this, but it works
        let branch: String = api::call_function("FugitiveHead", vec![])?;
        if !branch.is_empty() {
            let width = Window::current().get_width()?;
            // let width = 100;
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
    api::set_var("loaded_fugitive", true).expect("set_var failed");
    let branch = git_branch().expect("git_branch failed");
    assert_eq!(branch, "");
}

// fn word_count() -> oxi::Result<String> {
//     let words: Vec<_> = api::call_function("wordcount", vec![])?;
//
//     dbg!(&words);
//
//     Ok(String::new())
// }

#[oxi::test]
fn test_word_count() {
    // let result: Vec<_> = api::call_function("wordcount", vec![]).expect("call_function failed");
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
        format!("{}{}", size, units[i])
    };

    Ok(format_file_size(size))
}

#[test]
fn test_format_file_size() {
    let file = Path::new("Cargo.toml");
    let size = file.metadata().expect("metadata failed").len();

    // dbg!(&size);
    // let result = format_file_size(file).expect("format_file_size failed");
    // dbg!(result);
}
