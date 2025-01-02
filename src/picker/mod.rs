use dialoguer::theme::{ColorfulTheme, Theme};
use std::path::{Path, PathBuf};

mod pick_types;
pub use pick_types::PickType;

pub struct Picker {
    ci: usize,
    pwd: Option<PathBuf>,
    cwd: PathBuf,
    directories: Vec<PathBuf>,
    files: Vec<PathBuf>,
    pick_type: PickType,
    prompt: Option<String>,
    items: Vec<PathBuf>,
    theme: Box<dyn Theme>,
}

impl Picker {
    #[cfg(windows)]
    const DS: &'static str = "\\";
    #[cfg(not(windows))]
    const DS: &'static str = "/";

    pub fn file() -> Self {
        Self::new(PickType::File, Some("Pick a File"))
    }

    pub fn directory() -> Self {
        Self::new(PickType::Directory, Some("Pick a Folder"))
    }

    pub fn new(pick_type: PickType, prompt: Option<impl Into<String>>) -> Self {
        let prompt = prompt.map(|prompt| prompt.into());
        let cwd = PathBuf::from(".")
            .canonicalize()
            .expect("Cannot canonicalize current dir");

        let mut this = Self {
            ci: 0,
            pwd: cwd.parent().extract_option(),
            cwd: cwd.clone(),
            directories: vec![],
            files: vec![],
            pick_type,
            prompt,
            items: vec![],
            theme: Box::new(ColorfulTheme::default()),
        };

        this.process_files();
        this
    }

    fn process_files(&mut self) {
        if let Ok(entry) = self.cwd.read_dir() {
            for dir in entry.map_while(|d| d.ok()) {
                if dir.path().is_file() {
                    self.files.push(dir.path());
                } else {
                    self.directories.push(dir.path());
                }
            }
        }

        self.load_items();
    }

    fn load_items(&mut self) {
        self.items.clear();
        match self.pick_type {
            PickType::Directory => {
                self.items.append(&mut self.directories);
            }
            PickType::File => {
                self.items.append(&mut self.files);
                self.items.append(&mut self.directories);
            }
        }
    }

    pub fn select(&mut self) -> dialoguer::Result<PathBuf> {
        loop {
            let display = self.make_display(&self.items);
            let index = self.pick(&display)?;

            match self.pick_type {
                PickType::Directory => {
                    if index == 0 {
                        return Ok(self.cwd.clone());
                    } else if self.pwd.is_some() && index == 1 {
                        self.cwd.pop();
                    } else {
                        self.cwd = self.cwd.join(&self.items.as_slice()[index]);
                    }
                }
                PickType::File => {
                    if self.pwd.is_some() && index == 0 {
                        self.cwd.pop();
                    } else {
                        let path = &self.items.as_slice()[index];
                        if path.is_file() {
                            return Ok(path.to_owned());
                        } else {
                            self.cwd = self.cwd.join(path);
                        }
                    }
                }
            }

            self.process_files();
        }
    }

    fn pick(&mut self, display: &[String]) -> dialoguer::Result<usize> {
        let theme = ColorfulTheme::default();
        let mut dialog = dialoguer::FuzzySelect::with_theme(&theme);

        if let Some(ref prompt) = self.prompt {
            // Use custom prompt and include the relative path from the root
            let current_prompt = if let Some(name) = self.cwd.file_name() {
                let mut relative_path = String::new();
                let mut temp_path = self.cwd.clone();
                while let Some(parent) = temp_path.parent() {
                    relative_path.push_str(&("..".to_owned() + Self::DS));
                    temp_path = parent.to_path_buf();
                }
                format!(
                    "{} [root{}{}{}]",
                    prompt,
                    &("..".to_owned() + Self::DS),
                    relative_path,
                    name.to_string_lossy()
                )
            } else {
                prompt.to_string()
            };

            dialog = dialog.with_prompt(current_prompt);
        } else {
            // Use only the relative path from the root as the prompt
            let current_prompt = if let Some(name) = self.cwd.file_name() {
                let mut relative_path = String::new();
                let mut temp_path = self.cwd.clone();
                while let Some(parent) = temp_path.parent() {
                    relative_path.push_str("..\\");
                    temp_path = parent.to_path_buf();
                }
                format!("[root\\{}{}]", relative_path, name.to_string_lossy())
            } else {
                "[root]".to_string()
            };

            dialog = dialog.with_prompt(current_prompt);
        }

        #[rustfmt::skip]
        let index = dialog
            .default(if self.ci + 1 > display.len() { 0 } else { self.ci })
            .report(false)
            .items(display)
            .interact()?;

        Ok(index)
    }

    fn make_display(&self, items: &[PathBuf]) -> Vec<String> {
        let mut displays: Vec<String> = vec![];

        match (self.pick_type == PickType::Directory, self.pwd.is_some()) {
            (true, true) => {
                displays.push(".".into());
                displays.push("..".into())
            }
            (false, true) => displays.push("..".into()),
            (true, false) => displays.push(".".into()),
            (false, false) => {}
        }

        items.iter().for_each(|path| {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                let icon = if path.is_dir() {
                    Self::FOLDER_ICON
                } else {
                    Self::FILE_ICON
                };
                displays.push(format!("{icon} {name}"));
            }
        });

        displays
    }

    const FOLDER_ICON: &'static str = "📁"; // Icon for folders
    const FILE_ICON: &'static str = "📄"; // Icon for files
}

trait ToStrong<T: Send + Sync> {
    fn extract_option(&self) -> T;
}

impl ToStrong<Option<PathBuf>> for Option<&'_ Path> {
    fn extract_option(&self) -> Option<PathBuf> {
        self.map(|path| path.to_path_buf())
    }
}

pub trait PickerBuilder {
    fn with_theme(self, theme: impl Theme + 'static) -> Self;
    fn with_prompt(self, prompt: impl Into<String>) -> Self;
}

impl PickerBuilder for Picker {
    fn with_theme(mut self, theme: impl Theme + 'static) -> Self {
        self.theme = Box::new(theme);
        self
    }

    fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_picker_file() {
        let picker = Picker::file();
        assert_eq!(picker.pick_type, PickType::File);
        assert_eq!(picker.prompt.unwrap(), "Pick a File");
    }

    #[test]
    fn test_picker_directory() {
        let picker = Picker::directory();
        assert_eq!(picker.pick_type, PickType::Directory);
        assert_eq!(picker.prompt.unwrap(), "Pick a Folder");
    }

    #[test]
    fn test_picker_with_custom_prompt() {
        let picker = Picker::file().with_prompt("Select your file");
        assert_eq!(picker.prompt.unwrap(), "Select your file");
    }

    #[test]
    fn test_load_items_with_files_only() {
        let mut picker = Picker::file();
        picker.files = vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")];
        picker.directories = vec![PathBuf::from("dir1"), PathBuf::from("dir2")];
        picker.load_items();

        assert_eq!(
            picker.items,
            vec![
                PathBuf::from("file1.txt"),
                PathBuf::from("file2.txt"),
                PathBuf::from("dir1"),
                PathBuf::from("dir2")
            ]
        );
    }

    #[test]
    fn test_load_items_with_directories_only() {
        let mut picker = Picker::directory();
        picker.files = vec![PathBuf::from("file1.txt"), PathBuf::from("file2.txt")];
        picker.directories = vec![PathBuf::from("dir1"), PathBuf::from("dir2")];
        picker.load_items();

        assert_eq!(
            picker.items,
            vec![PathBuf::from("dir1"), PathBuf::from("dir2"),]
        );
    }

    #[test]
    fn test_extract_option() {
        let path = Some(Path::new("/test/path"));
        let extracted: Option<PathBuf> = path.extract_option();
        assert_eq!(extracted, Some(PathBuf::from("/test/path")));

        let none_path: Option<&Path> = None;
        let extracted_none: Option<PathBuf> = none_path.extract_option();
        assert_eq!(extracted_none, None);
    }
}
