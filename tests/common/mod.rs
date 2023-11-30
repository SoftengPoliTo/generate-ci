use std::{path::Path, fs};
use walkdir::WalkDir;

pub fn compare_template_output_with_expected_one(snapshot_path: &str, template_path: &str) {

    for entry in WalkDir::new(template_path)
    .follow_links(true)
    .into_iter()
    .filter_map(|e| e.ok()) {
        if entry.path().is_file() {
            let content = fs::read_to_string(Path::new(entry.path())).unwrap();
            let name = entry.path().file_name().unwrap().to_str().unwrap();

            insta::with_settings!({
                snapshot_path => Path::new(snapshot_path)
                .join(entry.path().strip_prefix(Path::new(template_path)).unwrap())
                .parent()
                .unwrap(),
                omit_expression => true
            },{
                insta::assert_snapshot!(name, content)
            });
        }
    }
}
