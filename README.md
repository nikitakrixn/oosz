```shell
docker run --name postgres-oosz -e POSTGRES_PASSWORD=mysecretpassword -d postgres
docker build -t rust-web .
docker run --name my-rust-app --link postgres-oosz:postgres -d rust-web
sqlx migrate run
sqlx migrate revert
cargo watch -c -x run -i /static
```
