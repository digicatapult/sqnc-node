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

#[cfg(test)]
mod tests {
    use dscp_runtime_types::BooleanExpressionSymbol;

    use super::make_pretty_processes;
    use crate::compiler::Process;

    #[test]
    fn transforms_name_single_process() {
        let processes = vec![Process {
            name: vec![116u8, 101u8, 115u8, 116u8].try_into().unwrap(),
            version: 1u32,
            program: vec![BooleanExpressionSymbol::Restriction(
                dscp_runtime_types::Restriction::None,
            )]
            .try_into()
            .unwrap(),
        }];
        let result = make_pretty_processes(&processes);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"[
  {
    "name": "test",
    "program": [
      {
        "Restriction": "None"
      }
    ],
    "version": 1
  }
]"#
            .to_owned()
        );
    }

    #[test]
    fn transforms_name_multiple_process() {
        let processes = vec![
            Process {
                name: vec![116u8, 101u8, 115u8, 116u8, 49u8].try_into().unwrap(),
                version: 1u32,
                program: vec![BooleanExpressionSymbol::Restriction(
                    dscp_runtime_types::Restriction::None,
                )]
                .try_into()
                .unwrap(),
            },
            Process {
                name: vec![116u8, 101u8, 115u8, 116u8, 50u8].try_into().unwrap(),
                version: 1u32,
                program: vec![BooleanExpressionSymbol::Restriction(
                    dscp_runtime_types::Restriction::None,
                )]
                .try_into()
                .unwrap(),
            },
        ];
        let result = make_pretty_processes(&processes);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"[
  {
    "name": "test1",
    "program": [
      {
        "Restriction": "None"
      }
    ],
    "version": 1
  },
  {
    "name": "test2",
    "program": [
      {
        "Restriction": "None"
      }
    ],
    "version": 1
  }
]"#
            .to_owned()
        );
    }

    #[test]
    fn transforms_deep_keys() {
        let processes = vec![Process {
            name: vec![116u8, 101u8, 115u8, 116u8].try_into().unwrap(), // test
            version: 1u32,
            program: vec![BooleanExpressionSymbol::Restriction(
                dscp_runtime_types::Restriction::InputHasMetadata {
                    index: 1u32,
                    metadata_key: vec![107u8, 101u8, 121u8].try_into().unwrap(), // key
                },
            )]
            .try_into()
            .unwrap(),
        }];
        let result = make_pretty_processes(&processes);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            r#"[
  {
    "name": "test",
    "program": [
      {
        "Restriction": {
          "InputHasMetadata": {
            "index": 1,
            "metadata_key": "key"
          }
        }
      }
    ],
    "version": 1
  }
]"#
            .to_owned()
        );
    }
}
