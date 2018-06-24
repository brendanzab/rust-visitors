use json::{MapVisitor, Value, ValueVisitor};

pub struct CollectValueVisitor;

impl CollectValueVisitor {
    pub fn new() -> CollectValueVisitor {
        CollectValueVisitor
    }
}

impl ValueVisitor<Value> for CollectValueVisitor {
    type MapVisitor = CollectMapVisitor;

    fn on_str(self, value: &str) -> Value {
        Value::Str(value.to_string())
    }

    fn on_num(self, value: f64) -> Value {
        Value::Num(value)
    }

    fn on_map(self, len: Option<usize>) -> Self::MapVisitor {
        CollectMapVisitor::new(len)
    }
}

pub struct CollectMapVisitor {
    last_key: String,
    pairs: Vec<(String, Value)>,
}

impl CollectMapVisitor {
    pub fn new(len: Option<usize>) -> CollectMapVisitor {
        CollectMapVisitor {
            last_key: String::new(),
            pairs: match len {
                None => Vec::new(),
                Some(len) => Vec::with_capacity(len),
            },
        }
    }
}

impl MapVisitor<Value> for CollectMapVisitor {
    type ValueVisitor = CollectValueVisitor;

    fn on_key(&mut self, key: &str) {
        self.last_key = key.to_owned();
    }

    fn on_value(&mut self) -> Self::ValueVisitor {
        CollectValueVisitor::new()
    }

    fn collect_value(&mut self, value: Value) {
        use std::mem;

        let last_key = mem::replace(&mut self.last_key, String::new());
        self.pairs.push((last_key, value));
    }

    fn done(self) -> Value {
        Value::Map(self.pairs)
    }
}
