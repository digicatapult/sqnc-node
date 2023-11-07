use std::error::Error;

use serde_json::Value;

use crate::compiler::Process;

fn transform_value(val: Value) -> Value {
    match val {
        Value::Array(arr) => {
            match arr.iter().all(|v| match v.as_u64() {
                Some(n) => n < 256,
                None => false,
            }) {
                true => {
                    let u8_vec = arr.iter().map(|v| v.as_u64().unwrap() as u8).collect();
                    Value::String(String::from_utf8(u8_vec).unwrap())
                }
                false => Value::Array(arr.into_iter().map(transform_value).collect()),
            }
        }
        Value::Object(map) => Value::Object(map.into_iter().map(|(key, val)| (key, transform_value(val))).collect()),
        v => v,
    }
}

pub fn make_pretty_processes(processes: &Vec<Process>) -> Result<String, Box<dyn Error>> {
    let serialised = serde_json::to_value(processes)?;
    let transformed = transform_value(serialised);
    Ok(serde_json::to_string_pretty(&transformed)?)
}
