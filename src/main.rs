// === ЗАДАНИЕ 4.4: FFI ===

use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::{c_char, c_double, c_int};

// --- Задача 1: Вызов C-функций из libc ---
// Объяви extern блок с функциями: abs, sqrt (из libm), getpid.
// Вызови каждую.

unsafe extern "C" {
    fn abs(x: c_int) -> c_int;
    fn sqrt(x: c_double) -> c_double;
    fn getpid() -> c_int;
}

fn task1() {
    unsafe {
        println!("abs(-42) = {}", abs(-42));
        println!("sqrt(144) = {}", sqrt(144.0));
        println!("pid = {}", getpid());
    }
}

// --- Задача 2: Строки между C и Rust ---
// C-строка: *const c_char, заканчивается нулём '\0'.
// Rust String: UTF-8, знает длину, нет нуля на конце.
// Нужны конверторы: CString (Rust → C), CStr (C → Rust).
//
// Объяви extern: fn strlen(s: *const c_char) -> usize;
// Создай CString из Rust строки, передай в strlen, выведи результат.

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

fn task2() {
    let rust_str = "Hello from Rust!";
    let c_string = CString::new(rust_str).unwrap();
    let len = unsafe { strlen(c_string.as_ptr()) };
    println!("strlen(\"{}\") = {}", rust_str, len);

    // Обратно: C-строка → Rust &str
    let back_to_rust: &str = unsafe { CStr::from_ptr(c_string.as_ptr()).to_str().unwrap() };
    println!("back: {}", back_to_rust);
}

// --- Задача 3: repr(C) структуры ---
// На RPi через ioctl часто передают структуры в C-код.
// Layout ДОЛЖЕН совпадать.
//
// Создай #[repr(C)] struct Point { x: f64, y: f64 }
// Напиши extern "C" fn distance(p: *const Point) -> f64
// — ЭТО будет Rust-функция с C ABI (можно вызвать из C-кода).
// Внутри: unsafe прочитай поля и верни sqrt(x² + y²).

#[repr(C)]
struct Point {
    x: f64,
    y: f64,
}

extern "C" fn distance(p: *const Point) -> f64 {
    unsafe {
        let x = (*p).x;
        let y = (*p).y;

        (x * x + y * y).sqrt()
    }
}

fn task3() {
    let p = Point { x: 3.0, y: 4.0 };
    let d = distance(&p as *const Point);
    println!("distance = {}", d); // 5.0
}

// --- Задача 4: Safe обёртка над unsafe FFI ---
// Это главный паттерн: unsafe FFI внутри, safe API снаружи.
//
// Задача: оберни C-функцию strtol (строка → число).
// extern "C" { fn strtol(s: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64; }
//
// Создай safe функцию: fn parse_c_int(s: &str, base: i32) -> Option<i64>
// - Конвертируй &str → CString
// - Вызови strtol
// - Если endptr == начало строки (ничего не спарсилось) → None
// - Иначе → Some(результат)

unsafe extern "C" {
    fn strtol(s: *const c_char, endptr: *mut *mut c_char, base: c_int) -> i64;
}

fn parse_c_int(s: &str, base: i32) -> Option<i64> {
    let c_string = CString::new(s).unwrap();
    let start = c_string.as_ptr();
    let mut endptr: *mut c_char = std::ptr::null_mut();
    let result = unsafe { strtol(start, &mut endptr, base) };

    if endptr == start as *mut c_char {
        None
    } else {
        Some(result)
    }
}

fn task4() {
    println!("{:?}", parse_c_int("42", 10)); // Some(42)
    println!("{:?}", parse_c_int("0xFF", 16)); // Some(255)
    println!("{:?}", parse_c_int("1010", 2)); // Some(10)
    println!("{:?}", parse_c_int("hello", 10)); // None
}

// --- Задача 5: Callback из C в Rust ---
// C-код может вызывать Rust-функции через указатели на функции.
// Это используется для callbacks (например, обработчик сигналов на Linux).
//
// Объяви C-функцию qsort:
// void qsort(void *base, size_t nmemb, size_t size,
//             int (*compar)(const void *, const void *));
//
// Напиши Rust callback compare_ints с сигнатурой extern "C".
// Отсортируй массив i32 через qsort.

unsafe extern "C" {
    fn qsort(
        base: *mut std::ffi::c_void,
        nmemb: usize,
        size: usize,
        compar: unsafe extern "C" fn(*const std::ffi::c_void, *const std::ffi::c_void) -> c_int,
    );
}

unsafe extern "C" fn compare_ints(a: *const std::ffi::c_void, b: *const std::ffi::c_void) -> c_int {
    let a_val = unsafe { *(a as *const i32) };
    let b_val = unsafe { *(b as *const i32) };

    a_val - b_val
}

fn task5() {
    let mut data = vec![5, 3, 8, 1, 9, 2];
    println!("before: {:?}", data);

    unsafe {
        qsort(
            data.as_mut_ptr() as *mut std::ffi::c_void,
            data.len(),
            std::mem::size_of::<i32>(),
            compare_ints,
        );
    }

    println!("after:  {:?}", data); // [1, 2, 3, 5, 8, 9]
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
