pub const PLACEHOLDER: &str = "fixtures";

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    /// Test that all .md files contain only printable ASCII characters (0x20-0x7E plus newlines/tabs).
    ///
    /// While GitHub supports UTF-8/Unicode, using ASCII-only ensures maximum
    /// compatibility across all tools and prevents display issues from:
    /// - Control characters (0x00-0x1F except newline/tab)
    /// - Invisible Unicode characters (zero-width spaces, non-breaking spaces)
    /// - Characters that may render differently across systems
    ///
    /// See docs/process.md section "Markdown File Encoding" for guidance.
    #[test]
    fn test_markdown_files_are_ascii_only() {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap();

        let mut errors = Vec::new();

        // Recursively find all .md files
        fn find_md_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        // Skip target and .git directories
                        if let Some(name) = path.file_name()
                            && (name == "target" || name == ".git" || name == "node_modules")
                        {
                            continue;
                        }
                        find_md_files(&path, files);
                    } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                        files.push(path);
                    }
                }
            }
        }

        let mut md_files = Vec::new();
        find_md_files(repo_root, &mut md_files);

        // Check each .md file for non-ASCII characters
        for md_file in md_files {
            if let Ok(content) = fs::read(&md_file) {
                for (line_num, line) in content.split(|&b| b == b'\n').enumerate() {
                    if let Some(pos) = line.iter().position(|&b| b > 127) {
                        let byte = line[pos];
                        let rel_path = md_file.strip_prefix(repo_root).unwrap_or(&md_file);
                        errors.push(format!(
                            "{}:{}: Non-ASCII byte 0x{:02X} at column {}",
                            rel_path.display(),
                            line_num + 1,
                            byte,
                            pos + 1
                        ));
                        break; // Only report first non-ASCII per file
                    }
                }
            }
        }

        if !errors.is_empty() {
            panic!(
                "\n\nERROR: Markdown files contain non-ASCII characters:\n\n{}\n\n\
                All .md files must use ASCII-only encoding (subset of UTF-8).\n\
                This ensures maximum compatibility and prevents display issues.\n\n\
                See docs/process.md section \"Markdown File Encoding\" for guidance.\n\
                Use ASCII equivalents: 'us' instead of 'µs', '<=' instead of '≤', etc.\n",
                errors.join("\n")
            );
        }
    }
}
