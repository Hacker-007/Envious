pub enum FocusedBlock {
    CodeEditor,
    Output,
}

pub struct App {
    pub index: u16,
    pub line_width: u16,
    pub current_line: u16,
    pub line_count: u16,
    pub code: String,
    pub generated_code: String,
    pub output: Vec<String>,
    pub focused_block: FocusedBlock,
}

impl App {
    pub fn new() -> App {
        App {
            index: 0,
            line_width: 0,
            current_line: 1,
            line_count: 1,
            code: String::new(),
            generated_code: String::new(),
            output: Vec::new(),
            focused_block: FocusedBlock::Output,
        }
    }

    pub fn add_char(&mut self, ch: char) {
        self.code.insert(self.index as usize, ch);
        self.index += 1;
        if ch == '\n' {
            self.line_width = 0;
            self.current_line += 1;
            self.line_count += 1;
        } else {
            self.line_width += 1;
        }
    }

    pub fn add_tab(&mut self) {
        self.code.insert_str(self.index as usize, "    ");
        self.index += 4;
        self.line_width += 4;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
