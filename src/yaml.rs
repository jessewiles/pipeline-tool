pub fn load_pipelines(path: &str) -> Pipelines {
    let f = std::fs::File::open(path).expect("Could not open file.");
    serde_yaml::from_reader(f).expect("Could not read values.")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn test_setup(text: String) -> (PathBuf, TempDir) {
        let tmp_dir = TempDir::new().expect("Bad things");
        let file_path = tmp_dir.path().join("test.yml");
        let mut file = File::create(file_path.clone()).expect("Bad file things");
        writeln!(file, "{}", text).expect("Normal bad things");

        (file_path, tmp_dir)
    }

    fn test_cleanup(file_path: String) {
        drop(file_path);
    }

    #[test]
    fn test_load_pipelines() {
        let yaml_content = "pipelines:
  - name: camus
    exe: fish
    args: orange.fish
"
        .to_string();
        let temp_info = test_setup(yaml_content);
        let file_path = temp_info.0;

        let file_path_s = file_path.as_path().display().to_string();
        let pipelines = load_pipelines(&file_path_s);

        assert_eq!("camus", pipelines.pipelines[0].name);
        assert_eq!("orange.fish", pipelines.pipelines[0].args);

        test_cleanup(file_path_s);
    }
}
