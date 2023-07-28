use std::convert::TryFrom;

#[derive(Copy, Clone)]
pub enum Flags {
    DrawLines = 0
}

impl TryFrom<usize> for Flags {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == Flags::DrawLines as usize => Ok(Flags::DrawLines),
            _ => Err(()),
        }
    }
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
