use nvim_oxi::api::{self, Buffer, Window};
use nvim_oxi::{self as oxi, print};
use oxi::opts::*;

#[oxi::module]
/// returns a useable status line
fn status_line() -> oxi::Result<()> {
    Ok(())
}

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
    current_window.set_option("spell", true).expect("set_option failed");
    spell_toggle().expect("spell_toggle failed");
    assert!(!current_window.get_option::<bool>("spell").expect("get_option failed"));
    spell_toggle().expect("spell_toggle failed");
    let what: String = curr_buf.get_option("spelllang").expect("get_option failed");
    assert_eq!(what, "en_us,de");
}
