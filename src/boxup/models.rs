pub enum OverflowHandler {
    Wrap,
    Ellipses,
}

pub enum Alignment {
    Left,
    Center,
    Right,
}

pub struct BoxupOptions {
    pub max_width: usize,
    pub overflow_handler: OverflowHandler,
    pub alignment: Alignment,
    pub line_after_title: bool,
    pub line_after_newline: bool,
}

impl BoxupOptions {
    pub fn new() -> BoxupOptions {
        BoxupOptions {
            max_width: 56,
            overflow_handler: OverflowHandler::Ellipses,
            alignment: Alignment::Left,
            line_after_title: false,
            line_after_newline: false,
        }
    }

    pub fn overflow_handler(self, overflow_handler: OverflowHandler) -> BoxupOptions {
        BoxupOptions {
            overflow_handler,
            ..self
        }
    }

    pub fn alignment(self, alignment: Alignment) -> BoxupOptions {
        BoxupOptions { alignment, ..self }
    }

    pub fn max_width(self, max_width: usize) -> BoxupOptions {
        BoxupOptions { max_width, ..self }
    }

    pub fn line_after_title(self, line_after_title: bool) -> BoxupOptions {
        BoxupOptions {
            line_after_title,
            ..self
        }
    }

    pub fn line_after_newline(self, line_after_newline: bool) -> BoxupOptions {
        BoxupOptions {
            line_after_newline,
            ..self
        }
    }
}
