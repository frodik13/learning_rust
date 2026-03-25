// === ЗАДАНИЕ 3.5: Error Handling ===

use std::fmt;
use std::num::ParseIntError;

// --- Задача 1: Оператор ? ---
// Напиши функцию, которая парсит строку "ключ=значение" и возвращает (ключ, число).
// Может упасть на: нет '=', значение не число.
// Пока используй String как тип ошибки.
fn parse_key_value(input: &str) -> Result<(String, i32), String> {
    let (key, val) = input.split_once("=").ok_or("нет '='")?;
    let num = val.parse::<i32>().map_err(|e| e.to_string())?;
    Ok((key.to_string(), num))
}

fn task1() {
    println!("{:?}", parse_key_value("temperature=25")); // Ok(("temperature", 25))
    println!("{:?}", parse_key_value("broken")); // Err(...)
    println!("{:?}", parse_key_value("count=abc")); // Err(...)
}

// --- Задача 2: Свой тип ошибки ---
// String как ошибка — плохо: нельзя программно обработать разные случаи.
// Создай enum ConfigError с вариантами:
//   - MissingEquals         — нет символа '='
//   - InvalidValue(ParseIntError) — значение не парсится в число
//
// Реализуй Display и std::error::Error для ConfigError.
// Перепиши parse_key_value_v2 используя ConfigError.

#[derive(Debug)]
enum ConfigError {
    MissingEquals,
    InvalidValue(ParseIntError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingEquals => write!(f, "нет символа '='"),
            ConfigError::InvalidValue(_) => write!(f, "значение не парсится в число"),
        }
    }
}
impl std::error::Error for ConfigError {}

// Подсказка: impl From<ParseIntError> for ConfigError позволит использовать ? напрямую.
impl From<ParseIntError> for ConfigError {
    fn from(value: ParseIntError) -> Self {
        ConfigError::InvalidValue(value)
    }
}

fn parse_key_value_v2(input: &str) -> Result<(String, i32), ConfigError> {
    let split: Vec<&str> = input.split('=').collect();
    if split.len() != 2 {
        Err(ConfigError::MissingEquals)
    } else {
        let int = split[1].parse::<i32>()?;

        Ok((split[0].to_string(), int))
    }
}

fn task2() {
    println!("{:?}", parse_key_value_v2("temperature=25")); // Ok(...)
    println!("error: {}", parse_key_value_v2("broken").unwrap_err());
    println!("error: {}", parse_key_value_v2("count=abc").unwrap_err());
}

// --- Задача 3: Пробрасывание через ? ---
// Функция читает "конфиг" (вектор строк "ключ=значение")
// и возвращает сумму всех значений.
// Если хоть одна строка невалидна — возвращает ошибку.
// Используй parse_key_value_v2 и оператор ?.

fn sum_config_values(lines: &[&str]) -> Result<i32, ConfigError> {
    let mut sum = 0;
    for line in lines {
        let parse = parse_key_value_v2(line)?;
        sum += parse.1;
    }

    Ok(sum)
}

fn task3() {
    let valid = vec!["a=1", "b=2", "c=3"];
    println!("sum: {:?}", sum_config_values(&valid)); // Ok(6)

    let invalid = vec!["a=1", "broken", "c=3"];
    println!("sum: {:?}", sum_config_values(&invalid)); // Err(MissingEquals)
}

// --- Задача 4: Разная обработка разных ошибок ---
// Напиши функцию, которая вызывает parse_key_value_v2 и обрабатывает ошибки:
// - MissingEquals → подставляет значение по умолчанию 0
// - InvalidValue → пробрасывает ошибку дальше
// Используй match на Result.

fn parse_or_default(input: &str) -> Result<(String, i32), ConfigError> {
    match parse_key_value_v2(input) {
        Ok(r) => Ok(r),
        Err(ConfigError::MissingEquals) => Ok((input.to_string(), 0)),
        Err(e) => Err(e),
    }
}

fn task4() {
    println!("{:?}", parse_or_default("temp=25")); // Ok(("temp", 25))
    println!("{:?}", parse_or_default("flag")); // Ok(("flag", 0))
    println!("{:?}", parse_or_default("bad=xyz")); // Err(InvalidValue(...))
}

// --- Задача 5: Collect Result ---
// Фишка Rust: .collect() может собирать Vec<Result<T,E>> в Result<Vec<T>, E>.
// Первая ошибка останавливает сбор.
// Спарси все строки в числа, вернув либо Vec<i32> либо первую ошибку.
fn parse_all(input: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    input
        .iter()
        .map(|x| x.parse::<i32>())
        .collect()
}

fn task5() {
    println!("{:?}", parse_all(&["1", "2", "3"]));       // Ok([1, 2, 3])
    println!("{:?}", parse_all(&["1", "abc", "3"]));     // Err(...)
}

fn main() {
    println!("=== Task 1 ===");
    task1();
    println!("\n=== Task 2 ===");
    task2();
    println!("\n=== Task 3 ===");
    task3();
    println!("\n=== Task 4 ===");
    task4();
    println!("\n=== Task 5 ===");
    task5();
}
