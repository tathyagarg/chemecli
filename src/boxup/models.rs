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
}

impl BoxupOptions {
    pub fn new() -> BoxupOptions {
        BoxupOptions {
            max_width: 54,
            overflow_handler: OverflowHandler::Ellipses,
            alignment: Alignment::Left,
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
}
