// === ЗАДАНИЕ 3.4: Итераторы ===

// --- Задача 1: Базовые адаптеры ---
// Перепиши каждую функцию с for-цикла на цепочку итераторов.
// НЕ используй for, while, loop — только методы итераторов.
fn sum_of_squares_loop(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for n in numbers {
        sum += n * n;
    }
    sum
}

// Перепиши через .map().sum()
fn sum_of_squares(numbers: &[i32]) -> i32 {
    numbers.iter().map(|x| x * x).sum()
}

fn even_names_loop(names: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for (i, name) in names.iter().enumerate() {
        if i % 2 == 0 {
            result.push(name.to_uppercase());
        }
    }
    result
}

// Перепиши через .enumerate().filter().map().collect()
fn even_names(names: &[&str]) -> Vec<String> {
    names
        .iter()
        .enumerate()
        .filter(|name| name.0 % 2 == 0)
        .map(|name| name.1.to_uppercase())
        .collect()
}

// --- Задача 2: fold —  самый мощный адаптер ---
// fold = Aggregate в C#. Из него можно выразить почти всё.
// Найди самую длинную строку в слайсе через .fold()
// Если слайс пустой — верни ""
fn longest_word<'a>(words: &[&'a str]) -> &'a str {
    words.iter().fold(
        "",
        |acc, &word| {
            if word.len() > acc.len() { word } else { acc }
        },
    )
}

// --- Задача 3: Свой итератор ---
// Создай структуру `Countdown` которая считает от n до 0 (включительно).
// Реализуй для неё trait Iterator с Item = u32.
//
// let c = Countdown::new(3);
// c.next() → Some(3)
// c.next() → Some(2)
// c.next() → Some(1)
// c.next() → Some(0)
// c.next() → None

struct Countdown {
    // твои поля
    current: Option<u32>
}

impl Countdown {
    pub fn new(start: u32) -> Self {
        Self { current: Some(start) }
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.current?;
        self.current = val.checked_sub(1);
        Some(val)
    }
}

// --- Задача 4: Цепочки итераторов ---
// Дан вектор строк. Нужно:
// 1. Отфильтровать пустые строки
// 2. Обрезать пробелы по краям (trim)
// 3. Преобразовать в UPPERCASE
// 4. Собрать в одну строку через ", "
// Всё — одной цепочкой.
fn clean_and_join(input: &[&str]) -> String {
    input
        .iter()
        .map(|x| x.trim().to_uppercase())
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .join(", ")
}

// --- Задача 5: Iterator + Option ---
// Дан слайс строк, каждая может быть числом или нет.
// Спарси только валидные числа и верни их сумму.
// Подсказка: .filter_map() — это .filter() + .map() в одном.
// str::parse::<i32>() возвращает Result, а .ok() превращает его в Option.
fn sum_valid_numbers(input: &[&str]) -> i32 {
    input
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .sum()
}

fn main() {
    // Задача 1
    let nums = vec![1, 2, 3, 4, 5];
    println!("sum of squares (loop): {}", sum_of_squares_loop(&nums));
    println!("sum of squares (iter): {}", sum_of_squares(&nums)); // 55

    let names = vec!["Alice", "Bob", "Charlie", "Dave", "Eve"];
    println!("even names (loop): {:?}", even_names_loop(&names));
    println!("even names (iter): {:?}", even_names(&names)); // ["ALICE", "CHARLIE", "EVE"]

    // Задача 2
    let words = vec!["hi", "hello", "hey", "greetings"];
    println!("longest: {}", longest_word(&words)); // greetings

    let words = vec![];
    println!("longest: {}", longest_word(&words));

    // Задача 3
    let countdown = Countdown::new(5);
    let nums: Vec<u32> = countdown.collect();
    println!("countdown: {:?}", nums); // [5, 4, 3, 2, 1, 0]

    // Бонус: раз Countdown — итератор, все адаптеры работают бесплатно:
    let sum: u32 = Countdown::new(10).sum();
    println!("sum 0..=10: {}", sum); // 55

    // Задача 4
    let messy = vec!["  hello ", "", " world  ", "  ", "  rust "];
    println!("cleaned: {}", clean_and_join(&messy)); // "HELLO, WORLD, RUST"

    // Задача 5
    let mixed = vec!["42", "abc", "7", "", "13", "xyz", "0"];
    println!("sum of valid: {}", sum_valid_numbers(&mixed)); // 62
}
