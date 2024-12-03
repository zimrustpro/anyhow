use std::{num::ParseIntError, str::Utf8Error};

#[derive(thiserror::Error, Debug)]
enum SystemError {
    #[error("Couldn't send: {0}")]
    SendError(String),

    #[error("Couldn't parse into a str: {0}")]
    StrFromUtf8Error(#[from] Utf8Error),

    #[error("Wrong color: Red {0} Green {1} Blue {2}")]
    ColorError(u8, u8, u8),

    #[error("Couldn't parse into an i32: {0}")]
    ParseI32Error(#[from] ParseIntError),
    #[error("Something happened")]
    OtherError,
}

fn parse_then_send(input: &[u8]) -> Result<(), SystemError> {
    let some_str = std::str::from_utf8(input)?;
    let number = some_str.parse::<i32>()?;
    send_number(number)?;
    Ok(())
}

fn send_number(number: i32) -> Result<(), SystemError> {
    match number {
        num if num == 500 => Err(SystemError::OtherError),
        num if num > 1_000_000 => Err(SystemError::SendError(format!(
            "{num} is too large, can't send!"
        ))),
        _ => {
            println!("Number sent!");
            Ok(())
        }
    }
}

fn main() {
    println!("{}", parse_then_send(b"nine").unwrap_err());
    println!("{:?}", parse_then_send(b"10").unwrap());
    println!("{}", parse_then_send(b"100000000").unwrap_err());
    println!("{}", SystemError::ColorError(8, 10, 200));
    parse_then_send(b"10098").unwrap();
}
