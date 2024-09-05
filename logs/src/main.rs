use std::{fs, io::Error};

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = Vec::new();

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt");

    let mut error_logs = Vec::new();

    match text {
        Ok(res) => {
            println!("{:?}", res.len());
            error_logs = extract_errors(res.as_str());

            match fs::write("errors.txt", error_logs.join("\n")) {
                Ok(_) => {
                    println!("writing done");
                }
                Err(err) => {
                    println!("{:?}", err);
                }
            }
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

    let text1 = fs::read_to_string("log1s.txt")?;
    println!("{}", text1.len());

    Ok(())
}

fn _divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("cannot div by 0"))
    } else {
        Ok(a / b)
    }
}

fn _validate_email(email: String) -> Result<(), Error> {
    if email.contains('@') {
        Ok(())
    } else {
        Err(Error::other("no"))
    }
}
