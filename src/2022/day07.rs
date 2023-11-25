use itertools::Itertools;

pub fn part1(input: String) {
    let mut fs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    let mut cd = String::new();

    let instructions: Vec<&str> = input
        .split('$')
        .map(|x| x.trim())
        .collect();

    for ins in instructions {
        let command = ins.chars().take(2).join("");

        match command.as_str() {
            "ls" => {
                let contents: Vec<&str> = ins.split('\n').skip(1).collect();

                for new_file in contents {
                    let (a, b): (&str, &str) = new_file.split(' ').collect_tuple().unwrap();
                    match a {
                        "dir" => {

                        },
                        _ => {
                            let mut filename = String::from(cd.clone());
                            filename.push('/');
                            filename.push_str(b);
                            filename.push(' ');
                            filename.push_str(a);
                            fs.push(filename.clone());
                            files.push(filename);
                        }
                    }
                }
            },
            "cd" => {
                let (_, dir): (&str, &str) = ins.split(' ').collect_tuple().unwrap();

                match dir {
                    ".." => {
                        cd = cd.rsplit_once('/').unwrap().0.to_owned();
                    },
                    _ => {
                        if dir != "/" {
                            cd.push('/');
                            cd.push_str(dir);
                        }
                    },
                }
            },
            _ => {},
        }

    }

    let mut dirs: Vec<String> = Vec::new();

    for file in fs {
        let (dir, _) = file.rsplit_once('/').unwrap();
        let mut dir_iter = dir;

        dirs.push(dir_iter.to_owned());
        while !dir_iter.is_empty() {
            let temp = dir_iter.rsplit_once('/').unwrap().0;
            dirs.push(temp.to_owned());
            dir_iter = temp;
        }
        dirs.push("/".into());
    }
    dirs = dirs.into_iter()
        .sorted()
        .dedup()
        .collect();

    let mut dir_sizes = Vec::new();
    for dir in &dirs {
        let mut dir_size = 0;
        for file in &files {
            if file.starts_with(dir) {
                let (_, file_size_str) = file.split_once(' ').unwrap();
                let file_size = file_size_str.parse::<i32>().unwrap();
                dir_size += file_size;
            }
        }
        dir_sizes.push((dir, dir_size));
    }

    let sum: i32 = dir_sizes
        .into_iter()
        .map(|x| x.1)
        .filter(|x| x <= &100_000)
        .sum();

    println!("Part 1: {:?}", sum);
}

pub fn part2(input: String) {
    let mut fs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    let mut cd = String::new();

    let instructions: Vec<&str> = input
        .split('$')
        .map(|x| x.trim())
        .collect();

    for ins in instructions {
        let command = ins.chars().take(2).join("");

        match command.as_str() {
            "ls" => {
                let contents: Vec<&str> = ins.split('\n').skip(1).collect();

                for new_file in contents {
                    let (a, b): (&str, &str) = new_file.split(' ').collect_tuple().unwrap();
                    match a {
                        "dir" => {

                        },
                        _ => {
                            let mut filename = String::from(cd.clone());
                            filename.push('/');
                            filename.push_str(b);
                            filename.push(' ');
                            filename.push_str(a);
                            fs.push(filename.clone());
                            files.push(filename);
                        }
                    }
                }
            },
            "cd" => {
                let (_, dir): (&str, &str) = ins.split(' ').collect_tuple().unwrap();

                match dir {
                    ".." => {
                        cd = cd.rsplit_once('/').unwrap().0.to_owned();
                    },
                    _ => {
                        if dir != "/" {
                            cd.push('/');
                            cd.push_str(dir);
                        }
                    },
                }
            },
            _ => {},
        }

    }

    let mut dirs: Vec<String> = Vec::new();

    for file in fs {
        let (dir, _) = file.rsplit_once('/').unwrap();
        let mut dir_iter = dir;

        dirs.push(dir_iter.to_owned());
        while !dir_iter.is_empty() {
            let temp = dir_iter.rsplit_once('/').unwrap().0;
            dirs.push(temp.to_owned());
            dir_iter = temp;
        }
        dirs.push("/".into());
    }
    dirs = dirs.into_iter()
        .sorted()
        .dedup()
        .collect();

    let mut dir_sizes = Vec::new();
    for dir in &dirs {
        let mut dir_size = 0;
        for file in &files {
            if file.starts_with(dir) {
                let (_, file_size_str) = file.split_once(' ').unwrap();
                let file_size = file_size_str.parse::<i32>().unwrap();
                dir_size += file_size;
            }
        }
        dir_sizes.push((dir, dir_size));
    }

    let dir_sizes2 = dir_sizes.to_owned();
    let (_, largest_dir_size) = dir_sizes
        .into_iter()
        .max_by(|x, y| x.1.cmp(&y.1)).unwrap();
    let current_free_space = 70_000_000 - largest_dir_size;
    let min_space_needed = 30_000_000 - current_free_space;

    let smallest_dir = dir_sizes2
        .to_owned()
        .into_iter()
        .map(|x| x.1)
        .filter(|x| x > &min_space_needed)
        .min().unwrap();


    println!("Part 2: {:?}", smallest_dir);
}