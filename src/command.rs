
// TODO: implement command option struct
#[derive(Debug, Clone)]
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
            source: String::from(source),
            dest: String::from(dest),
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

