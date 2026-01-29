# Contributing to Oxyon

Thank you for your interest in contributing to Oxyon! This guide will help you understand the project structure and how to get started.

## 🏗️ Code Architecture

### Project Overview

Oxyon is built in Rust and organized as follows:

```
oxyon/
├── src/
│   ├── main.rs          # Entry point of the application
│   ├── gui/             # GUI-related code (user interface)
│   ├── converters/      # Conversion logic for each tool
│   │   ├── ffmpeg.rs    # FFmpeg wrapper
│   │   ├── pandoc.rs    # Pandoc wrapper
│   │   ├── exiftool.rs  # ExifTool wrapper
│   │   └── ...
│   ├── utils/           # Helper functions
│   └── config.rs        # Configuration management
├── Cargo.toml           # Dependencies and project metadata
└── build.rs             # Build-time scripts
```

### Key Concepts

1. **Wrappers**: Each converter (FFmpeg, Pandoc, etc.) has its own wrapper that communicates with the external tool
2. **GUI Layer**: The user interface is separate from the conversion logic
3. **Configuration**: User settings are stored in `oxyon_config.toml`

## 🚀 Getting Started

### Setting Up Your Development Environment

1. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone and Build**
   ```bash
   git clone https://github.com/Promethyxx/oxyon.git
   cd oxyon
   cargo build
   ```

3. **Run in Development Mode**
   ```bash
   cargo run
   ```

### Making Changes

1. **Create a new branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Keep code clean and well-commented
   - Follow Rust naming conventions
   - Test your changes locally

3. **Test your code**
   ```bash
   cargo test
   cargo clippy  # Check for common mistakes
   cargo fmt     # Format your code
   ```

4. **Commit your changes**
   ```bash
   git add .
   git commit -m "Add: brief description of your changes"
   ```

5. **Push and create a Pull Request**
   ```bash
   git push origin feature/your-feature-name
   ```

## 📝 Coding Guidelines

### Rust Best Practices

- Use meaningful variable and function names
- Add comments for complex logic
- Handle errors properly (don't use `.unwrap()` excessively)
- Write unit tests for new features

### Example Code Structure

```rust
// Good example
pub fn convert_video(input: &Path, output: &Path, format: VideoFormat) -> Result<(), Error> {
    // Validate input
    if !input.exists() {
        return Err(Error::FileNotFound);
    }
    
    // Perform conversion
    let result = ffmpeg::convert(input, output, format)?;
    
    Ok(result)
}

// Add unit tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_convert_video() {
        // Test logic here
    }
}
```

## 🐛 Reporting Bugs

When reporting bugs, please include:

1. **Operating System**: Windows, macOS, Linux (with version)
2. **Rust Version**: Output of `rustc --version`
3. **Steps to Reproduce**: Detailed steps to recreate the bug
4. **Expected Behavior**: What should happen
5. **Actual Behavior**: What actually happens
6. **Error Messages**: Any error output or logs

## 💡 Suggesting Features

Feature requests are welcome! Please:

1. Check if the feature is already requested in [Issues](https://github.com/Promethyxx/oxyon/issues)
2. Clearly describe the feature and its use case
3. Explain why it would be valuable to users
4. If possible, suggest an implementation approach

## 🔧 Common Development Tasks

### Adding a New File Format

1. Identify which tool supports it (FFmpeg, Pandoc, etc.)
2. Add the format to the relevant converter module
3. Update the GUI to show the new format option
4. Add tests
5. Update documentation

### Modifying the GUI

- GUI code is in `src/gui/`
- Use the existing patterns for consistency
- Test on different screen sizes
- Ensure accessibility (keyboard navigation, contrast, etc.)

### Working with External Tools

When adding or modifying tool wrappers:
- Check tool version compatibility
- Handle tool output and errors gracefully
- Provide clear error messages to users
- Document required tool versions

## 📚 Useful Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Practical examples
- [FFmpeg Documentation](https://ffmpeg.org/documentation.html)
- [Pandoc Documentation](https://pandoc.org/MANUAL.html)

## ❓ Questions?

If you have questions:

1. Check existing [Issues](https://github.com/Promethyxx/oxyon/issues) and [Pull Requests](https://github.com/Promethyxx/oxyon/pulls)
2. Open a new issue with the "question" label
3. Be respectful and patient - maintainers are volunteers

## 🤝 Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Help others learn and grow

Thank you for contributing to Oxyon! 🎉