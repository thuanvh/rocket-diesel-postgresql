-- Your SQL goes here
create table customer(
    id INT GENERATED ALWAYS AS IDENTITY primary key,
    name varchar not null,
    address varchar
);

create table license (
    id INT GENERATED ALWAYS AS IDENTITY primary key, 
    name varchar not null,
    duration integer not null,
    cpu integer not null,
    storage integer not null,
    number_of_nodes integer not null);

create table customer_license (
    id INT GENERATED ALWAYS AS IDENTITY primary key, 
    customer_id int not null,
    license_id INT not null,
    license_code varchar not null,
    active boolean NOT NULL,
    start_date timestamp not null,
    end_date timestamp not null,
    cpu integer not null,
    storage integer not null,
    number_of_nodes integer not null,
    CONSTRAINT fk_license
      FOREIGN KEY(license_id) 
	  REFERENCES license(id),
    CONSTRAINT fk_customer
      FOREIGN KEY(customer_id) 
	  REFERENCES customer(id));