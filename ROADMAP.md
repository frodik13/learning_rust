# Rust Learning Roadmap

> C# разработчик, полгода пишет на Rust (с AI). Цель — глубокое понимание + embedded.

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
- [ ] **3.4** Итераторы: ленивость, свой итератор, zero-cost абстракция
- [ ] **3.5** Error handling: thiserror, From конверсии, error propagation паттерны
- [ ] **3.6** Enums как state machines, newtype pattern, type-level programming

## Фаза 4: Память и unsafe
- [ ] **4.1** Smart pointers: Box, Rc, Arc, RefCell, interior mutability
- [ ] **4.2** Drop trait, деструкторы, порядок дропа
- [ ] **4.3** Unsafe: raw pointers, transmute, unsafe trait impl
- [ ] **4.4** FFI: вызов C из Rust и наоборот (критично для embedded)
- [ ] **4.5** Repr, alignment, layout — контроль памяти как в C

## Фаза 5: Concurrency без GC
- [ ] **5.1** Send/Sync — почему Rust гарантирует thread safety на уровне типов
- [ ] **5.2** Channels, Mutex, RwLock, Atomics
- [ ] **5.3** async/await: Future, Pin, poll — как это работает внутри
- [ ] **5.4** Async без рантайма (для embedded)

## Фаза 6: Embedded Rust
- [ ] **6.1** no_std + no_alloc: что остаётся от языка
- [ ] **6.2** Volatile, MMIO, регистры через PAC
- [ ] **6.3** embedded-hal трейты, драйверы переиспользуемые между платформами
- [ ] **6.4** Линкер скрипты, memory.x, startup code
- [ ] **6.5** RTIC / Embassy — cooperative vs preemptive
- [ ] **6.6** Реальный проект: прошивка с периферией

## Подход
- Каждая тема = задание, где нужно САМОМУ написать код без AI
- Я объясняю концепцию, даю задачу, ты решаешь
- Если застрял — задаёшь вопрос, я подсказываю, но не даю ответ
