// === ЗАДАНИЕ 4.2: Drop trait ===

use std::cell::RefCell;
use std::rc::Rc;

// --- Задача 1: Предскажи порядок дропа ---
// Структура с именем, которая печатает при дропе.
struct Named {
    name: String,
}

impl Drop for Named {
    fn drop(&mut self) {
        println!("  drop: {}", self.name);
    }
}

// Предскажи порядок вывода, потом запусти и проверь.
fn task1() {
    println!("--- переменные ---");
    let _a = Named { name: "A".into() };
    let _b = Named { name: "B".into() };
    let _c = Named { name: "C".into() };
    // В каком порядке дропнутся?
    // _a, _b, _c.
}

// --- Задача 2: drop() vs .drop() ---
// Раскомментируй поочерёдно и объясни — почему одно работает, а другое нет? Такие усорвия языка.
fn task2() {
    let a = Named {
        name: "explicit".into(),
    };
    // a.drop();     // вариант 1
    drop(a); // вариант 2
    println!("task2 end");
}

// --- Задача 3: Drop в структуре ---
// Предскажи порядок: сначала сама структура или поля? Сначала поля
struct Wrapper {
    first: Named,
    second: Named,
}

impl Drop for Wrapper {
    fn drop(&mut self) {
        println!("  drop: Wrapper itself");
    }
}

fn task3() {
    let _w = Wrapper {
        first: Named {
            name: "first field".into(),
        },
        second: Named {
            name: "second field".into(),
        },
    };
}

// --- Задача 4: RAII паттерн ---
// Создай структуру GpioPin, которая:
// - При создании (new) печатает "GPIO {pin}: exported"
// - При дропе печатает "GPIO {pin}: unexported"
// Это RAII — ресурс захватывается в конструкторе, освобождается в деструкторе.
// На реальном RPi: new() пишет в /sys/class/gpio/export,
//                  drop() пишет в /sys/class/gpio/unexport.

// твой код тут...
struct GpioPin {
    pin: u32,
}

impl GpioPin {
    fn new(pin: u32) -> Self {
        println!("GPIO {pin}: exported");
        Self { pin }
    }
}

impl Drop for GpioPin {
    fn drop(&mut self) {
        println!("GPIO {}: unexported", self.pin);
    }
}

fn task4() {
    println!("--- создаём пины ---");
    let _pin1 = GpioPin::new(17);
    let _pin2 = GpioPin::new(27);
    println!("--- работаем с пинами ---");
    // ... тут мог бы быть код работы с GPIO
    println!("--- выходим из функции ---");
    // пины автоматически unexport при выходе
}

// --- Задача 5: Drop и Rc ---
// Предскажи, когда именно данные будут уничтожены.
// Подсказка: Rc дропает данные только когда strong_count == 0.
fn task5() {
    let data = Rc::new(Named {
        name: "shared data".into(),
    });
    println!("count: {}", Rc::strong_count(&data));

    let clone1 = Rc::clone(&data);
    println!("count: {}", Rc::strong_count(&data));

    {
        let clone2 = Rc::clone(&data);
        println!("count: {}", Rc::strong_count(&data));
        println!("--- inner scope end ---");
    } // clone2 дропается тут. Дропнутся ли данные? Нет, т.к. кол-во ссылок  еще не равно 0 и кто то пользуется.

    println!("count: {}", Rc::strong_count(&data));
    drop(clone1);
    println!("count after drop clone1: {}", Rc::strong_count(&data));
    println!("--- before final drop ---");
} // data дропается. strong_count == 0 → Named::drop вызывается

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
