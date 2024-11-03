mod lib;
use std::io;

fn main() {
    let request = match lib::read_input(io::stdin()) {
        Err(why) => panic!("{}", why.to_string()),
        Ok(request) => request,
    };

    match request {
        lib::Request::Valid(json_val) => match json_val.req_type.as_str() {
            "ping" => {
                let value = serde_json::json!({
                    "status": "success",
                    "message": "pong"
                });
                match lib::write_output(io::stdout(), &value) {
                    Err(why) => panic!("{}", why.to_string()),
                    Ok(_) => (),
                }
            }
            "update" => {
                if json_val.host.is_some() {
                    let value = serde_json::json!({
                        "status": "success"
                    });
                    match lib::write_output(io::stdout(), &value) {
                        Err(why) => panic!("{}", why.to_string()),
                        Ok(_) => (),
                    }
                } else {
                    let value = serde_json::json!({
                        "status": "error",
                        "message": "Missing required field 'host' for request type 'update'"
                    });
                    match lib::write_output(io::stdout(), &value) {
                        Err(why) => panic!("{}", why.to_string()),
                        Ok(_) => (),
                    }
                }
            }
            "clear" => {
                let value = serde_json::json!({
                    "status": "success"
                });
                match lib::write_output(io::stdout(), &value) {
                    Err(why) => panic!("{}", why.to_string()),
                    Ok(_) => (),
                }
            }
            _ => {
                let value = serde_json::json!({
                    "status": "error",
                    "message": "Invalid request type"
                });
                match lib::write_output(io::stdout(), &value) {
                    Err(why) => panic!("{}", why.to_string()),
                    Ok(_) => (),
                }
            }
        },
        lib::Request::Invalid(invalid_request) => {
            let value = serde_json::json!({
                "status": "error",
                "message": format!("Invalid request: {}", invalid_request)
            });
            match lib::write_output(io::stdout(), &value) {
                Err(why) => panic!("{}", why.to_string()),
                Ok(_) => (),
            }
        }
    }
}
