#[derive(Clone)]
pub struct Button {
    pub arrow: String,
    pub text: String,
    pub width: u8,
}

impl Button {
    pub fn new(arrow: String, text: String, width: u8) -> Button {
        Button { arrow, text, width }
    }

    pub fn display(&self) -> String {
        let mut buffer = String::new();

        buffer.push('╭');
        buffer.push_str(
            (0..(self.width - 2))
                .map(|_| "─")
                .collect::<String>()
                .as_str(),
        );
        buffer.push_str("╮\n");

        buffer.push_str(format!("│ {: ^24} │\n", self.arrow).as_str());

        let text = if self.text.len() > 26 {
            let subbuffer = &self.text[..21];
            subbuffer.to_string().push_str("...");
            subbuffer
        } else {
            &self.text
        };

        buffer.push_str(format!("│ {: ^24} │\n", text).as_str());

        buffer.push('╰');
        buffer.push_str(
            (0..(self.width - 2))
                .map(|_| "─")
                .collect::<String>()
                .as_str(),
        );
        buffer.push_str("╯\n");

        buffer
    }

    pub fn update(&mut self, text: String) {
        self.text = text;
    }
}