#[derive(Copy, Clone)]
pub enum Flags {
    DrawLines
}

pub fn get_command_line_flags(args: Vec<String>) -> Vec<bool> {
    let mut clf = Vec::new();

    if args.contains(&"-l".to_string()) || args.contains(&"--draw-lines".to_string()) {
        clf.push(true);
    }
    else {
        clf.push(false);
    }

    clf
}
