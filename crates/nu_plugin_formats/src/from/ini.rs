use nu_plugin::{EvaluatedCall, LabeledError};
use nu_protocol::{PluginExample, Record, ShellError, Value};

pub const CMD_NAME: &str = "from ini";

pub fn from_ini_call(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let span = input.span();
    let input_string = input.as_string()?;
    let head = call.head;

    let ini_config: Result<ini::Ini, ini::ParseError> = ini::Ini::load_from_str(&input_string);
    match ini_config {
        Ok(config) => {
            let mut sections = Record::new();

            for (section, properties) in config.iter() {
                let mut section_record = Record::new();

                // section's key value pairs
                for (key, value) in properties.iter() {
                    section_record.push(key, Value::string(value, span));
                }

                let section_record = Value::record(section_record, span);

                // section
                match section {
                    Some(section_name) => {
                        sections.push(section_name, section_record);
                    }
                    None => {
                        // Section (None) allows for key value pairs without a section
                        if !properties.is_empty() {
                            sections.push(String::new(), section_record);
                        }
                    }
                }
            }

            // all sections with all its key value pairs
            Ok(Value::record(sections, span))
        }
        Err(err) => Err(ShellError::UnsupportedInput(
            format!("Could not load ini: {err}"),
            "value originates from here".into(),
            head,
            span,
        )
        .into()),
    }
}

pub fn examples() -> Vec<PluginExample> {
    vec![PluginExample {
        example: "'[foo]
a=1
b=2' | from ini"
            .into(),
        description: "Converts ini formatted string to record".into(),
        result: Some(Value::test_record(Record {
            cols: vec!["foo".to_string()],
            vals: vec![Value::test_record(Record {
                cols: vec!["a".to_string(), "b".to_string()],
                vals: vec![Value::test_string("1"), Value::test_string("2")],
            })],
        })),
    }]
}
