# Заметки по изучению Rust

## Фаза 2: Ownership, Borrowing, Lifetimes

### 2.1 Ownership и Move

**Главное правило:** у каждого значения один владелец. Когда владелец уходит из скоупа — значение дропается.

**Move vs Copy:**
- `String`, `Vec`, `Box` — при `let b = a` происходит **move**, `a` невалидна
- `i32`, `f64`, `bool`, `char`, `&T` — реализуют `Copy`, при `let b = a` значение **копируется**, оба живут

```rust
let a = String::from("hello");
let b = a;          // move — a мертва
// println!("{}", a); // ОШИБКА

let x: i32 = 5;
let y = x;          // copy — оба живы
println!("{} {}", x, y); // OK
```

**Что в памяти:**
```
// i32: всё на стеке
x = [05 00 00 00]     // 4 байта
y = [05 00 00 00]     // независимая копия

// String: стек + куча
a = [ptr | len: 5 | cap: 5]  --> куча: "hello"
     после move a невалидна
b = [ptr | len: 5 | cap: 5]  --> та же куча
```

**Передача в функцию = move (или copy для Copy-типов):**
```rust
fn take(s: String) { }       // забирает владение
fn look(s: &String) { }      // только смотрит (borrow)
fn change(s: &mut String) { } // меняет через ссылку
```

**4 комбинации ownership + mutability:**
```rust
fn f(s: String)         // владение, нельзя менять s
fn f(mut s: String)     // владение + можно менять (для вызывающего разницы нет)
fn f(s: &String)        // shared borrow, только чтение
fn f(s: &mut String)    // exclusive borrow, можно менять оригинал
```

### 2.2 Borrowing и NLL

**Правило заимствования — в один момент можно иметь:**
- ЛИБО сколько угодно `&T` (shared borrow)
- ЛИБО один `&mut T` (exclusive borrow)
- Нельзя `&T` + `&mut T` одновременно

**NLL (Non-Lexical Lifetimes):** ссылка живёт не до конца `{}`, а до **последнего использования**.

```rust
let mut s = String::from("hello");
let r1 = &s;
println!("{}", r1);   // последнее использование r1 → r1 мертва
let r2 = &mut s;      // OK — shared borrow уже закончился
r2.push_str("!");
```

**Почему `Vec::push` конфликтует с `&v[0]`:**
`push()` может вызвать реаллокацию — новый буфер в куче, старый освобождается. Ссылка на элемент стала бы dangling pointer. Borrow checker запрещает это на этапе компиляции.

### 2.3 Lifetimes

**Зачем:** когда функция принимает ссылки и возвращает ссылку, компилятор должен знать — возвращённая ссылка привязана к какому аргументу?

```rust
// Не скомпилируется — непонятно, результат живёт как s1 или s2
fn longest(s1: &str, s2: &str) -> &str { ... }

// Скомпилируется — результат живёт не дольше обоих аргументов
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str { ... }
```

**`'a` — это ограничение**, не конкретное время. Компилятор берёт пересечение (меньшее из двух).

**Elision rules (когда lifetime не нужен вручную):**
1. Один входной `&` параметр → выход получает его lifetime
2. Метод с `&self` → выход привязан к `self`
3. Несколько входов, но один `&self` → выход привязан к `self`

**Несколько lifetimes — привязывай выход только к тому входу, от которого он зависит:**
```rust
fn with_prefix<'a, 'b>(prefix: &'a str, text: &'b str) -> &'b str {
    text  // результат зависит от text, не от prefix
}
```

**Lifetime в структуре:**
```rust
struct Excerpt<'a> {
    text: &'a str,  // структура не может жить дольше, чем данные, на которые ссылается
}
```

---

## Фаза 3: Traits

### 3.1 Traits — основы

**Trait = интерфейс + дефолтные реализации:**
```rust
trait Sensor {
    fn read_value(&self) -> f64;           // обязательный
    fn unit(&self) -> &str { "unknown" }   // дефолтный
}

impl Sensor for Thermometer {
    fn read_value(&self) -> f64 { self.temperature }
    fn unit(&self) -> &str { "°C" }  // переопределяем дефолт
}

impl Sensor for Accelerometer {
    fn read_value(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    // unit() — дефолтный "unknown"
}
```

**Derive — автогенерация стандартных трейтов:**
```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: f64, y: f64 }
// Debug   → println!("{:?}", p)
// Clone   → p.clone()
// PartialEq → p1 == p2 (не Eq, потому что f64 имеет NaN != NaN)
```

**Display — аналог ToString() в C#:**
```rust
impl fmt::Display for Thermometer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.temperature)  // write!, не println!
    }
}
```

### Q&A: `impl Trait` vs `dyn Trait`

**`&impl Trait` — статический dispatch (мономорфизация):**
- Компилятор генерирует отдельную копию функции для каждого типа
- Вызовы инлайнятся, нет overhead
- Бинарник чуть больше
- Нельзя хранить разные типы в одной коллекции

**`&dyn Trait` — динамический dispatch (vtable):**
- Одна копия функции, вызовы через указатель на таблицу методов
- Можно хранить разные типы вместе (гетерогенная коллекция)

**Гетерогенность** — коллекция из разных типов за одним интерфейсом:
```rust
// Гомогенная — все одного типа
let therms: Vec<Thermometer> = vec![t1, t2, t3];

// Гетерогенная — разные типы, но все реализуют Sensor
let sensors: Vec<Box<dyn Sensor>> = vec![
    Box::new(Thermometer { temperature: 25.0 }),
    Box::new(Accelerometer { x: 1.0, y: 2.0, z: 3.0 }),
];
```

**Когда что использовать:**

| Ситуация | Выбор |
|---|---|
| Обычная функция | `impl Trait` |
| Коллекция разных типов | `dyn Trait` |
| Embedded, критичен каждый такт | `impl Trait` |
| Уменьшить размер бинарника | `dyn Trait` |
| Трейт не object-safe | только `impl Trait` |

Для embedded — почти всегда `impl Trait` (нет indirect calls, не нужен `Box`/куча).

### 3.2 Generics, Trait Bounds, Associated Types

**Generic функция с trait bound:**
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T { ... }
```

**Несколько bounds — два синтаксиса:**
```rust
fn f<T: PartialOrd + Display>(x: &T) { ... }
fn f<T>(x: &T) where T: PartialOrd + Display { ... }  // where — читабельнее
```

**Generic struct — bound на impl, не на struct:**
```rust
struct Pair<T> { first: T, second: T }  // хранит любой T

impl<T: PartialOrd> Pair<T> {          // метод only для сравнимых T
    fn larger(&self) -> &T { ... }
}
```

**Associated type vs Generic в trait:**
```rust
// Generic — можно реализовать НЕСКОЛЬКО раз
trait Convert<T> { fn convert(&self) -> T; }
impl Convert<f64> for X { ... }
impl Convert<String> for X { ... }  // оба OK

// Associated type — реализуешь ОДИН раз, тип фиксирован
trait Register {
    type Value;
    fn read(&self) -> Self::Value;
}
impl Register for Reg8 { type Value = u8; ... }
```

Правило: тип логически один для реализации → associated type. Может быть несколько → generic.

**Bound на associated type:**
```rust
fn print_register<R: Register>(reg: &R)
where
    R::Value: Display,  // "Value должен быть печатаемым"
{
    println!("0x{:08X} = {}", reg.address(), reg.read());
}
```

**dyn Trait с associated type — нужно фиксировать Value:**
```rust
// dyn Register          — ОШИБКА: компилятор не знает размер Value
// dyn Register<Value = u8>  — OK: Value зафиксирован, vtable можно построить
let regs: Vec<&dyn Register<Value = u8>> = vec![&r1, &r2];
```

### 3.3 Closures: Fn, FnMut, FnOnce

**Три трейта замыканий (от слабого к сильному):**
- `Fn` — захватывает по `&T` (только читает). Можно вызывать сколько угодно раз.
- `FnMut` — захватывает по `&mut T` (может менять). Можно вызывать много раз.
- `FnOnce` — захватывает по `T` (забирает владение). Можно вызвать один раз.

Иерархия: `Fn` ⊂ `FnMut` ⊂ `FnOnce`. Кто реализует `Fn` — автоматически реализует остальные.

Компилятор **сам выбирает** минимально необходимый трейт.

**`move` closure — принудительное перемещение:**
```rust
fn make_greeter(name: String) -> impl Fn() -> String {
    let greeting = format!("Hello, {}!", name);
    move || greeting.clone()  // move забирает greeting внутрь замыкания
}
```
Без `move` замыкание захватывает `greeting` по ссылке, но `greeting` — локальная переменная,
которая умрёт при выходе из функции → dangling reference. `move` переносит владение внутрь замыкания.

Критично для: возврата замыканий из функций, передачи в потоки, embedded callbacks.

**Closure как параметр:**
```rust
fn count_matches<T, F>(items: &[T], predicate: F) -> i32
where F: Fn(&T) -> bool  // предикат только читает элемент
```

**Возврат разных замыканий — `Box<dyn Fn>`:**
```rust
fn make_transformer(upper: bool) -> Box<dyn Fn(String) -> String> {
    if upper {
        Box::new(|s| s.to_uppercase())
    } else {
        Box::new(|s| s.to_lowercase())
    }
}
```
`impl Fn` = один конкретный анонимный тип. Две ветки if/else — два РАЗНЫХ типа.
`Box<dyn Fn>` стирает конкретный тип через vtable.

### 3.4 Итераторы

**Трейт Iterator — один обязательный метод:**
```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

**Итераторы ленивые** — пока не вызовешь `.collect()`, `.sum()`, `.for_each()` — ничего не вычисляется.

**Соответствие C# LINQ → Rust:**

| C# LINQ | Rust |
|---|---|
| `.Select()` | `.map()` |
| `.Where()` | `.filter()` |
| `.Aggregate()` | `.fold()` |
| `.First()` | `.next()` |
| `.ToList()` | `.collect::<Vec<_>>()` |
| `.Any()` | `.any()` |
| `.Sum()` | `.sum()` |

**Основные адаптеры:**
```rust
// map + sum
numbers.iter().map(|x| x * x).sum()

// enumerate + filter + map + collect
names.iter()
    .enumerate()
    .filter(|(i, _)| i % 2 == 0)
    .map(|(_, name)| name.to_uppercase())
    .collect::<Vec<_>>()

// fold — самый мощный, как Aggregate в C#
words.iter().fold("", |acc, &word| {
    if word.len() > acc.len() { word } else { acc }
})

// filter_map — filter + map в одном (удобно для parse)
input.iter().filter_map(|s| s.parse::<i32>().ok()).sum()

// join через collect + join
items.iter().collect::<Vec<_>>().join(", ")
```

**Свой итератор — хранить состояние, не данные:**
```rust
struct Countdown { current: Option<u32> }

impl Countdown {
    fn new(start: u32) -> Self { Self { current: Some(start) } }
}

impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let val = self.current?;           // None → return None
        self.current = val.checked_sub(1); // 0-1 = None, не паника
        Some(val)
    }
}
```
Итератор = O(1) по памяти. Не создавай Vec заранее — вычисляй по запросу.
Реализовав Iterator, бесплатно получаешь `.map()`, `.filter()`, `.sum()`, `.collect()` и все остальные адаптеры.

### 3.5 Error Handling

**В Rust нет исключений.** Вместо `try/catch` — `Result<T, E>` и оператор `?`.

**Оператор `?`** — пробрасывает ошибку наверх (как throw, но на этапе компиляции):
```rust
fn read_config(path: &str) -> Result<String, io::Error> {
    let content = std::fs::read_to_string(path)?;  // Err → сразу return Err
    Ok(content.trim().to_string())
}
```

**`split_once` + `ok_or` — идиоматичный парсинг:**
```rust
let (key, val) = input.split_once('=').ok_or("нет '='")?;  // Option → Result
let num = val.parse::<i32>().map_err(|e| e.to_string())?;  // конвертация ошибки
```

**Свой тип ошибки:**
```rust
#[derive(Debug)]
enum ConfigError {
    MissingEquals,
    InvalidValue(ParseIntError),
}

impl fmt::Display for ConfigError { ... }
impl std::error::Error for ConfigError {}

// From позволяет ? автоматически конвертировать ParseIntError → ConfigError
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self { ConfigError::InvalidValue(e) }
}
```
После `impl From` можно писать `let num = val.parse::<i32>()?;` — `?` вызовет `.into()` автоматически.

**Match на ошибке — разная обработка разных случаев:**
```rust
match parse_key_value_v2(input) {
    Ok(r) => Ok(r),
    Err(ConfigError::MissingEquals) => Ok((input.to_string(), 0)),  // recover
    Err(e) => Err(e),  // propagate
}
```

**Collect в Result — мощный паттерн:**
```rust
// Vec<Result<T,E>> → Result<Vec<T>, E>. Первая ошибка останавливает сбор.
input.iter().map(|x| x.parse::<i32>()).collect::<Result<Vec<_>, _>>()
```

### 3.6 Enums как state machines, Newtype pattern

**Enum в Rust — алгебраический тип (каждый вариант хранит свои данные):**
```rust
enum Command {
    ReadGpio { pin: u8 },
    WriteGpio { pin: u8, value: bool },
    SetPwm { pin: u8, duty: f64 },
    Shutdown,
}
```
В C# enum — просто числа. В Rust — полноценные tagged unions.

**State machine через enum — невалидные состояния невозможны:**
```rust
enum LedState {
    Off,
    On { brightness: u8 },
    Blinking { interval_ms: u64 },
}

impl LedState {
    fn turn_on(self, brightness: u8) -> LedState {  // self, не &self!
        LedState::On { brightness }
    }
}
```
`self` (не `&self`) потребляет старое состояние. После `led.turn_on(128)` старый `led` недоступен — компилятор гарантирует корректность переходов.

**Newtype — type safety через обёртку:**
```rust
struct GpioPin(u8);
struct I2cAddress(u8);

fn read_gpio(pin: &GpioPin) -> bool { ... }
fn read_i2c(addr: &I2cAddress) -> u8 { ... }
// read_gpio(addr) — ошибка компиляции! Нельзя перепутать.
```
Zero-cost: в рантайме newtype — тот же u8, обёртка стирается компилятором.
