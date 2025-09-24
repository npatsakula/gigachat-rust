# GigaChat Rust SDK

[![Crates.io](https://img.shields.io/crates/v/gigachat-rust)](https://crates.io/crates/gigachat-rust)
[![Documentation](https://docs.rs/gigachat-rust/badge.svg)](https://docs.rs/gigachat-rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Rust SDK для работы с GigaChat API от Сбера - доступ к русскоязычным LLM.

## Возможности

- **Поддержка Sber API**:
  - [x] Генерация текста.
    - [x] Синхронная.
    - [x] Потоковая.
  - [x] Генерация эмбеддингов.
  - [x] Детекция LLM-сгенерированного текста.
  - [x] Пакетная обработка.
  - [ ] Обработка медиа.
  - [ ] Поддержка функций.
- **Конфигурация клиента**:
  - [x] Автоматическое использование сертификатов мин.цифры.
  - [x] Возможности для конфигурации корпоративного прокси-сервера.
  - [x] OAuth авторизация с автоматической ротацией токенов.
- **Средства интроспекции**:
  - [x] Поддержка сквозной трассировки при помощи `tracing`.
  - [x] Поддержка перехвата HTTP запросов.

## Getting started

Добавьте в ваш `Cargo.toml`:

```toml
[dependencies]
gigachat-rust = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Запуск примеров

### Аутентификация

Для использования GigaChat API вам необходимо получить токен аутентификации от Сбера. Установите его как переменную окружения:

```bash
export GIGACHAT_TOKEN="ваш-токен"
```

## Примеры использования

Вместо встраивания примеров кода в README, мы предоставляем подробные рабочие примеры в директории [examples](./examples/):

| Пример | Описание | Файл |
|--------|----------|------|
| [Генерация текста](./examples/generate.rs) | Демонстрирует генерацию текста с помощью GigaChat | [generate.rs](./examples/generate.rs) |
| [Текстовые эмбеддинги](./examples/embeddings.rs) | Пример создания векторных представлений текста | [embeddings.rs](./examples/embeddings.rs) |

## Конфигурация

### Опции Client Builder

`GigaChatClientBuilder` позволяет настраивать клиент:

```rust
use gigachat_rust::client::GigaChatClientBuilder;
use reqwest::Proxy;

let client = GigaChatClientBuilder::new(token)
    // Настройка URL аутентификации при необходимости
    // .auth_url("https://ваш-auth-url.com")
    // Настройка базового URL при необходимости
    // .base_url("https://ваш-base-url.com")
    // Настройка прокси при необходимости
    // .proxy(Proxy::http("http://proxy:8080").unwrap())
    // Добавление заголовков по умолчанию
    // .default_header("X-Custom-Header", "custom-value")
    .build()
    .await?;
```

## Обработка ошибок

Все вызовы API возвращают тип `Result`, что делает обработку ошибок прямолинейной:

```rust
use anyhow::Result;
use gigachat_rust::{client::GigaChatClientBuilder, generation::{Model, structures::Message}};

async fn example() -> Result<()> {
    let token = std::env::var("GIGACHAT_TOKEN")?;
    let client = GigaChatClientBuilder::new(token).build().await?;

    let response = client
        .generate()
        .with_model(Model::GigaChat2Lite)
        .with_messages(vec![Message::user("Привет, мир!")])
        .execute()
        .await?;

    println!("Успех: {response:?}");

    Ok(())
}
```
