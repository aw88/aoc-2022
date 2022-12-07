use aoc_utils::*;

#[derive(Debug)]
struct Directory {
    name: String,
    subdirectories: Vec<Directory>,
    files: Vec<File>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn create(name: String, size: usize) -> File {
        File { name, size }
    }
}

impl Directory {
    fn create(name: String) -> Directory {
        Directory {
            name,
            subdirectories: vec![],
            files: vec![],
        }
    }

    fn add_subdirectory(&mut self, subdirectory: Directory) {
        self.subdirectories.push(subdirectory);
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn size(&self) -> usize {
        let files_size = self.files.iter().map(|f| f.size).sum::<usize>();
        let dirs_size = self.subdirectories.iter().map(|d| d.size()).sum::<usize>();
        files_size + dirs_size
    }

    fn sizes(&self) -> Vec<usize> {
        self.subdirectories
            .iter()
            .flat_map(|dir| dir.sizes())
            .chain([self.size()])
            .collect()
    }
    
    // fn traverse(&self, f: impl Fn(&Directory)) {
    //     for dir in self.subdirectories.iter() {
    //         dir.traverse(&f);
    //     }
    //     f(self);
    // }
}

fn build_filesystem(lines: Vec<String>) -> Directory {
    let mut current_dir = vec![Directory::create("/".to_string())];

    for line in lines {
        match line.as_str() {
            "$ ls" => (),
            "$ cd /" => (),
            "$ cd .." => {
                let directory = current_dir.pop().unwrap();
                current_dir.last_mut().unwrap().add_subdirectory(directory);
            }
            line if line.starts_with("$ cd") => {
                let dir_name = line
                    .split_once(" cd ")
                    .map(|(_, name)| name)
                    .expect("Parse error");
                current_dir.push(Directory::create(dir_name.to_string()));
            }
            line if line.starts_with("dir") => (),
            line => {
                let (size, name) = line.split_once(' ').expect("Parse error");
                current_dir.last_mut().unwrap().add_file(File::create(
                    name.to_string(),
                    size.parse().expect("Parse error"),
                ));
            }
        }
    }

    while current_dir.len() > 1 {
        let directory = current_dir.pop().unwrap();
        current_dir.last_mut().unwrap().add_subdirectory(directory);
    }

    current_dir.into_iter().next().unwrap()
}

fn main() {
    let filename = input_filename();
    let input = read_lines(&filename);

    let filesystem = build_filesystem(input);
    let dir_sizes = filesystem.sizes();

    // Part 1
    let answer = dir_sizes
        .iter()
        .filter(|&&size| size < 100_000)
        .sum::<usize>();

    println!("The answer to part one is: {:?}", answer);

    // Part 2
    let total_disk = 70_000_000;
    let required_space = 30_000_000;
    let used_space = filesystem.size();
    let unused_space = total_disk - used_space;
    let space_to_free = required_space - unused_space;

    let answer = dir_sizes.iter().filter(|&&size| size > space_to_free).min().unwrap();
    println!("The answer to part two is: {:?}", answer);
}
