// === ЗАДАНИЕ 3.3: Closures ===

// --- Задача 1: Определи трейт ---
// Для каждого замыкания определи: Fn, FnMut или FnOnce?
// Раскомментируй правильный вариант.

fn task1() {
    let name = String::from("Rust");
    let numbers = vec![1, 2, 3];
    let mut sum = 0;

    // Замыкание A: читает name
    let a = || println!("{}", name);
    // a реализует: Fn

    // Замыкание B: меняет sum
    let mut b = || {
        sum += 1;
    };
    // b реализует FnMut

    // Замыкание C: забирает numbers
    let c = || {
        drop(numbers);
    };
    // c реализует FnOnce

    a();
    a();
    b();
    b();
    c();
    // c(); // потому что переменная, которую захватило замыкание уже уничтожена.

    println!("sum = {}", sum);
}

// --- Задача 2: Closure как параметр ---
// Допиши функцию `apply_twice`: принимает значение и замыкание,
// применяет замыкание два раза.
// Какой trait bound нужен: Fn, FnMut, или FnOnce? Почему?
fn apply_twice<T, F>(mut value: T, f: F) -> T
where
    F: Fn(T) -> T, // исправь trait если нужно
{
    value = f(value);
    value = f(value);
    value
}

fn task2() {
    let result = apply_twice(1, |x| x * 2);
    println!("apply_twice(1, *2) = {}", result); // 4

    let result = apply_twice(String::from("ha"), |s| s + "ha");
    println!("apply_twice(ha, +ha) = {}", result); // hahaha
}

// --- Задача 3: move closure ---
// Эта функция должна вернуть замыкание, которое при вызове возвращает greeting.
// Без move не скомпилируется — почему? Не знаю почему, объясни.
// Исправь, добавив move.
fn make_greeter(name: String) -> impl Fn() -> String {
    let greeting = format!("Hello, {}!", name);
    move || greeting.clone()
}

fn task3() {
    let greet = make_greeter(String::from("Fedor"));
    println!("{}", greet());
    println!("{}", greet()); // должно работать дважды
}

// --- Задача 4: FnMut в реальности ---
// Напиши функцию `count_matches`: принимает слайс &[T] и предикат (замыкание),
// возвращает количество элементов, для которых предикат вернул true.
// Какой bound на замыкание? Подумай: предикат только читает элемент.

fn count_matches<T, F>(numbers: &[T], f: F) -> i32
where
    F: Fn(&T) -> bool,
{
    let mut count = 0;

    for num in numbers {
        if f(num) {
            count += 1;
        }
    }

    count
}

fn task4() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let evens = count_matches(&numbers, |x| x % 2 == 0);
    println!("evens: {}", evens); // 5

    let big = count_matches(&numbers, |x| *x > 5);
    println!("big: {}", big); // 5

    // С замыканием, которое захватывает переменную
    let threshold = 7;
    let above = count_matches(&numbers, |x| *x > threshold);
    println!("above {}: {}", threshold, above); // 3
}

// --- Задача 5: Возврат разных замыканий ---
// Эта функция возвращает замыкание-трансформер.
// Если uppercase == true, возвращает замыкание, переводящее строку в верхний регистр.
// Иначе — в нижний.
// Почему тут нужен Box<dyn Fn>? Почему нельзя impl Fn? Потому что у замыкания может быть разный размер, поэтому нужен Box
fn make_transformer(uppercase: bool) -> Box<dyn Fn(String) -> String> {
    if uppercase {
        Box::new(|x: String| x.to_uppercase())
    } else {
        Box::new(|x: String| x.to_lowercase())
    }
}

fn task5() {
    let upper = make_transformer(true);
    let lower = make_transformer(false);
    println!("{}", upper(String::from("hello"))); // HELLO
    println!("{}", lower(String::from("WORLD"))); // world
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
