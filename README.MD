rustup override set nightly

sudo apt install libpq-dev

cargo install diesel_cli --no-default-features --features postgres

echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env

docker-compose -f stack.yml up

docker run --rm -P -p 127.0.0.1:5432:5432 -e POSTGRES_PASSWORD="1234" --name pg postgres:alpine

psql postgresql://postgres:1234@localhost:5432/postgres


diesel setup

sudo docker build . -t license_data_manager
sudo docker run -it license_data_manager -e RUST_BACKTRACE=1 \ 
-e RUST_LOG=debug \
-e ROCKET_ADDRESS=0.0.0.0 \
-e ROCKET_PORT=8000 \
-e DATABASE_URL=postgresql://postgres:1234@localhost:5432/license_db 

docker container inspect pg1
sudo docker run -e RUST_BACKTRACE=1 \
-e RUST_LOG=debug \
-e ROCKET_ADDRESS=0.0.0.0 \
-e ROCKET_PORT=8000 \
-e DATABASE_URL=postgresql://postgres:1234@172.17.0.2:5432/license_db \
-it license_data_manager 

docker-compose up -d


Reference:
https://github.com/genekuo/rust-microservices.git
