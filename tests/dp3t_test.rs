use zenroom_rs::runtime::Runtime;
use serde_json::*;
use crate::common::load_json_file;

pub mod common;

#[test]
fn create_sk() {
    let rt = Runtime::new(String::new());
    let script = String::from(r#"
rule check version 1.0.0
rule input encoding hex
rule output encoding hex
Given nothing
When I create the random object of '256' bits
and I rename the 'random object' to 'secret day key'
Then print the 'secret day key'
"#);
    let output = rt.execute(script, Value::Null, Value::Null);

    assert!(output.is_ok());
    println!("{}", serde_json::to_string_pretty(&output.unwrap()).unwrap());
}

#[test]
fn renew_sk() {
    let rt = Runtime::new(String::new());
    let script = String::from(r#"
scenario 'dp3t': Decentralized Privacy-Preserving Proximity Tracing
rule check version 1.0.0
rule input encoding hex
rule output encoding hex
Given I have a valid 'secret day key'
When I renew the secret day key to a new day
Then print the 'secret day key'
"#);
    let data = json!({
            "secret_day_key": "5a25359bb882f7a612fd562c9033de336141572f140c7e24ae7e6ced713208c5"
    });
    let output = rt.execute(script, data, Value::Null);

    assert!(output.is_ok());
    println!("{}", serde_json::to_string_pretty(&output.unwrap()).unwrap());
}

#[test]
fn create_ephids() {
    let rt = Runtime::new(String::new());
    let script = String::from(r#"
scenario 'dp3t': Decentralized Privacy-Preserving Proximity Tracing
rule check version 1.0.0
rule input encoding hex
rule output encoding hex
Given I have a valid 'secret day key'
and I have a valid number in 'moments'
When I create the ephemeral ids for each moment of the day
and I randomize the 'ephemeral ids' array
Then print the 'ephemeral ids'
"#);
    let keys = json!({
            "secret_day_key": "5a25359bb882f7a612fd562c9033de336141572f140c7e24ae7e6ced713208c5"
    });
    let data = json!({ "moments": 8 });
    let output = rt.execute(script, data, keys);

    assert!(output.is_ok());
    println!("{}", serde_json::to_string_pretty(&output.unwrap()).unwrap());
}

#[test]
fn proximity_check() {
    let rt = Runtime::new(String::from("memmanager=sys"));
    let script = String::from(r#"
scenario 'dp3t'
rule check version 1.0.0
rule input encoding hex
rule output encoding hex
Given I have a valid array in 'list of infected'
and I have a valid array in 'ephemeral ids'
When I set 'moments' to '8' base '10'
and I create the proximity tracing of infected ids
Then print the 'proximity tracing'
"#);
    let keys = load_json_file("tests/ephids.json");
    let data = load_json_file("tests/infected_list.json");
    let output = rt.execute(script, data, keys);

    assert!(output.is_ok());
    println!("{}", serde_json::to_string_pretty(&output.unwrap()).unwrap());
}


