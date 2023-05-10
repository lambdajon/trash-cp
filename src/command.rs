// TODO: implement command option struct
#[derive(Debug, Clone, PartialEq)]
pub struct CopyCommand {
    source: String,
    dest: String,
    is_interactive: bool,
    is_recursive: bool,
    verbose: bool,
}

impl CopyCommand {
    pub fn new(source: &str, dest: &str) -> Self {
        Self {
            source: (*source).to_string(),
            dest: (*dest).to_string(),
            is_interactive: false,
            is_recursive: false,
            verbose: false,
        }
    }
    fn recursive(&mut self, val: bool) {
        self.is_recursive = val;
    }

    fn interactive(&mut self, val: bool) {
        self.is_interactive = val;
    }
    fn exec() {
        // need implement execution of copy command
    }
}


#[cfg(test)]
mod command_tests {
    use super::CopyCommand;

    #[test]
    fn chek_build_new_command() {
        let com = CopyCommand::new(&String::from("/home/trash"), &String::from("/home/trash/app"));

        assert_eq!(com, CopyCommand {
            source: String::from("/home/trash"),
            dest: String::from("/home/trash/app"),
            is_interactive: false,
            is_recursive: false,
            verbose: false,
        })
    }
}