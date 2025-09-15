use chrono::Local;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        FormError {
            form_values: (field_name, field_value),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        let user = self.name.clone();
        let pwd = self.password.clone();
        if self.name.len() == 0 {
            return Err(FormError::new("name", user, "Username is empty"));
        }
        if pwd.len() < 8 {
            return Err(FormError::new(
                "password",
                pwd,
                "Password should be at least 8 characters long",
            ));
        }
        let has_letter = self.password.chars().any(|c| c.is_ascii_alphabetic());
        let has_number = self.password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = self.password.chars().any(|c| !c.is_ascii_alphanumeric());
        if !(has_letter && has_number && has_symbol) {
            Err(FormError::new(
                "password",
                pwd,
                "Password should be a combination of ASCII numbers, letters and symbols",
            ))
        } else {
            Ok(())
        }
    }
}
