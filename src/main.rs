// === ЗАДАНИЕ 4.3: Unsafe Rust ===

use std::fmt;

// --- Задача 1: Raw pointers ---
// Создай две переменные i32. Получи raw pointers на них.
// Через unsafe прочитай значения и выведи сумму.
// Потом поменяй значение через *mut pointer.
fn task1() {
    let mut x = 10;
    let y = 20;

    // Создай raw pointers (это safe)
    let ptr_x: *mut i32 = &mut x;
    let ptr_y: *const i32 = &y;

    // Прочитай через unsafe и выведи сумму
    unsafe { println!("sum = {}", *ptr_x + *ptr_y); }

    // Измени x через ptr_x
    unsafe { *ptr_x = 5; }
    println!("x after mutation = {}", x);
}

// --- Задача 2: unsafe функция ---
// Напиши unsafe функцию, которая читает N байт из raw pointer.
// Это реальный паттерн: драйвер читает из memory-mapped регистра.
//
// unsafe fn read_bytes(ptr: *const u8, len: usize) -> Vec<u8>
//
// Внутри: создай Vec, скопируй байты из ptr.
// Подсказка: std::ptr::read() или std::slice::from_raw_parts()

// твой код тут...
unsafe fn read_bytes(ptr: *const u8, len: usize) -> Vec<u8> {
    unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);   
        let mut result = Vec::new();
        result.extend_from_slice(&slice);
        result
    }
}

fn task2() {
    let data: [u8; 5] = [0xDE, 0xAD, 0xBE, 0xEF, 0x42];
    let ptr = data.as_ptr();

    unsafe {
        let bytes = read_bytes(ptr, 5);
        println!("bytes: {:02X?}", bytes); // [DE, AD, BE, EF, 42]
    
        // Читаем только 3 байта с offset 1
        let partial = read_bytes(ptr.add(1), 3);
        println!("partial: {:02X?}", partial); // [AD, BE, EF]
    }
}

// --- Задача 3: transmute — самая опасная функция ---
// std::mem::transmute переинтерпретирует биты одного типа как другой.
// Как reinterpret_cast в C++. Размеры ДОЛЖНЫ совпадать.
//
// Задача: преобразуй [u8; 4] в u32 (little-endian) через transmute.
// Потом сделай то же самое БЕЗОПАСНО через u32::from_le_bytes.
fn task3() {
    let bytes: [u8; 4] = [0x78, 0x56, 0x34, 0x12];

    // unsafe способ
    let value: u32 = unsafe { std::mem::transmute(bytes) };
    println!("transmute: 0x{:08X}", value);

    // safe способ — ВСЕГДА предпочитай этот
    let value_safe = u32::from_le_bytes(bytes);
    println!("from_le_bytes: 0x{:08X}", value_safe);
}

// --- Задача 4: unsafe trait impl ---
// Допустим у нас есть trait, который гарантирует что тип
// безопасно обнулять (все нули — валидное значение).
// Это unsafe trait — реализующий ОБЕЩАЕТ что это правда.
// Компилятор не проверяет — ты берёшь ответственность.

unsafe trait Zeroable {
    fn zeroed() -> Self;
}

// Реализуй Zeroable для u8, u16, u32.
// НЕ реализуй для bool (0 = false, но unsafe trait не стоит
// реализовывать для типов, где это неочевидно).
// НЕ реализуй для String (обнулённый String = dangling pointer = UB).

// unsafe impl Zeroable for ??? { ... }

// Напиши safe обёртку:
// fn zero_init<T: Zeroable>() -> T { T::zeroed() }
unsafe impl Zeroable for u8 {
    fn zeroed() -> Self {
        0x00
    }
}

unsafe impl Zeroable for u16 {
    fn zeroed() -> Self {
        0x00
    }
}

unsafe impl Zeroable for u32 {
    fn zeroed() -> Self {
        0x00
    }
}

fn zero_init<T: Zeroable>() -> T {
    T::zeroed()
}

fn task4() {
    let x: u32 = zero_init();
    let y: u8 = zero_init();
    println!("zeroed u32: {}", x); // 0
    println!("zeroed u8: {}", y);  // 0
}

// --- Задача 5: Когда НЕ использовать unsafe ---
// Перепиши этот unsafe код в safe Rust. Unsafe тут не нужен.
fn find_max_unsafe(data: &[i32]) -> i32 {
    assert!(!data.is_empty());
    unsafe {
        let mut max = *data.get_unchecked(0);
        let mut i = 1;
        while i < data.len() {
            let val = *data.get_unchecked(i);
            if val > max {
                max = val;
            }
            i += 1;
        }
        max
    }
}

fn find_max_safe(data: &[i32]) -> i32 {
    let mut max = data[0];
    for i in 1..data.len() {
        if data[i] > max {
            max = data[i];
        }
    }

    max
}

fn task5() {
    let data = vec![3, 7, 1, 9, 4];
    println!("unsafe: {}", find_max_unsafe(&data));
    println!("safe: {}", find_max_safe(&data));
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
