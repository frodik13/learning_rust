// === ЗАДАНИЕ 3.2: Generics, Trait Bounds, Associated Types ===

use std::fmt::{Display};

// --- Задача 1: Generic функция с trait bound ---
// Напиши функцию `largest`, которая принимает слайс &[T] и возвращает ссылку
// на максимальный элемент.
// Подумай: какой trait bound нужен, чтобы сравнивать элементы через > ?
// Сигнатура: fn largest<T: ???>(list: &[T]) -> &T

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for element in list {
        if element > max {
            max = element;
        }
    }

    max
}

// --- Задача 2: Несколько trait bounds ---
// Напиши функцию `print_largest`, которая находит максимальный элемент
// и печатает его. Нужны ДВА bound: для сравнения и для вывода.
// Два варианта синтаксиса — используй where:
//   fn print_largest<T: Bound1 + Bound2>(list: &[T])
//   fn print_largest<T>(list: &[T]) where T: Bound1 + Bound2

// твой код тут...
fn print_largest<T>(list: &[T]) where T: PartialOrd + Display {
    let max = largest(list);
    println!("Max value: {}", max);
}

// --- Задача 3: Generic структура ---
// Создай структуру Pair<T> с двумя полями: first: T, second: T
// Реализуй метод `larger(&self) -> &T` — возвращает ссылку на большее.
// Trait bound нужен только на impl, не на struct (struct хранит любой T).

// твой код тут...
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: PartialOrd> Pair<T> {
    fn larger(&self) -> &T {
        if self.first > self.second {
            &self.first
        } else {
            &self.second
        }
    }
}

// --- Задача 4: Associated type ---
// Создай trait `Register` для работы с регистрами (embedded контекст).
// У регистра есть associated type `Value` — тип значения (u8, u16, u32...).
//
// trait Register {
//     type Value;
//     fn read(&self) -> Self::Value;
//     fn write(&mut self, val: Self::Value);
//     fn address(&self) -> u32;
// }
//
// Создай две структуры:
//   - Reg8  { addr: u32, value: u8 }   → Register с Value = u8
//   - Reg16 { addr: u32, value: u16 }  → Register с Value = u16
//
// Напиши функцию, которая принимает любой Register, читает значение
// и печатает адрес. Подумай: impl Trait или generic? Какой bound на Value
// нужен для println?

// твой код тут...
trait Register {
    type Value;
    fn read(&self) -> Self::Value;
    fn write(&mut self, val: Self::Value);
    fn address(&self) -> u32;
}

struct Reg8 {
    addr: u32,
    value: u8,
}

struct Reg16 {
    addr: u32,
    value: u16,
}

impl Register for Reg8 {
    type Value = u8;

    fn read(&self) -> Self::Value {
        self.value
    }

    fn write(&mut self, val: Self::Value) {
        self.value = val;
    }

    fn address(&self) -> u32 {
        self.addr
    }
}

impl Register for Reg16 {
    type Value = u16;

    fn read(&self) -> Self::Value {
        self.value
    }

    fn write(&mut self, val: Self::Value) {
        self.value = val;
    }

    fn address(&self) -> u32 {
        self.addr
    }
}

fn print_register<R: Register>(reg: &R) where R::Value: Display {
    let value = reg.read();
    let addr = reg.address();
    println!("Address: 0x{:08X}, Value = {}", addr, value);
}

// --- Задача 5: where clause и сложные bounds ---
// Напиши функцию `dump_registers`, которая принимает слайс из &dyn Register<Value = u8>
// и печатает адрес + значение каждого.
// Почему тут нужен конкретный Value = u8, а не generic?

// твой код тут...
fn dump_registers(values: &[&dyn Register<Value = u8>]) {
    for v in values {
        let value = v.read();
        let addr = v.address();
        println!("Address: 0x{:08X}, value = {}", addr, value);
    }
}

fn main() {
    // Задача 1
    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number: {}", largest(&numbers));
    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char: {}", largest(&chars));

    // Задача 2
    print_largest(&numbers);
    print_largest(&chars);

    // Задача 3
    let pair = Pair { first: 10, second: 20 };
    println!("larger: {}", pair.larger());
    let pair_str = Pair { first: "apple", second: "banana" };
    println!("larger: {}", pair_str.larger());

    // Задача 4
    let mut r8 = Reg8 { addr: 0x4000_0000, value: 0xFF };
    let mut r16 = Reg16 { addr: 0x4000_0004, value: 0x1234 };
    print_register(&r8);
    print_register(&r16);
    r8.write(0x00);
    r16.write(0x5678);
    print_register(&r8);
    print_register(&r16);

    // Задача 5
    let regs: Vec<&dyn Register<Value = u8>> = vec![
        &Reg8 { addr: 0x00, value: 0xAA },
        &Reg8 { addr: 0x01, value: 0xBB },
    ];
    dump_registers(&regs);
}
