use std::collections::HashMap;
use regex::Regex;

/// Validates the username.
///
/// Checks that the username is not empty.
///
/// ## Arguments
///
/// * `username`: The username to validate
///
/// ## Returns
///
/// Returns `Result<(), String>`. Ok if the username passes validation,
/// otherwise returns Err with an error message.
fn validate_username(username: &str) -> Result<(), String> {
    if username.is_empty() {
        return Err("Имя пользователя должно быть заполнено".to_string());
    }

    Ok(())
}
/// Validates the password.
///
/// Checks that the password is not empty and meets certain requirements.
///
/// ## Arguments
///
/// * `password`: The password to validate
///
/// ## Returns
///
/// Returns `Result<(), String>`. Ok if the password passes validation,
/// otherwise returns Err with an error message.
fn validate_password(password: &str) -> Result<(), String> {
    // Проверяем, что пароль не пустой
    if password.is_empty() {
        return Err("Пароль должен быть заполнен".to_string());
    }

    // Проверяем длину пароля (минимум 8 символов)
    if password.len() < 8 {
        return Err("Пароль должен содержать минимум 8 символов".to_string());
    }

    // Проверяем формат пароля (заглавная буква, цифра и т.д.)
    let password_regex = Regex::new(r"^(?=.*[A-Z])(?=.*[0-9]).+$").unwrap();
    if !password_regex.is_match(password) {
        return Err("Пароль должен содержать хотя бы одну заглавную букву и одну цифру".to_string());
    }

    Ok(())
}

/// Validates the registration.
///
/// Checks the username and password for compliance with certain requirements.
///
/// ## Arguments
///
/// * `username`: The username to validate
/// * `password`: The password to validate
///
/// ## Returns
///
/// Returns a `HashMap<String, String>` containing error messages for each field.
fn validate_registration(username: &str, password: &str) -> HashMap<String, String> {
    let mut errors = HashMap::new();

    // Проверяем имя пользователя
    if let Err(error) = validate_username(username) {
        errors.insert("username".to_string(), error);
    }

    // Проверяем пароль
    if let Err(error) = validate_password(password) {
        errors.insert("password".to_string(), error);
    }

    errors
}