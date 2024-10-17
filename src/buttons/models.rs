use crate::boxup::{
    boxer::boxup,
    models::{Alignment, BoxupOptions, OverflowHandler},
};

#[derive(Clone)]
pub struct Button {
    pub arrow: String,
    pub text: String,
    pub width: usize,
}

impl Button {
    pub fn new(arrow: String, text: String, width: usize) -> Button {
        Button { arrow, text, width }
    }

    pub fn display(&self) -> String {
        boxup(
            String::new(),
            format!("{}\n{}", self.arrow, self.text),
            BoxupOptions::new()
                .alignment(Alignment::Center)
                .max_width(self.width)
                .overflow_handler(OverflowHandler::Ellipses),
        )
    }

    pub fn update(&mut self, text: String) {
        self.text = text;
    }
}
