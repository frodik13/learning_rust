// === ЗАДАНИЕ 3.6: Enums как state machines, Newtype ===

use std::fmt::{self, Display, format};

// --- Задача 1: Enum с данными ---
// Создай enum Command для парсера команд на Raspberry Pi:
//   - ReadGpio { pin: u8 }
//   - WriteGpio { pin: u8, value: bool }
//   - SetPwm { pin: u8, duty: f64 }      — duty от 0.0 до 1.0
//   - Sleep { ms: u64 }
//   - Shutdown
//
// Напиши функцию execute(cmd: &Command) -> String
// которая через match возвращает описание действия.
// Для SetPwm проверь что duty в диапазоне [0.0, 1.0], иначе верни ошибку.

// твой код тут...
enum Command {
    ReadGpio { pin: u8 },
    WriteGpio { pin: u8, value: bool },
    SetPwm { pin: u8, duty: f64 },
    Sleep { ms: u64 },
    Shutdown,
}

fn execute(cmd: &Command) -> String {
    match cmd {
        Command::ReadGpio { pin } => format!("Read GPIO pin: {}", pin),
        Command::WriteGpio { pin, value } => format!(
            "Write GPIO pin: {}, value: {}",
            pin,
            if *value { 1 } else { 0 }
        ),
        Command::SetPwm { pin, duty } => {
            if *duty < 0.0 || *duty > 1.0 {
                format!("Error duty: {:.2}", duty)
            } else {
                format!("Set pwn for pin: {}, duty: {:.2}", pin, duty)
            }
        },
        Command::Sleep { ms } => format!("Sleep {}ms", ms),
        Command::Shutdown => "Shutdown".to_string(),
    }
}

fn task1() {
    let commands = vec![
        Command::ReadGpio { pin: 17 },
        Command::WriteGpio { pin: 27, value: true },
        Command::SetPwm { pin: 18, duty: 0.75 },
        Command::Sleep { ms: 1000 },
        Command::Shutdown,
    ];
    for cmd in &commands {
        println!("{}", execute(cmd));
    }
}

// --- Задача 2: State machine ---
// Смоделируй LED, который может быть в трёх состояниях:
//   - Off
//   - On { brightness: u8 }        — 1..=255
//   - Blinking { interval_ms: u64 } — мигает с интервалом
//
// Реализуй методы через impl на enum:
//   - fn turn_on(self, brightness: u8) -> LedState
//   - fn turn_off(self) -> LedState
//   - fn blink(self, interval_ms: u64) -> LedState
//
// Важно: self, не &self — метод ПОТРЕБЛЯЕТ старое состояние и возвращает новое.
// Это гарантирует, что нельзя использовать старое состояние после перехода.

// твой код тут...
enum LedState {
    Off,
    On { brightness: u8},
    Blinking {interval_ms: u64},
}

impl LedState {
    fn turn_on(self, brightness: u8) -> LedState {
        LedState::On { brightness }
    }

    fn turn_off(self) -> LedState {
        LedState::Off
    }

    fn blink(self, interval_ms: u64) -> LedState {
        LedState::Blinking { interval_ms }
    }
}

impl Display for LedState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LedState::Off => write!(f, "Off"),
            LedState::On { brightness } => write!(f, "On({})", brightness),
            LedState::Blinking { interval_ms } => write!(f, "Blinking({}ms)", interval_ms),
        }
    }
}

fn task2() {
    let led = LedState::Off;
    println!("{}", led);           // Off
    let led = led.turn_on(128);
    println!("{}", led);           // On(128)
    let led = led.blink(500);
    println!("{}", led);           // Blinking(500ms)
    let led = led.turn_off();
    println!("{}", led);           // Off
}

// --- Задача 3: Newtype pattern ---
// На RPi ты работаешь с GPIO пинами и I2C адресами.
// Оба — просто числа, легко перепутать.
// Создай newtype обёртки:
//   - GpioPin(u8)
//   - I2cAddress(u8)
//
// Реализуй Display для обоих.
// Напиши функции:
//   - fn read_gpio(pin: GpioPin) -> bool
//   - fn read_i2c(addr: I2cAddress) -> u8
//
// Убедись, что нельзя передать GpioPin вместо I2cAddress.

// твой код тут...
struct GpioPin(u8);
struct I2cAddress(u8);

impl Display for GpioPin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Gpio pin: {}", self.0)
    }
}

impl Display for I2cAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I2cAddress: {}", self.0)
    }
}

fn read_gpio(pin: &GpioPin) -> bool {
    pin.0 > 0 && pin.0 <= 36
}

fn read_i2c(addr: &I2cAddress) -> u8 {
    addr.0
}

fn task3() {
    let pin = GpioPin(17);
    let addr = I2cAddress(0x48);
    
    println!("GPIO {}: {}", pin, read_gpio(&pin));
    println!("I2C {}: 0x{:02X}", addr, read_i2c(&addr));
    //
    // // Это НЕ должно компилироваться (раскомментируй для проверки):
    // read_gpio(addr);   // ошибка типов!
    // read_i2c(pin);     // ошибка типов!
}

// --- Задача 4: Enum + newtype + Result ---
// Объедини всё вместе. Создай:
//   - enum SensorReading — Temperature(f64), Humidity(f64), Pressure(f64)
//   - struct SensorId(u8) — newtype для ID датчика
//   - fn read_sensor(id: SensorId) -> Result<SensorReading, String>
//     (для id.0 == 1 верни Temperature, для 2 — Humidity, для 3 — Pressure,
//      иначе — Err)
//
// Реализуй Display для SensorReading — выводи значение с единицей измерения.

// твой код тут...
enum SensorReading {
    Temperature(f64),
    Humidity(f64),
    Pressure(f64),
}

struct SensorId(u8);

impl Display for SensorReading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensorReading::Temperature(temp) => write!(f, "Temperature: {temp:.1} °C"),
            SensorReading::Humidity(humidity) => write!(f, "Humidity: {humidity:.1} %"),
            SensorReading::Pressure(pressure) => write!(f, "Pressure: {pressure:.1} mm Hg"),
        }
    }
}

fn read_sensor(sensor: SensorId) -> Result<SensorReading, String> {
    match sensor.0 {
        1 => Ok(SensorReading::Temperature(24.4)),
        2 => Ok(SensorReading::Humidity(64.2)),
        3 => Ok(SensorReading::Pressure(733.3)),
        _ => Err("ошибка чтения датчика".to_string()),
    }
}

fn task4() {
    for id in 1..=4 {
        let sensor = SensorId(id);
        match read_sensor(sensor) {
            Ok(reading) => println!("Sensor {}: {}", id, reading),
            Err(e) => println!("Sensor {}: ERROR - {}", id, e),
        }
    }
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
}
