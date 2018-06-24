pub use self::collect::{CollectMapVisitor, CollectValueVisitor};
pub use self::redact::{RedactMapVisitor, RedactValueVisitor};
pub use self::stringify::{StringifyMapVisitor, StringifyValueVisitor};

pub mod collect;
pub mod redact;
pub mod stringify;

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Num(f64),
    Map(Vec<(String, Value)>),
}

pub trait ValueVisitor<T> {
    type MapVisitor: MapVisitor<T>;

    fn on_str(self, &str) -> T;
    fn on_num(self, f64) -> T;
    fn on_map(self, Option<usize>) -> Self::MapVisitor;
}

pub trait MapVisitor<T> {
    type ValueVisitor: ValueVisitor<T>;

    fn on_key(&mut self, &str);
    fn on_value(&mut self) -> Self::ValueVisitor;
    fn collect_value(&mut self, T);
    fn done(self) -> T;
}

impl Value {
    pub fn dispatch<T>(&self, visitor: impl ValueVisitor<T>) -> T {
        match *self {
            Value::Str(ref value) => visitor.on_str(value),
            Value::Num(value) => visitor.on_num(value),
            Value::Map(ref pairs) => {
                let mut sub_visitor = visitor.on_map(Some(pairs.len()));
                for (ref key, ref json) in pairs.iter() {
                    sub_visitor.on_key(key);
                    let value = json.dispatch(sub_visitor.on_value());
                    sub_visitor.collect_value(value);
                }
                sub_visitor.done()
            }
        }
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Num(n)
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Value::Num(f64::from(n))
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Str(s)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Self {
        Value::Str(s.to_owned())
    }
}

#[macro_export]
macro_rules! json {
    ({$($key:expr => $val:tt),*}) => {
        $crate::json::Value::Map(vec![$(($key.to_owned(), json!($val))),*])
    };
    ($($key:expr => $val:tt),*) => {
        $crate::json::Value::Map(vec![$(($key.to_owned(), json!($val))),*])
    };
    ($val:expr) => {
        $crate::json::Value::from($val)
    };
}
