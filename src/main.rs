// === ЗАДАНИЕ 4.5: Repr, Alignment, Layout ===

use std::mem;

// --- Задача 1: Размер и выравнивание ---
// Предскажи size_of и align_of для каждой структуры, потом проверь.
// Подсказка: padding добавляется чтобы следующее поле было выровнено.
// Общий размер кратен наибольшему alignment.

struct A {
    x: u8,    // 1 байт, align 1
    y: u32,   // 4 байта, align 4
    z: u8,    // 1 байт, align 1
}

struct B {
    y: u32,   // 4 байта, align 4
    x: u8,    // 1 байт, align 1
    z: u8,    // 1 байт, align 1
}

#[repr(C)]
struct C {
    x: u8,
    y: u32,
    z: u8,
}

#[repr(C)]
struct D {
    y: u32,
    x: u8,
    z: u8,
}

fn task1() {
    // Предскажи значения, потом раскомментируй:
    println!("A: size={}, align={}", mem::size_of::<A>(), mem::align_of::<A>()); // 8 или 12
    println!("B: size={}, align={}", mem::size_of::<B>(), mem::align_of::<B>()); // 8 или 12
    println!("C: size={}, align={}", mem::size_of::<C>(), mem::align_of::<C>()); // 12
    println!("D: size={}, align={}", mem::size_of::<D>(), mem::align_of::<D>()); // 8
    //
    // Вопрос: почему A и B могут отличаться по размеру без repr(C)? Потому что rust может сам добавлять паддинг и порядок полей может поменяться
    // Вопрос: почему C и D отличаются по размеру с repr(C)? Потому что данные упорядочены и u32 должен быть выравнен и кратен 4
}

// --- Задача 2: repr(C) для бинарного протокола ---
// На RPi ты читаешь пакет по UART. Формат фиксирован:
//   Byte 0:     тип пакета (u8)
//   Byte 1:     длина данных (u8)
//   Byte 2-3:   ID устройства (u16, little-endian)
//   Byte 4-7:   значение (f32, little-endian)
//   Byte 8:     checksum (u8)
//
// Создай #[repr(C, packed)] struct Packet с этими полями.
// Напиши fn parse_packet(bytes: &[u8; 9]) -> Packet
// через unsafe { std::ptr::read_unaligned(bytes.as_ptr() as *const Packet) }
//
// Почему packed? Без него компилятор вставит padding после первых двух u8.

// твой код тут...

#[repr(C, packed)]
struct Packet {
    packet_type: u8,
    length: u8,
    device_id: u16,
    value: f32,
    checksum: u8,
}

fn parse_packet(bytes: &[u8; 9]) -> Packet {
    unsafe {
        std::ptr::read_unaligned(bytes.as_ptr() as *const Packet)
    }
}

fn task2() {
    let raw: [u8; 9] = [
        0x01,             // тип: 1
        0x04,             // длина: 4
        0x0A, 0x00,       // device_id: 10 (little-endian)
        0x00, 0x00, 0xC8, 0x42, // value: 100.0f32 (little-endian)
        0xFF,             // checksum
    ];
    let packet = parse_packet(&raw);
    let device_id = packet.device_id;
    let value = packet.value;
    println!("type: {}", packet.packet_type);
    println!("len: {}", packet.length);
    println!("device: {}", device_id);
    println!("value: {}", value);
    println!("checksum: 0x{:02X}", packet.checksum);
    println!("packet size: {}", mem::size_of::<Packet>());  // должно быть 9!
}

// --- Задача 3: repr(transparent) для newtype ---
// repr(transparent) гарантирует что newtype имеет ТОЧНО такой же layout
// как inner type. Это важно для FFI — можно передавать newtype где ожидают inner.
//
// Создай #[repr(transparent)] struct Celsius(f64);
// Создай #[repr(transparent)] struct Register(u32);
//
// Проверь что size_of и align_of совпадают с inner типом.

// твой код тут...
#[repr(transparent)] struct Celsius(f64);
#[repr(transparent)] struct Register(u32);

fn task3() {
    println!("f64:     size={}, align={}", mem::size_of::<f64>(), mem::align_of::<f64>());
    println!("Celsius: size={}, align={}", mem::size_of::<Celsius>(), mem::align_of::<Celsius>());
    println!("u32:      size={}, align={}", mem::size_of::<u32>(), mem::align_of::<u32>());
    println!("Register: size={}, align={}", mem::size_of::<Register>(), mem::align_of::<Register>());
}

// --- Задача 4: Enum layout ---
// Enum хранит discriminant (тег) + данные самого большого варианта.
// Предскажи размеры:

enum Small {
    A,
    B,
    C,
}

enum WithData {
    Empty,
    Byte(u8),
    Word(u32),
    Big([u8; 32]),
}

// Бонус: Option<Box<T>> имеет ТАКОЙ ЖЕ размер как Box<T>. Почему?
// (niche optimization: Box<T> не может быть null, поэтому None = null)

fn task4() {
    println!("Small: size={}", mem::size_of::<Small>()); // 4 + 4 а тут почему 1?
    println!("WithData: size={}", mem::size_of::<WithData>()); // 32 + 4
    println!("Box<i32>: size={}", mem::size_of::<Box<i32>>()); // 4 + 4(ссылка)
    println!("Option<Box<i32>>: size={}", mem::size_of::<Option<Box<i32>>>()); // 4 + 4(ссылка) Не знаю почему так
}

// --- Задача 5: offset_of! и ручная работа с layout ---
// Иногда нужно знать смещение поля в структуре (для ioctl, memory-mapped IO).
// std::mem::offset_of! (стабилен с Rust 1.77)

#[repr(C)]
struct SensorRegisters {
    status: u8,       // offset 0
    _reserved: [u8; 3], // padding
    temperature: u32, // offset 4
    humidity: u32,    // offset 8
    pressure: u32,    // offset 12
}

fn task5() {
    println!("SensorRegisters size: {}", mem::size_of::<SensorRegisters>());
    println!("status offset:      {}", mem::offset_of!(SensorRegisters, status));
    println!("temperature offset: {}", mem::offset_of!(SensorRegisters, temperature));
    println!("humidity offset:    {}", mem::offset_of!(SensorRegisters, humidity));
    println!("pressure offset:    {}", mem::offset_of!(SensorRegisters, pressure));
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
