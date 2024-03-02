use std::error::Error;
use std::io::Read;
use std::net::{SocketAddr, TcpStream};

use shared::networking::{Fragment, Request};

pub fn validate_server_argument(
    arguments: Vec<String>,
) -> Result<SocketAddr, Box<dyn Error>> {
    match arguments.len() {
        0 | 1 => panic!("Needs one argument: IP address for the server to listen on"),
        2 => (),
        _ => panic!("Too many arguments, only one is needed: IP address of the server"),
    }
    let Ok(server_address) = arguments[1].parse::<SocketAddr>() else {
        panic!(
            "Invalid IP address: {}\nWrite it using the following format 127.0.0.1:8000",
            arguments[1]
        );
    };
    Ok(server_address)
}

pub fn receive_request(
    stream: &mut TcpStream,
) -> Result<Request, Box<dyn Error>> {
    let mut buffer = [0u8; 4];
    stream.read_exact(&mut buffer)?;
    let _total_message_size = u32::from_be_bytes(buffer);
    stream.read_exact(&mut buffer)?;
    let json_message_size = u32::from_be_bytes(buffer) as usize;
    let mut json_buffer = vec![0; json_message_size];
    stream.read_exact(&mut json_buffer)?;
    let json_fragment = String::from_utf8(json_buffer)?;
    let fragment: Fragment = serde_json::from_str(&json_fragment)?;

    match fragment {
        Fragment::FragmentRequest(request) => Ok(request),
        _ => Err("Error: unexpected fragment".into()),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Needs one argument: IP address for the server to listen on")]
    fn test_validate_server_argument_too_few_arguments() {
        let arguments = vec!["server".to_string()];
        validate_server_argument(arguments).unwrap();
    }

    #[test]
    #[should_panic(expected = "Too many arguments, only one is needed: IP address of the server")]
    fn test_validate_server_argument_too_many_arguments() {
        let arguments = vec!["server".to_string(), "".to_string(), "".to_string()];
        validate_server_argument(arguments).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid IP address: \nWrite it using the following format 127.0.0.1:8000")]
    fn test_validate_server_argument_invalid_ip_address() {
        let arguments = vec!["server".to_string(), "".to_string()];
        validate_server_argument(arguments).unwrap();
    }
}