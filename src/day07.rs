use itertools::Itertools;

enum FileType {
    File(u32),
    Dir(Vec<FileType>),
}

// Lag en vec med bare absolute paths, ikke nested vecs
// eks
// /sdg/fas/gs 2352
// /sdg/fas/ww 14
// /sdg/he 4900
//
// grupper filer
// regn ut true directory size

pub fn part1(input: String) {
    let input: String = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k".into();

    let mut fs: Vec<FileType> = Vec::new();
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

        // println!("{:?}", cd);
    }

    println!("Part 1: {:?}", 0);
}

pub fn part2(input: String) {

    // println!("Part 2: {:?}", input);
}