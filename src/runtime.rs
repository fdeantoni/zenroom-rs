#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde_json::Value;
use std::ffi::CString;
use crate::ZenError;

const BUF_SIZE: usize = 1024 * 128;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct Runtime {
    conf: String,
}

impl Runtime {

    pub fn new(conf: String) -> Self {
        Runtime {
            conf,
        }
    }

    pub fn execute(&self, script: String, data: Value, keys: Value) -> Result<Value, ZenError> {
        let c_conf = CString::new(self.conf.clone()).unwrap();
        let c_keys = valueToCString(keys);
        let c_data = valueToCString(data);
        let c_script = CString::new(script).unwrap();

        let result = execute_zen(c_conf, c_script, c_data, c_keys);
        result.and_then(|string| {
            serde_json::from_str(string.as_str()).map_err(|error| {
                let msg = format!("Error converting Zenroom result {} to JSON. Reason: {}", string, error.to_string());
                ZenError::new(msg)
            })
        })
    }
}

fn valueToCString(value: Value) -> CString {
    if !value.is_null() {
        let string = serde_json::to_string(&value).unwrap();
        CString::new(string).unwrap()
    } else {
        CString::new("").unwrap()
    }
}

fn execute_zen(conf: CString, script: CString, data: CString, keys: CString) -> Result<String, ZenError> {

    let mut stdout = Vec::<i8>::with_capacity(BUF_SIZE);
    let stdout_ptr = stdout.as_mut_ptr();
    let mut stderr = Vec::<i8>::with_capacity(BUF_SIZE);
    let stderr_ptr = stderr.as_mut_ptr();

    let output = unsafe {
        std::mem::forget(stdout);
        std::mem::forget(stderr);

        let _result = zencode_exec_tobuf(
            script.into_raw(),
            conf.into_raw(),
            keys.into_raw(),
            data.into_raw(),
            stdout_ptr,
            BUF_SIZE as u64,
            stderr_ptr,
            BUF_SIZE as u64
        );

        let stdout_buffer = Vec::from_raw_parts(
            stdout_ptr,
            BUF_SIZE,
            BUF_SIZE);
        let stdout_output = String::from_utf8(std::mem::transmute(stdout_buffer)).map(|res| {
            res.trim_matches(char::from(0)).to_string()
        });

        let stderr_buffer = Vec::from_raw_parts(
            stderr_ptr,
            BUF_SIZE,
            BUF_SIZE);
        let stderr_output = String::from_utf8(std::mem::transmute(stderr_buffer)).map(|res| {
            res.trim_matches(char::from(0)).to_string()
        });

        stdout_output.and_then(|string| {
            if string.is_empty() {
                stderr_output
            } else {
                Ok(string)
            }
        })
    };

    output.map_err(|error| {
        let msg = format!("Failed to convert buffers to string. Reason: {}", error.to_string());
        ZenError::new(msg)
    })
}

#[cfg(test)]
mod tests {
    use crate::runtime::*;
    use serde_json::Value;

    #[test]
    fn basic() {

        let rt = Runtime::new(String::new());
        let script = String::from(r#"
rule check version 1
Scenario 'simple':Create the keypair
Given that I am known as 'Alice'
When I create the keypair
Then print all data"#);
        let output = rt.execute(script, Value::Null, Value::Null);

        assert!(output.is_ok());
        println!("{}", serde_json::to_string_pretty(&output.unwrap()).unwrap());
    }
}

