use file_picker::{Picker, PickerBuilder};

fn main() -> dialoguer::Result<()> {
    let dir = Picker::directory().with_prompt("Pick a folder").select()?;
    println!("{}", dir.display());
    Ok(())
}
