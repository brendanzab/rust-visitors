#[macro_use]
mod json;

fn main() {
    let json = json! {
        "hello" => {
            "i am" => { "cow" => 1 },
            "you are" => { "cow" => 2 }
        },
        "world" => 31337,
        "bye" => "314"
    };

    println!("json = {:?}", &json);
    println!(
        "minified = {}",
        json.dispatch(json::StringifyValueVisitor::new())
    );
    println!(
        "redacted = {}",
        json.dispatch(json::RedactValueVisitor::new(
            json::StringifyValueVisitor::new(),
        ))
    );
    println!(
        "redacted_json = {:?}",
        json.dispatch(json::RedactValueVisitor::new(
            json::CollectValueVisitor::new(),
        ))
    );
}
