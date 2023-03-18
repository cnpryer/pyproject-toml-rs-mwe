use pyproject_toml::PyProjectToml;

fn main() {
    let pyproject_toml_str = r#"[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[project]
name = "mock_project"
version = "0.0.1"
description = ""
dependencies = ["click==8.1.3"]

[[project.authors]]
name = "Chris Pryer"
email = "cnpryer@gmail.com"

[project.optional-dependencies]
dev = [
    "pytest>=6",
    "black==22.8.0",
    "isort==5.12.0",
]
test = []
"#;
    let pyproject_toml = PyProjectToml::new(pyproject_toml_str).expect("failed to parse toml");

    // Not every run will evaluate true
    assert_eq!(
        pyproject_toml_str,
        toml_edit::ser::to_string_pretty(&pyproject_toml).expect("failed to serialize toml")
    );
}
