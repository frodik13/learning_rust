# Rust Learning Roadmap

> C# разработчик, полгода пишет на Rust (с AI). Цель — глубокое понимание + Linux/Raspberry Pi.

## ~~Фаза 1: Основы~~ (пропущена — уже знаешь)

## Фаза 2: Ownership — понять, а не угадать
- [x] **2.1** Ownership, move, Copy/Clone — что реально происходит в памяти
- [x] **2.2** Borrowing: почему `&mut` эксклюзивный, data races на этапе компиляции
- [x] **2.3** Lifetimes: когда компилятор не может вывести сам, elision rules
- [x] **2.4** Самостоятельная борьба с borrow checker (задачи без подсказок)

## Фаза 3: Система типов — мощь Rust
- [x] **3.1** Traits: impl vs dyn, object safety, orphan rule
- [x] **3.2** Generics + trait bounds + associated types (когда что использовать)
- [x] **3.3** Closures: Fn/FnMut/FnOnce, захват переменных, move closures
- [x] **3.4** Итераторы: ленивость, свой итератор, zero-cost абстракция
- [x] **3.5** Error handling: thiserror, From конверсии, error propagation паттерны
- [x] **3.6** Enums как state machines, newtype pattern, type-level programming

## Фаза 4: Память и unsafe
- [ ] **4.1** Smart pointers: Box, Rc, Arc, RefCell, interior mutability
- [ ] **4.2** Drop trait, деструкторы, порядок дропа
- [ ] **4.3** Unsafe: raw pointers, transmute, unsafe trait impl
- [ ] **4.4** FFI: вызов C из Rust и наоборот (критично для RPi — обёртки над sysfs, ioctl)
- [ ] **4.5** Repr, alignment, layout — контроль памяти как в C

## Фаза 5: Concurrency без GC
- [ ] **5.1** Send/Sync — почему Rust гарантирует thread safety на уровне типов
- [ ] **5.2** Channels, Mutex, RwLock, Atomics
- [ ] **5.3** async/await: Future, Pin, poll — как это работает внутри
- [ ] **5.4** Tokio — async runtime для Linux

## Фаза 6: Linux системное программирование на Rust
- [ ] **6.1** Работа с файлами, путями, std::fs, std::io
- [ ] **6.2** Процессы, сигналы, std::process, nix crate
- [ ] **6.3** Сетевое программирование: TCP/UDP, сокеты
- [ ] **6.4** GPIO, I2C, SPI на Raspberry Pi (rppal / linux-embedded-hal)
- [ ] **6.5** Сериализация: serde, JSON, бинарные форматы
- [ ] **6.6** CLI-приложение: clap, работа с конфигами
- [ ] **6.7** Демон/сервис на Rust: логирование, graceful shutdown, systemd

## Практические проекты
1. **CLI-утилита** (фазы 3-4) — парсинг, файлы, error handling
2. **Многопоточный сервер** (фаза 5) — каналы + shared state
3. **GPIO-монитор для RPi** (фаза 6) — чтение датчиков, запись логов
4. **HTTP API на RPi** (фазы 5-6) — axum/actix + реальная периферия
5. **Системный демон** (фаза 6) — сервис с graceful shutdown под systemd

## Подход
- Каждая тема = задание, где нужно САМОМУ написать код без AI
- Я объясняю концепцию, даю задачу, ты решаешь
- Если застрял — задаёшь вопрос, я подсказываю, но не даю ответ
