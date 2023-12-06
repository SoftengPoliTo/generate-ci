use std::{fs, path::Path};

use walkdir::WalkDir;

pub fn compare_template_output_with_expected_one(snapshot_path: &Path, template_path: &Path) {
    for entry in WalkDir::new(template_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().is_file() {
            compare(snapshot_path, template_path, entry.path());
        }
    }
    fs::remove_dir_all(template_path).unwrap();
}

fn compare(snapshot_path: &Path, path: &Path, entry: &Path) {
    let content = fs::read_to_string(entry).unwrap();
    let name = entry.file_name().unwrap().to_str().unwrap();

    insta::with_settings!({
        snapshot_path => snapshot_path
        .join(entry.strip_prefix(path).unwrap())
        .parent()
        .unwrap(),
        prepend_module_to_snapshot => false
    },{
        insta::assert_snapshot!(name, content);
    });
}
