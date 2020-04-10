#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

const BUF_SIZE: usize = 1024 * 128;

pub fn execute(script: String) -> Result<String, String> {
    let script = CString::new(script).unwrap();
    let keys = CString::new("").unwrap();
    let conf = CString::new("debug=1").unwrap();
    let data = CString::new("").unwrap();

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

        (stdout_output, stderr_output)
    };

    let stdout = output.0.unwrap();
    let stderr = output.1.unwrap();

    return if !stderr.contains("error") {
        Ok(stdout)
    } else {
        Err(stderr)
    }
}




#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn empty() {

        let script = String::from(r#"
rule check version 1
Scenario 'simple':Create the keypair
Given that I am known as 'Alice'
When I create the keypair
Then print all data"#);
        let output = execute(script);

        println!("{:?}", output)
    }
}
