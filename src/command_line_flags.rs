use std::convert::TryFrom;

#[derive(Copy, Clone)]
pub enum Flags {
    DrawTrails = 0
}

impl TryFrom<usize> for Flags {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == Flags::DrawTrails as usize => Ok(Flags::DrawTrails),
            _ => Err(()),
        }
    }
}

pub fn get_command_line_flags(args: Vec<String>) -> Vec<usize> {
    let mut clf = Vec::new();
    clf.push(0); // DrawLines
    let draw_lines_option = args.iter().position(|r| r.split('=').next() == Some("-d")
                                                                          || r.split('=').next() == Some("--draw-trails"));

    if draw_lines_option != None {
        let draw_lines_index = draw_lines_option.unwrap();
        if args[draw_lines_index].contains('=') {
            clf[Flags::DrawTrails as usize] = get_numeric_value_after_equal_sign(args[draw_lines_index].clone());
        }
        else {
            clf[Flags::DrawTrails as usize] = 50; // default to 50 circles of trails
        }
    }

    clf
}

pub fn get_numeric_value_after_equal_sign(arg: String) -> usize {
    let mut splitted = arg.split('=');
    let _ = splitted.next(); // eat the value before =
    let value = splitted.next();
    value.unwrap_or_else(|| panic!("No value after argument here: {}", arg)).parse::<usize>().unwrap()
}
