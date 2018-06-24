use json::{MapVisitor, ValueVisitor};

pub struct RedactValueVisitor<V> {
    downstream: V,
}

impl<V> RedactValueVisitor<V> {
    pub fn new(downstream: V) -> RedactValueVisitor<V> {
        RedactValueVisitor { downstream }
    }
}

impl<V, T> ValueVisitor<T> for RedactValueVisitor<V>
where
    V: ValueVisitor<T>,
{
    type MapVisitor = RedactMapVisitor<V::MapVisitor>;

    fn on_str(self, value: &str) -> T {
        self.downstream.on_str(value)
    }

    fn on_num(self, value: f64) -> T {
        self.downstream.on_num(value)
    }

    fn on_map(self, len: Option<usize>) -> Self::MapVisitor {
        RedactMapVisitor::new(self.downstream.on_map(len))
    }
}

pub struct RedactMapVisitor<V> {
    downstream: V,
    skip: bool,
}

impl<V> RedactMapVisitor<V> {
    pub fn new(downstream: V) -> RedactMapVisitor<V> {
        RedactMapVisitor {
            downstream,
            skip: false,
        }
    }
}

impl<V, T> MapVisitor<T> for RedactMapVisitor<V>
where
    V: MapVisitor<T>,
{
    type ValueVisitor = RedactValueVisitor<V::ValueVisitor>;

    fn on_key(&mut self, key: &str) {
        self.skip = key == "hello";
        if !self.skip {
            self.downstream.on_key(key);
        }
    }

    fn on_value(&mut self) -> Self::ValueVisitor {
        RedactValueVisitor::new(self.downstream.on_value())
    }

    fn collect_value(&mut self, value: T) {
        if !self.skip {
            self.downstream.collect_value(value);
        }
    }

    fn done(self) -> T {
        self.downstream.done()
    }
}
