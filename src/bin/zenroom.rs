use zenroom_rs::runtime::*;
use serde_json::Value;

fn main() {
    let script = String::from(r#"
rule check version 1
Scenario 'simple':Create the keypair
Given that I am known as 'Alice'
When I create the keypair
Then print all data"#);
    let rt = Runtime::new(String::new());
    let output = rt.execute(script, Value::Null, Value::Null);

    println!("{:?}", output);
}