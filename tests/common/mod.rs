use std::{fs, path::Path};

use walkdir::WalkDir;

pub fn compare_template(snapshot_path: &Path, template_path: &Path) {
    for entry in WalkDir::new(template_path)
        .into_iter() {
            entry.map_or( (), |e| if e.path().is_file() {
                compare(snapshot_path, template_path, e.path());
            })
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
        prepend_module_to_snapshot => false
    },{
        insta::assert_snapshot!(name, content);
    });
}
