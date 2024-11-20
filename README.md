Prompts the user to pick a file interactively from the current directory.

# Arguments

* `label` - A string slice that holds the prompt label to display to the user.

# Returns

* `std::io::Result<PathBuf>` - Returns the `PathBuf` of the selected file.

# Examples

```rust
fn main() -> std::io::Result<()> {
    let file_path = file_picker::file_picker("Select a file:")?;
    println!("You selected the file: {}", file_path.display());

    let dir_path = file_picker::dir_picker("Select a directory:")?;
    println!("You selected the directory: {}", dir_path.display());
}
```
