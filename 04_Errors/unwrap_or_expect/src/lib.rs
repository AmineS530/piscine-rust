pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}


pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(),
        Security::Message => match server {
            Ok(s) => s.to_string(),
            Err(_) => panic!("ERROR: program stops"),
        },
        Security::Warning => match server {
            Ok(s) => s.to_string(),
            Err(_) => format!("WARNING: check the server"),
        },
        Security::NotFound => match server {
            Ok(s) => s.to_string(),
            Err(err) => format!("Not found: {}", err),
        },
        Security::UnexpectedUrl => match server {
            Ok(s) => panic!("{}", s),
            Err(err) => format!("{}", err),
        },
    }
}