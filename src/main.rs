// === ЗАДАНИЕ 4.1: Smart Pointers ===

use std::arch::naked_asm;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;

// --- Задача 1: Box и рекурсивные типы ---
// Создай бинарное дерево:
//   enum Tree<T> {
//       Leaf(T),
//       Node { left: ???, right: ??? },
//   }
// Без Box не скомпилируется — почему?
// Реализуй метод fn sum(&self) -> i32 для Tree<i32> (сумма всех значений).

// твой код тут...
enum Tree<T> {
    Leaf(T),
    Node {
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

impl Tree<i32> {
    fn sum(&self) -> i32 {
        let mut sum = 0;

        let mut queue: VecDeque<&Tree<i32>> = VecDeque::new();
        queue.push_back(self);
        while !queue.is_empty() {
            let node = queue.pop_back();
            match node {
                Some(n) => match n {
                    Tree::Leaf(value) => sum += value,
                    Tree::Node { left, right } => {
                        queue.push_back(left);
                        queue.push_back(right);
                    }
                },
                None => continue,
            }
        }

        sum
    }
}

fn task1() {
    let tree = Tree::Node {
        left: Box::new(Tree::Node {
            left: Box::new(Tree::Leaf(1)),
            right: Box::new(Tree::Leaf(2)),
        }),
        right: Box::new(Tree::Leaf(3)),
    };
    println!("sum = {}", tree.sum()); // 6
}

// --- Задача 2: Rc — несколько владельцев ---
// Сценарий: на RPi несколько задач читают из одного конфига.
// Config нельзя клонировать (дорого), но несколько частей кода
// должны иметь доступ к нему.
//
// struct Config { device_name: String, pin_count: u8 }
// struct GpioManager { config: ??? }
// struct Logger { config: ??? }
//
// Создай один Config, оберни в Rc, передай в оба.
// Убедись что оба читают одни и те же данные.
// Выведи Rc::strong_count на каждом шаге.

// твой код тут...
struct Config {
    device_name: String,
    pin_count: u8,
}

struct GpioManager {
    config: Rc<Config>,
}

struct Logger {
    config: Rc<Config>,
}

fn task2() {
    let config = Rc::new(Config {
        device_name: "RPi4".to_string(),
        pin_count: 40,
    });
    println!("count after create: {}", Rc::strong_count(&config));

    let gpio = GpioManager {
        config: Rc::clone(&config),
    };
    println!("count after gpio: {}", Rc::strong_count(&config));

    let logger = Logger {
        config: Rc::clone(&config),
    };
    println!("count after logger: {}", Rc::strong_count(&config));

    println!(
        "gpio sees: {} ({} pins)",
        gpio.config.device_name, gpio.config.pin_count
    );
    println!(
        "logger sees: {} ({} pins)",
        logger.config.device_name, logger.config.pin_count
    );

    drop(gpio);
    println!("count after drop gpio: {}", Rc::strong_count(&config));
}

// --- Задача 3: RefCell — interior mutability ---
// Сценарий: несколько модулей пишут в общий лог.
// Лог должен быть мутабельным, но Rc не даёт &mut.
// Решение: Rc<RefCell<Vec<String>>>
//
// Создай SharedLog = Rc<RefCell<Vec<String>>>
// Напиши функцию fn log_message(log: &SharedLog, msg: &str)
// Напиши функцию fn print_log(log: &SharedLog)

type SharedLog = Rc<RefCell<Vec<String>>>;

fn log_message(log: &SharedLog, msg: &str) {
    log.borrow_mut().push(msg.to_string());
}

fn print_log(log: &SharedLog) {
    let l = log.borrow();
    l.iter().for_each(|x| println!("{x}"));
}

fn task3() {
    let log = Rc::new(RefCell::new(Vec::new()));

    log_message(&log, "System started");
    log_message(&log, "GPIO initialized");

    let log2 = Rc::clone(&log);
    log_message(&log2, "Sensor reading: 25.5°C");

    print_log(&log); // все 3 сообщения
    println!("log refs: {}", Rc::strong_count(&log));
}

// --- Задача 4: Box<dyn Trait> — owned trait objects ---
// Создай trait Device с методом fn status(&self) -> String
// Создай два типа: Led { pin: u8, on: bool } и Buzzer { pin: u8, freq: u32 }
// Реализуй Device для обоих.
//
// Создай функцию fn create_devices() -> Vec<Box<dyn Device>>
// которая возвращает вектор разных устройств.
// Почему тут Box, а не &dyn? Потому что устройства создаются внутри функции
// и должны ЖИТЬ после возврата. Ссылка на локальную переменную — dangling.

// твой код тут...
trait Device {
    fn status(&self) -> String;
}

struct Led {
    pin: u8,
    on: bool,
}

struct Buzzer {
    pin: u8,
    freq: u32,
}

impl Device for Led {
    fn status(&self) -> String {
        format!("Led pin: {} status {}", self.pin, self.on)
    }
}

impl Device for Buzzer {
    fn status(&self) -> String {
        format!("Buzzer pin: {}, freq: {}", self.pin, self.freq)
    }
}

fn create_devices() -> Vec<Box<dyn Device>> {
    let mut result: Vec<Box<dyn Device>> = Vec::new();

    let led = Box::new(Led { pin: 8, on: false });
    let buzzer = Box::new(Buzzer { pin: 13, freq: 33 });

    result.push(led);
    result.push(buzzer);

    result
}

fn task4() {
    let devices = create_devices();
    for dev in &devices {
        println!("{}", dev.status());
    }
}

// --- Задача 5: Когда RefCell паникует ---
// Этот код скомпилируется, но УПАДЁТ в рантайме. Почему?
// Исправь, чтобы не паниковал.
fn task5() {
    let data = RefCell::new(vec![1, 2, 3]);

    let r1 = data.borrow();
    println!("r1: {:?}", r1);
    drop(r1);

    let mut r2 = data.borrow_mut(); // паника?
    r2.push(4);
    println!("r2: {:?}", r2);
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
