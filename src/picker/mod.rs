use colored::Colorize;
use dialoguer::theme::{ColorfulTheme, Theme};
use std::env::current_dir;
use std::path::PathBuf;

mod pick_types;
pub use pick_types::PickType;

pub struct Picker {
    ci: usize,
    cwd: PathBuf,
    directories: Vec<PathBuf>,
    files: Vec<PathBuf>,
    pick_type: PickType,
    prompt: Option<String>,
    items: Vec<PathBuf>,
    theme: Box<dyn Theme>,
}

impl Picker {
    const FOLDER_ICON: &'static str = "📁"; // Icon for folders
    const FILE_ICON: &'static str = "📄"; // Icon for files
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
        let cwd = current_dir().expect("Failed to initialize current dir, use `with_cwd` instead");

        let mut this = Self {
            ci: 0,
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

    pub fn select(&mut self) -> dialoguer::Result<PathBuf> {
        loop {
            let display = self.make_display(&self.items);
            let index = self.pick(&display)?;
            self.ci = index;

            match self.pick_type {
                PickType::Directory => {
                    if index == 0 {
                        break Ok(self.cwd.clone());
                    } else if self.cwd.parent().is_some() && index == 1 {
                        self.cwd.pop();
                    } else {
                        self.cwd = self.cwd.join(&self.items.as_slice()[index]);
                    }
                }
                PickType::File => {
                    if self.cwd.parent().is_some() && index == 0 {
                        self.cwd.pop();
                    } else {
                        let path = &self.items.as_slice()[index];
                        if path.is_file() {
                            break Ok(path.to_owned());
                        } else {
                            self.cwd = self.cwd.join(path);
                        }
                    }
                }
            }

            self.process_files();
        }
    }

    fn process_files(&mut self) {
        self.items.clear();
        if self.pick_type == PickType::Directory {
            self.items.push(".".into());
        }

        if self.cwd.parent().is_some() {
            self.items.push("..".into());
        }

        if let Ok(entry) = self.cwd.read_dir() {
            for dir in entry.filter_map(Result::ok) {
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

    fn simplify_path(&self) -> String {
        let mut relative_path = String::new();
        let mut temp_path = self.cwd.clone();
        while let Some(parent) = temp_path.parent() {
            relative_path.push_str(&(".".to_owned() + Self::DS));
            temp_path = parent.to_path_buf();
        }

        relative_path
    }

    fn pick(&mut self, display: &[String]) -> dialoguer::Result<usize> {
        let theme = ColorfulTheme::default();
        let mut dialog = dialoguer::FuzzySelect::with_theme(&theme);

        if let Some(ref prompt) = self.prompt {
            let current_prompt = if let Some(name) = self.cwd.file_name() {
                let simplified = self.simplify_path();
                let pcwd = format!("{}{}{}", Self::DS, simplified, name.to_string_lossy());
                format!("{} {}", prompt, pcwd.dimmed())
            } else {
                prompt.to_string()
            };

            dialog = dialog.with_prompt(current_prompt);
        } else {
            let current_prompt = if let Some(name) = self.cwd.file_name() {
                let simplified = self.simplify_path();
                let pcwd = format!("{}{}{}", Self::DS, simplified, name.to_string_lossy());
                pcwd.dimmed().to_string()
            } else {
                Self::DS.dimmed().to_string()
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
        if let Some(path) = items.first() {
            if *path == PathBuf::from(".") {
                displays.push(".".into());
            } else if *path == PathBuf::from("..") {
                displays.push("..".into());
            }
        }
        if let Some(path) = items.get(1) {
            if *path == PathBuf::from(".") {
                displays.push(".".into());
            } else if *path == PathBuf::from("..") {
                displays.push("..".into());
            }
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
}

pub trait PickerBuilder {
    fn with_theme(self, theme: impl Theme + 'static) -> Self;
    fn with_prompt(self, prompt: impl Into<String>) -> Self;
    fn with_cwd(self, cwd: impl Into<PathBuf>) -> Self;
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

    fn with_cwd(mut self, cwd: impl Into<PathBuf>) -> Self {
        self.cwd = cwd.into();
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
}
