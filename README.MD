rustup override set nightly

sudo apt install libpq-dev

cargo install diesel_cli --no-default-features --features postgres

echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env

docker-compose -f stack.yml up

docker run --rm -P -p 127.0.0.1:5432:5432 -e POSTGRES_PASSWORD="1234" --name pg postgres:alpine

psql postgresql://postgres:1234@localhost:5432/postgres


diesel setup

Reference:
https://github.com/genekuo/rust-microservices.git