# Используем образ с Rust
FROM rust:latest

# Копируем исходный код
WORKDIR /usr/src/oosz
COPY . .

# Собираем приложение
RUN cargo build --release

# Создаем образ для запуска
FROM debian:buster-slim

# Устанавливаем зависимости PostgreSQL
RUN apt-get update && apt-get install -y libpq-dev

# Копируем бинарный файл из предыдущего этапа
COPY --from=builder /usr/src/oosz/target/release/oosz /usr/local/bin/oosz

EXPOSE 8000
CMD ["oosz"]