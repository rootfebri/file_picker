use dialoguer::theme::ColorfulTheme;
use std::fmt::Formatter;
use std::path::PathBuf;

static FILE: dialoguer::console::Emoji<'_, '_> = dialoguer::console::Emoji("📃", "📃");
static FOLDER: dialoguer::console::Emoji<'_, '_> = dialoguer::console::Emoji("📂", "📂");

struct Item(PathBuf);

impl Item {
    pub fn new(item: PathBuf) -> Self {
        Self(item)
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let file_name = match self.0.file_name() {
            Some(file) => file.to_string_lossy().to_string(),
            None => self.0.display().to_string(),
        };
        let icon = match self.0.is_file() {
            true => FILE,
            false => FOLDER,
        };
        write!(f, "{} {}", icon, file_name)
    }
}

/// Prompts the user to pick a file interactively from the current directory.
///
/// # Arguments
///
/// * `label` - A string slice that holds the prompt label to display to the user.
///
/// # Returns
///
/// * `std::io::Result<PathBuf>` - Returns the `PathBuf` of the selected file.
///
/// # Examples
///
/// ```rust
/// use crate::file_picker::file_picker;
///
/// let file_path = file_picker("Select a file:").expect("Failed to pick a file");
/// println!("You selected the file: {:?}", file_path);
/// ```
///
pub fn file_picker(label: &str) -> std::io::Result<PathBuf> {
    let mut base_dir = std::env::current_dir()?;
    loop {
        let items = items(base_dir.clone());
        if let Ok(item) = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .with_prompt(label)
            .highlight_matches(true)
            .interact()
        {
            match items.get(item) {
                None => {}
                Some(item) => {
                    if item.0.display().to_string() == *".." {
                        base_dir.pop();
                    } else if item.0.is_file() {
                        return Ok(item.0.to_path_buf());
                    } else {
                        base_dir = base_dir.join(&item.0);
                    }
                }
            }
        }
    }
}

fn items(wd: PathBuf) -> Vec<Item> {
    let mut items = vec![Item::new(PathBuf::from(".."))];
    let x = match std::fs::read_dir(&wd) {
        Ok(rd) => rd
            .filter_map(|x| x.ok())
            .map(|entry| Item::new(entry.path()))
            .collect::<Vec<_>>(),
        Err(_) => vec![],
    };

    items.extend(x);
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let file = file_picker("Pick a file to assert is_file and exists").unwrap();
        assert!(file.is_file());
        assert!(file.exists());
    }
}
