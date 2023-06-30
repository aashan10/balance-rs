
#[derive(Debug, Clone)]
pub struct SourceText {
    pub text: String,
    lines: Vec<TextLine>,
}

impl SourceText {
    pub fn new(text: String) -> SourceText {
        let mut source = SourceText {
            text,
            lines: Vec::new(),
        };

        source.lines = source.parse_lines(source.text.clone());
        source

    }

    fn parse_lines(&self, text: String) -> Vec<TextLine> {
        let mut lines = Vec::new();
        let mut position = 0;
        let mut line_start = 0;

        while position < text.len() {
            let line_break_width = self.get_line_break_width(&text, position);
            if line_break_width == 0 {
                position += 1;
            } else {
                lines.push(TextLine::new(
                    self,
                    line_start,
                    position - line_start,
                    line_break_width,
                ));
                position += line_break_width;
                line_start = position;
            }
        }

        if position >= line_start {
            lines.push(TextLine::new(self, line_start, position - line_start, 0));
        }

        lines
    }


    pub fn span_to_string(&self, span: TextSpan) -> String {
        self.bounds_to_string(span.start, span.end)
    }

    pub fn to_string(&self) -> String {
        self.text.clone()
    }

    pub fn bounds_to_string(&self, start: usize, end: usize) -> String {
        self.text[start..end].to_string()
    }

    fn get_line_break_width(&self, text: &String, position: usize) -> usize {
        // get character at position of text
        let c = text.as_bytes()[position] as char;

        let l = if position + 1 >= text.len() {
            '\0'
        } else {
            text.as_bytes()[position + 1] as char
        };

        if c == '\r' && l == '\n' {
            return 2;
        }

        if c == '\r' || c == '\n' {
            return 1;
        }

        return 0;
    }
}


#[derive(Debug, Clone)]
struct TextLine {
    source: SourceText,
    start: usize,
    length: usize,
    length_with_line_breaks: usize,
}

impl TextLine {
    pub fn new(
        source: &SourceText,
        start: usize,
        length: usize,
        length_with_line_breaks: usize,
    ) -> Self {
        Self {
            source: source.clone(),
            start,
            length,
            length_with_line_breaks,
        }
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn get_length_with_line_breaks(&self) -> usize {
        self.length_with_line_breaks
    }

    pub fn get_end(&self) -> usize {
        self.start + self.length
    }

    pub fn get_span(&self) -> TextSpan {
        TextSpan::new(self.start, self.length)
    }

    pub fn get_span_with_line_breaks(&self) -> TextSpan {
        TextSpan::new(self.start, self.length_with_line_breaks)
    }

    pub fn to_string(&self) -> String {
        self.source.span_to_string(self.get_span())
    }
}

struct TextSpan {
    start: usize,
    length: usize,
    end: usize,
}

impl TextSpan {
    fn new(start: usize, length: usize) -> Self {
        Self {
            start,
            length,
            end: start + length,
        }
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_length(&self) -> usize {
        self.length
    }

    fn get_end(&self) -> usize {
        self.end
    }
}
