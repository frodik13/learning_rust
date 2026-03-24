// === ЗАДАНИЕ 3.1: Traits ===

use std::{f64, fmt::{self, Display}};

// --- Задача 1: Базовый trait ---
// Создай trait `Sensor` с двумя методами:
//   - fn read_value(&self) -> f64;        (обязательный)
//   - fn unit(&self) -> &str;             (дефолтная реализация, возвращает "unknown")
//
// Создай две структуры:
//   - Thermometer { temperature: f64 }
//   - Accelerometer { x: f64, y: f64, z: f64 }
//
// Реализуй Sensor для обеих:
//   - Thermometer::read_value() → temperature, unit() → "°C"
//   - Accelerometer::read_value() → sqrt(x² + y² + z²), unit() оставь дефолтный

// твой код тут...
trait Sensor {
    fn read_value(&self) -> f64;
    fn unit(&self) -> &str {
        "unknown"
    }
}

struct Thermometer {
    temperature: f64,
}

struct Accelerometer {
    x: f64,
    y: f64,
    z: f64,
}

impl Sensor for Thermometer {
    fn read_value(&self) -> f64 {
        self.temperature
    }
    fn unit(&self) -> &str {
        "°C"
    }
}

impl Sensor for Accelerometer {
    fn read_value(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

// --- Задача 2: impl Trait vs dyn Trait ---
// Раскомментируй и исправь обе функции.

// Эта функция принимает ЛЮБОЙ сенсор — статический dispatch (мономорфизация).
// Компилятор создаст отдельную копию функции для каждого типа.
fn print_reading(sensor: &impl Sensor) {
    println!("{}: {} {}", "reading", sensor.read_value(), sensor.unit());
}

// Эта функция принимает ЛЮБОЙ сенсор — динамический dispatch (vtable).
// Одна копия функции, но вызовы через указатель.
fn log_sensor(sensor: &dyn Sensor) {
    println!("[LOG] {} {}", sensor.read_value(), sensor.unit());
}

// --- Задача 3: Trait как bound ---
// Напиши функцию `max_reading`, которая принимает слайс ссылок на dyn Sensor
// и возвращает максимальное значение read_value().
// Сигнатура: fn max_reading(sensors: &[&dyn Sensor]) -> f64

fn max_reading(sensors: &[&dyn Sensor]) -> f64 {
    let mut max_value = f64::MIN;
    for s in sensors.iter() {
        let value = s.read_value();
        if value > max_value { max_value = value; }
    }

    max_value
}

// --- Задача 4: Display trait ---
// Реализуй std::fmt::Display для Thermometer,
// чтобы println!("{}", therm) выводило "25.5°C"

impl Display for Thermometer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "{:.1}°C", self.temperature);
        Ok(())
    }
}

// --- Задача 5: Derive и стандартные трейты ---
// Добавь нужные #[derive(...)] к структуре, чтобы main скомпилировался.
// Подумай: какие трейты нужны для println!("{:?}"), для ==, для .clone()?
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    // Задача 1
    let t = Thermometer { temperature: 25.5 };
    let a = Accelerometer { x: 1.0, y: 2.0, z: 3.0 };
    println!("{}: {} {}", "therm", t.read_value(), t.unit());
    println!("{}: {} {}", "accel", a.read_value(), a.unit());

    // Задача 2
    print_reading(&t);
    print_reading(&a);
    log_sensor(&t);
    log_sensor(&a);

    // Задача 3
    let sensors: Vec<&dyn Sensor> = vec![&t, &a];
    println!("max = {}", max_reading(&sensors));

    // Задача 4
    println!("display: {}", t);

    // Задача 5
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1.clone();
    println!("{:?}", p1);
    println!("equal: {}", p1 == p2);
}
