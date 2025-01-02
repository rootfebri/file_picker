pub use spinners::TickStrings;

use dialoguer::theme::ColorfulTheme;
use std::fmt::Formatter;
use std::path::PathBuf;

mod picker;
pub use picker::*;
mod spinners;

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
/// use file_picker::file_picker;
///
/// let file_path = file_picker("Select a file:", None).expect("Failed to pick a file");
/// println!("You selected the file: {:?}", file_path);
/// ```
///
/// # Deprecated
/// This function is deprecated. Use [`file_picker::Picker`] instead.
#[deprecated = "use `file_picker::Picker` instead"]
pub fn file_picker(label: &str, base: Option<PathBuf>) -> std::io::Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let mut base_dir = base.unwrap_or(cwd);
    loop {
        let items = items(base_dir.clone())
            .into_iter()
            .filter_map(|x| {
                if x.0.display().to_string() != *"." {
                    Some(x)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
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

/// Prompts the user to pick a directory interactively from the current directory.
///
/// # Arguments
///
/// * `label` - A string slice that holds the prompt label to display to the user.
///
/// # Returns
///
/// * `std::io::Result<PathBuf>` - Returns the `PathBuf` of the selected directory.
///
/// # Examples
///
/// ```rust
/// use file_picker::dir_picker;
///
/// let dir_path = dir_picker("Select a directory:", None).expect("Failed to pick a directory");
/// println!("You selected the directory: {:?}", dir_path);
/// ```
/// # Deprecated
/// This function is deprecated. Use [`file_picker::Picker`] instead.
#[deprecated = "use `file_picker::Picker` instead"]
pub fn dir_picker(label: &str, base: Option<PathBuf>) -> std::io::Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let mut base_dir = base.unwrap_or(cwd);

    loop {
        let items = items(base_dir.clone())
            .into_iter()
            .filter_map(|x| if x.0.is_dir() { Some(x) } else { None })
            .collect::<Vec<_>>();
        if let Ok(item) = dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .with_prompt(label)
            .report(false)
            .highlight_matches(true)
            .interact()
        {
            match items.get(item) {
                None => {}
                Some(item) => {
                    if item.0.display().to_string() == *"." {
                        return Ok(base_dir);
                    } else if item.0.display().to_string() == *".." {
                        base_dir.pop();
                    } else {
                        base_dir = base_dir.join(&item.0);
                    }
                }
            }
        }
    }
}

fn items(wd: PathBuf) -> Vec<Item> {
    let mut items = vec![
        Item::new(PathBuf::from(".")),
        Item::new(PathBuf::from("..")),
    ];

    let mut x = match std::fs::read_dir(&wd) {
        Ok(rd) => rd
            .filter_map(|x| x.ok())
            .map(|entry| Item::new(entry.path()))
            .collect::<Vec<_>>(),
        Err(_) => vec![],
    };

    items.append(&mut x);
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_picker() {
        let file = file_picker("Pick a file to assert is_file and exists", None).unwrap();
        assert!(file.is_file());
        assert!(file.exists());
    }

    #[test]
    fn test_dir_picker() {
        let dir = dir_picker("Pick a file to assert is_dir and exists", None).unwrap();

        assert_ne!(dir.display().to_string(), *".");
        assert!(dir.is_dir());
        assert!(dir.exists());
    }

    #[test]
    fn tick_string() {
        let start = std::time::Instant::now();
        let tick = TickStrings::new();
        println!("Time taken to construct {} ms", start.elapsed().as_millis());

        assert_eq!(tick.arc.frames.first().unwrap(), "◜");
    }
}
