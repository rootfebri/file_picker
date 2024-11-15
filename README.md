Prompts the user to pick a file interactively from the current directory.

# Arguments

* `label` - A string slice that holds the prompt label to display to the user.

# Returns

* `std::io::Result<PathBuf>` - Returns the `PathBuf` of the selected file.

# Examples

```rust
fn main() {
    use crate::file_picker::file_picker;
    let file_path = file_picker("Select a file:").expect("Failed to pick a file");
    println!("You selected the file: {:?}", file_path);
}
```
