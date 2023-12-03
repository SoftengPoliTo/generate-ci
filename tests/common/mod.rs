use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn compare_template_output_with_expected_one(snapshot_path: &PathBuf, template_path: &PathBuf) {
    for entry in WalkDir::new(template_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.path().is_file() {
            compare(snapshot_path, template_path, &entry.path().to_path_buf());
        }
    }
}

pub fn compare_config_toml_wih_expected_one(snapshot_path: &PathBuf, config_path: &PathBuf) {
    let entry = config_path.parent().unwrap();
    compare(snapshot_path, &entry.to_path_buf(), config_path);
}

fn compare(snapshot_path: &PathBuf, path: &PathBuf, entry: &PathBuf) {
    let content = fs::read_to_string(entry).unwrap();
    let name = entry.file_name().unwrap().to_str().unwrap();

    insta::with_settings!({
        snapshot_path => Path::new(snapshot_path)
        .join(entry.strip_prefix(Path::new(path)).unwrap())
        .parent()
        .unwrap(),
        prepend_module_to_snapshot => false
    },{
        insta::assert_snapshot!(name, content);
    });
}
