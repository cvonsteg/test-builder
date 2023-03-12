use clap::Parser;
use std::{
    fs,
    path::{Path, PathBuf},
};

trait TestResource {
    fn resource_name(&self) -> &str;
    fn build(&self, path: &Path);
}

#[derive(PartialEq, Debug, Clone)]
struct TestFile {
    name: String,
}

impl TestFile {
    fn new(name: &str) -> Self {
        TestFile {
            name: String::from(name),
        }
    }
}

impl TestResource for TestFile {
    fn resource_name(&self) -> &str {
        &self.name
    }
    fn build(&self, path: &Path) {
        fs::File::create(path).unwrap();
    }
}

#[derive(PartialEq, Debug, Clone)]
struct TestDirectory {
    name: String,
    dirs: Vec<TestDirectory>,
    files: Vec<TestFile>,
}

impl TestDirectory {
    fn new(name: &str) -> Self {
        TestDirectory {
            name: String::from(name),
            dirs: Vec::<TestDirectory>::new(),
            files: Vec::<TestFile>::new(),
        }
    }

    fn register_file(&mut self, file: TestFile) {
        self.files.push(file);
    }

    fn register_directory(&mut self, directory: TestDirectory) {
        self.dirs.push(directory)
    }
}

impl TestResource for TestDirectory {
    fn resource_name(&self) -> &str {
        &self.name
    }
    fn build(&self, path: &Path) {
        fs::create_dir(path).unwrap();
    }
}

fn create_resource<T: TestResource>(resource: &T, path: &Path) -> PathBuf {
    let full_path = path.join(resource.resource_name());
    if !full_path.exists() {
        resource.build(&full_path);
    }
    full_path
}

fn traverse_and_build(test_dir: &TestDirectory, path: &Path) {
    let root = create_resource(test_dir, path);
    for file in &test_dir.files {
        create_resource(file, &root);
    }
    for dir in &test_dir.dirs {
        traverse_and_build(dir, &root);
    }
}

fn is_init_file(path: &Path) -> bool {
    path.ends_with("__init__.py")
}

fn split_test_path(path: &Path) -> (PathBuf, PathBuf) {
    let path_to_test = path.parent().unwrap();
    let test_dir = path.strip_prefix(path_to_test).unwrap();
    (path_to_test.to_owned(), test_dir.to_owned())
}

fn path_to_test_path(path: &Path) -> String {
    let name = path.file_name().unwrap().to_str().unwrap();
    format!("test_{}", name)
}

fn parse_src_tree(src_path: &Path, test_path: &Path) -> TestDirectory {
    let mut test_dir = TestDirectory::new(test_path.to_str().unwrap());
    if let Ok(entries) = fs::read_dir(src_path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();
            if entry_path.is_file() {
                let file_name = match is_init_file(&entry_path) {
                    true => String::from("__init__.py"),
                    false => path_to_test_path(&entry_path),
                };
                test_dir.register_file(TestFile::new(&file_name));
            } else if entry_path.is_dir() {
                let dir_name = path_to_test_path(&entry_path);
                let sub_dir = parse_src_tree(&entry_path, Path::new(&dir_name));
                test_dir.register_directory(sub_dir);
            } else {
                panic!("UNKNOWN ENTITY")
            }
        }
    }
    test_dir
}

#[derive(Parser, Debug)]
struct CliArgs {
    #[arg(short = 's', long = "source", default_value = "./src")]
    source_path: PathBuf,
    #[arg(short = 't', long = "test", default_value = "./tests")]
    test_path: PathBuf,
}

fn main() {
    // let pathdir = Path::new("./python_code/src");
    // let testdir = Path::new("./python_code/tests");
    let args = CliArgs::parse();
    let (root, test_dir) = split_test_path(&args.test_path);
    let parsed = parse_src_tree(&args.source_path, &test_dir);
    traverse_and_build(&parsed, &root);
}

#[cfg(test)]
mod tests {
    use super::{parse_src_tree, TestDirectory, TestFile};
    use std::path::Path;

    #[test]
    fn test_register_file() {
        // given
        let mut dir = TestDirectory::new(".");
        assert!(dir.files.is_empty());
        let file = TestFile::new("test_file.py");
        let expected = vec![file.clone()];
        // when
        dir.register_file(file);
        // then
        assert_eq!(dir.files, expected);
    }

    #[test]
    fn test_register_dir() {
        // given
        let mut dir = TestDirectory::new(".");
        assert!(dir.dirs.is_empty());
        let new_dir = TestDirectory::new("./src");
        let expected = vec![new_dir.clone()];
        // when
        dir.register_directory(new_dir);
        // then
        assert_eq!(dir.dirs, expected);
    }

    #[test]
    fn test_parsing() {
        // given
        let src_root = Path::new("./python_code/src");
        let test_root = Path::new("tests");

        let mut module_1_dir = TestDirectory::new("test_module_1");
        module_1_dir.files = vec![
            TestFile::new("__init__.py"),
            TestFile::new("test_submodule1.py"),
        ];
        let mut expected = TestDirectory::new("tests");
        expected.dirs = vec![module_1_dir];
        expected.files = vec![TestFile::new("test_main.py")];
        // when
        let result = parse_src_tree(src_root, test_root);
        // then
        assert_eq!(result, expected);
    }
}
