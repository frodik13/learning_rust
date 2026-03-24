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
