enum JsonTypes {
    Str(String),
    Int(i32),
    Json,
}

pub enum ParseError {
    InvalidFormat,
}

struct JsonObject<T> {
    key: String,
    value: T,
}

type ParseResult<T> = Result<JsonObject<T>, ParseError>;

fn main() {
    // lol wtf this is so complicated
    // ref: https://pest.rs/book/examples/json.html
    // i think im just gonna steal ideas from there and clean up this after sleep
    parse_json("Hello, world!".to_string()).ok();
}

fn parse_json(json_str: String) -> ParseResult<JsonTypes> {
    /*
    iterate over json string using a stack method for matching quotes and braces
    {"string": "example", "boolean": true, "number", 1, "object": {"string": "example2"}}
    */

    let json_chars = vec!['{', '}', ','];

    let char_stack: Vec<char> = Vec::new();
    let is_key = true;

    for target_char in json_str.chars() {
        // if first character is not open curly brace, fastfail
        if char_stack.len() == 0 && target_char != '{' {
            return Err(ParseError::InvalidFormat);
        }
        if json_chars.contains(&target_char) {
            println!("json character!");
        }
    }

    let result: JsonObject<JsonTypes> = JsonObject {
        key: "objectkey".to_string(),
        value: JsonTypes::Str("This is a value.".to_string()),
    };

    // let result: JsonObject<JsonTypes> = JsonObject {
    //     key: "objectkey".to_string(),
    //     value: JsonTypes::Json(JsonObject {
    //         key: "test".to_string(),
    //         value: JsonTypes::Int(1),
    //     }),
    // };

    Ok(result)
}
