use regex::Regex;
use std::fs;
use std::iter::Peekable;
use std::slice::Iter;

struct Command<'a> {
    command: &'a str,
    argument: Option<&'a str>,
}

struct File {
    name: String,
    size: Option<usize>,
    kind: FileKind,
}

enum FileKind {
    DIR,
    FILE
}

fn main() {
    // Get text iterator
    let content = fs::read_to_string("sample")
        .expect("Could not read file");
    let lines = content.trim().split('\n')
        .collect::<Vec<&str>>();
    let mut it = lines.iter().peekable();

    // Setup data structures
    let mut cwd: Vec<&str> = vec![];

    while let Some(line) = it.next() {
        let cmd: Command = parse_command(line).expect("Line is not a command");
        println!("Command {} on arg {}", cmd.command, cmd.argument.unwrap_or("<none>"));
        match cmd.command {
            "cd" => change_directory(&mut cwd, cmd.argument.unwrap()),
            "ls" => read_file_listing(&mut it), // TODO: put output into a data structure
            _ => panic!("Bad command"),
        };
    }
}

fn parse_command(line: &str) -> Option<Command> {
    println!("Line: {line}");
    let re = Regex::new(r"^\$\s+(\S+)(?:\s+(\S+))?").unwrap();
    let caps = match re.captures(line) {
        Some(c) => c,
        None => return None,
    };
    let command = caps.get(1).expect("Bad line").as_str();
    let argument = match caps.get(2) {
        Some(m) => Some(m.as_str()),
        None => None,
    };
    Some(Command{command, argument})
}

fn change_directory<'a>(cwd: &mut Vec<&'a str>, path: &'a str) {
    match path {
        "/" => cwd.clear(),
        ".." => {cwd.pop();},
        dir => cwd.push(dir),
    };
}

fn read_file_listing(it: &mut Peekable<Iter<&str>>) -> Vec<File> {
    let files = vec![];
    while it.peek().is_some() && parse_command(it.peek().unwrap()).is_none() {
        let file = parse_file_line(it.next().unwrap());
        files.push(file);
        _print_file_info(file);
    }
    files
}

fn parse_file_line(line: &str) -> File {
    let dir_regex = Regex::new(r"^dir\s+(\S+)$").unwrap();
    let file_regex = Regex::new(r"^(\d+)\s+(\S+)$").unwrap();

    if let Some(caps) = dir_regex.captures(line) {
        File{name: String::from(caps.get(1).unwrap().as_str()), size: None, kind: FileKind::DIR}
    } else if let Some(caps) = file_regex.captures(line) {
        File{name: String::from(caps.get(2).unwrap().as_str()), size: Some(caps.get(1).unwrap().as_str().parse().expect("Should be a number")), kind: FileKind::FILE}
    } else {
        panic!("Bad file line!")
    }
}

fn _print_file_info(file: File) {
    match file.kind {
        FileKind::DIR => println!("Dir named {}", file.name),
        FileKind::FILE => println!("File named {} of size {}", file.name, file.size.unwrap()),
    };
}
