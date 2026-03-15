use std::{io::{BufRead, BufReader, Write}, net::TcpListener, thread};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Request {
    method: String,
    number: f64,
}

#[derive(Serialize, Deserialize)]
struct Response {
    method: String,
    prime: bool,
}

fn read_request(line: String) -> Result<Request, String> {
    let request: Request = serde_json::from_str(&line).map_err(|_| "malformed".to_string())?;

    if request.method != "isPrime".to_string() {
        return Err("malformed".to_string());
    }

    Ok(request)
}

fn is_prime(num: f64) -> bool {
    if num <= 1.0 || num.fract() != 0.0 {
        return false;
    }

    let num_int = num.trunc() as i64;

    for x in 2..=(num.sqrt() as i64) {
        if num_int % x == 0 {
            return false;
        }
    }

    true
}

fn problem2() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:10000")?;

    for stream in listener.incoming() {
        thread::spawn(move || -> Result<(), _> {
            match stream {
                Ok(mut stream) => {
                    let read_stream = stream.try_clone()?;
                    let buf = BufReader::new(read_stream);

                    let mut lines = buf.lines();

                    while let Some(Ok(line)) = lines.next() {
                        println!("line: {}", line);
                        match read_request(line) {
                            Ok( Request { method: _, number } ) => {
                                let response = Response {
                                    method: "isPrime".to_string(),
                                    prime: is_prime(number),
                                };

                                let mut response_string = serde_json::to_string(&response)?;
                                response_string += "\n";

                                stream.write_all(&response_string.as_bytes())?;
                                stream.flush()?;
                            }
                            Err(_) => {
                                stream.write_all("malformed\n".to_string().as_bytes())?;
                                stream.flush()?;
                                break;
                            }
                        }
                    }
                },
                Err(err) => {
                    println!("connection failed: {}", err)
                }
            }

            Ok::<(), std::io::Error>(())
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        println!("test is prime");

        assert!(!is_prime(4.0));

        assert!(is_prime(13.0));

        assert!(is_prime(82244839.0));

        assert!(!is_prime(736440013039892466168685578995649383226029227239946187698.0));
    }

    #[test]
    fn test_problem2() {
        assert!(problem2().is_ok());
    }
}
