use json::{MapVisitor, ValueVisitor};

pub struct StringifyValueVisitor;

impl StringifyValueVisitor {
    pub fn new() -> StringifyValueVisitor {
        StringifyValueVisitor
    }
}

impl ValueVisitor<String> for StringifyValueVisitor {
    type MapVisitor = StringifyMapVisitor;

    fn on_str(self, value: &str) -> String {
        format!("\"{}\"", value)
    }

    fn on_num(self, value: f64) -> String {
        value.to_string()
    }

    fn on_map(self, _: Option<usize>) -> Self::MapVisitor {
        StringifyMapVisitor::new()
    }
}

pub struct StringifyMapVisitor {
    empty: bool,
    string: String,
}

impl StringifyMapVisitor {
    pub fn new() -> StringifyMapVisitor {
        StringifyMapVisitor {
            empty: true,
            string: "{".to_owned(),
        }
    }
}

impl MapVisitor<String> for StringifyMapVisitor {
    type ValueVisitor = StringifyValueVisitor;

    fn on_key(&mut self, key: &str) {
        if !self.empty {
            self.string.push(',');
        }
        self.string.push('"');
        self.string.push_str(key);
        self.string.push_str("\":");
        self.empty = false;
    }

    fn on_value(&mut self) -> Self::ValueVisitor {
        StringifyValueVisitor::new()
    }

    fn collect_value(&mut self, value: String) {
        self.string.push_str(&value);
    }

    fn done(mut self) -> String {
        self.string.push('}');
        self.string
    }
}
