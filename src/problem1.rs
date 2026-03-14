use std::{io::{self, Read, Write}, net::TcpListener};


fn problem1() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:10000")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = Vec::new();

                stream.read_to_end(&mut buf)?;

                stream.write_all(&buf)?;
            }

            Err(err) => {
                println!("connection failed: {}", err)
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        assert!(problem1().is_ok());
    }
}
