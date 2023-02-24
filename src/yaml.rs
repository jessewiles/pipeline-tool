pub fn load_pipelines(path: &str) -> Pipelines {
    let f = std::fs::File::open(path).expect("Could not open file.");
    serde_yaml::from_reader(f).expect("Could not read values.")
}
