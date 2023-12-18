use ci_generate::cargo::SKIPPED_FOLDERS;
use std::{fs, path::Path};
use walkdir::WalkDir;

pub fn compare_template(snapshot_path: &Path, template_path: &Path) {
    // https://docs.rs/walkdir/latest/walkdir/struct.IntoIter.html#method.skip_current_dir
    let mut it = WalkDir::new(template_path).into_iter();
    loop {
        let entry = match it.next() {
            None => break,
            Some(entry) => entry.unwrap(),
        };
        let file_name = entry.file_name().to_string_lossy().to_string();
        let skip_entry = SKIPPED_FOLDERS.contains(&file_name.as_str());

        if skip_entry && entry.file_type().is_dir() {
            it.skip_current_dir();
            continue;
        }
        if entry.file_type().is_file() && !SKIPPED_FOLDERS.contains(&file_name.as_str()) {
            compare(snapshot_path, template_path, entry.path());
        }
    }
}

fn compare(snapshot_path: &Path, path: &Path, entry: &Path) {
    let content = fs::read_to_string(entry).unwrap();
    let name = entry.file_name().and_then(|v| v.to_str());
    insta::with_settings!({
        snapshot_path => snapshot_path
        .join(entry.strip_prefix(path).unwrap())
        .parent()
        .unwrap(),
        prepend_module_to_snapshot => false,
    },{
        insta::assert_snapshot!(name, content);
    });
}
