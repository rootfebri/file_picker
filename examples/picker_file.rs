use file_picker::{Picker, PickerBuilder};

fn main() -> dialoguer::Result<()> {
    let dir = Picker::file().with_prompt("Pick a file").select()?;
    println!("{}", dir.display());
    Ok(())
}
