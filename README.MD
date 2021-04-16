rustup override set nightly

sudo apt install libpq-dev

cargo install diesel_cli --no-default-features --features postgres

echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env

docker-compose -f stack.yml up

docker run --rm -P -p 127.0.0.1:5432:5432 -e POSTGRES_PASSWORD="1234" --name pg postgres:alpine
docker run -d --network=host -P -p 0.0.0.0:5432:5432 -e POSTGRES_PASSWORD="1234" --name pg postgres:alpine

psql postgresql://postgres:1234@localhost:5432/postgres


diesel setup

sudo docker build . -t leo5th/slm:1.1
sudo docker build . -t license_data_manager
sudo docker run -it license_data_manager -e RUST_BACKTRACE=1 \ 
-e RUST_LOG=debug \
-e ROCKET_ADDRESS=0.0.0.0 \
-e ROCKET_PORT=8000 \
-e DATABASE_URL=postgresql://postgres:1234@localhost:5432/license_db 

docker container inspect pg1
sudo docker run -d -e RUST_BACKTRACE=1 \
-e RUST_LOG=debug \
-e ROCKET_ADDRESS=0.0.0.0 \
-e ROCKET_PORT=8100 \
-e DATABASE_URL=postgresql://postgres:1234@172.17.0.2:5432/license_db \
--name license_data_manager -p 0.0.0.0:8100:8100 -it leo5th/slm:1.1 

docker-compose up -d


Reference:
https://github.com/genekuo/rust-microservices.git


table! {
    customer (id) {id -> Int4,name -> Varchar,address -> Nullable<Varchar>,    }
}

table! {
    customer_license (id) {id -> Int4,customer_id -> Int4,
license_id -> Int4,license_code -> Varchar,active -> Bool,
start_date -> Timestamp,end_date -> Timestamp,cpu -> Int4,
storage -> Int4,number_of_nodes -> Int4,
    }
}

table! {
    license (id) {id -> Int4,name -> Varchar,duration -> Int4,cpu -> Int4,storage -> Int4,number_of_nodes -> Int4,
    }
}


curl -d '{"name": "customer name 1", "address": "Address 1"}' -H "Content-Type: application/json" -X POST http://localhost:8000/customers
curl -d '{"name": "customer name 2", "address": "Address 2"}' -H "Content-Type: application/json" -X POST http://localhost:8000/customers
curl http://localhost:8000/customers/1
curl -d '{"id":1, "name": "Name 1", "address": "new address"}' -H "Content-Type: application/json" -X PUT http://localhost:8000/customers/1
curl -X DELETE http://localhost:8000/customers/2
curl http://localhost:8000/customers

curl -d '{"name": "license 1", "duration": 3, "cpu": 16, "storage": 1024, "number_of_nodes": 100}' -H "Content-Type: application/json" -X POST http://localhost:8000/licenses
curl -d '{"name": "license 2", "duration": 5, "cpu": 8, "storage": 1024, "number_of_nodes": 10}' -H "Content-Type: application/json" -X POST http://localhost:8000/licenses
curl http://localhost:8000/licenses/1
curl -d '{"id": 2, "name": "license 2 update", "duration": 5, "cpu": 8, "storage": 1024, "number_of_nodes": 10}' -H "Content-Type: application/json" -X PUT http://localhost:8000/licenses/2
curl -X DELETE http://localhost:8000/licenses/2
curl http://localhost:8000/licenses


curl -d '{"customer_id": 1, "license_id": 1, "license_code": "AKWOFJALJWO", "active": true, "start_date": "2020-04-12T00:00:00", "end_date": "2020-10-12T00:00:00", "cpu": 1, "storage": 4, "number_of_nodes": 1}' -H "Content-Type: application/json" -X POST http://localhost:8000/customer_licenses
curl -d '{"customer_id": 3, "license_id": 1, "license_code": "ZLKJELAMSDKEW", "active": true, "start_date": "2020-04-12T00:00:00", "end_date": "2020-10-12T00:00:00", "cpu": 1, "storage": 4, "number_of_nodes": 1}' -H "Content-Type: application/json" -X POST http://localhost:8000/customer_licenses
curl http://localhost:8000/customer_licenses/1
curl -d '{"id": 4, "customer_id": 3, "license_id": 1, "license_code": "ZLKJELAMSDKEW", "active": true, "start_date": "2020-04-12T00:00:00", "end_date": "2020-10-12T00:00:00", "cpu": 2, "storage": 4, "number_of_nodes": 1}' -H "Content-Type: application/json" -X PUT http://localhost:8000/customer_licenses/4
curl -X DELETE http://localhost:8000/customer_licenses/4
curl http://localhost:8000/customer_licenses

curl http://localhost:8000/customer_licenses/customer/1
curl http://localhost:8000/customer_licenses/license/1
