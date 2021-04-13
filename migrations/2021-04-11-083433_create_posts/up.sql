-- Your SQL goes here
create table license (
    id INT GENERATED ALWAYS AS IDENTITY primary key, 
    name varchar not null,
    duration integer not null,
    number_of_nodes integer not null);

create table customer_license (
    id INT GENERATED ALWAYS AS IDENTITY primary key, 
    customer_name varchar not null,
    address varchar,
    license_id INT not null,
    license_code varchar not null,
    start_date timestamp not null,
    end_date timestamp not null,
    number_of_nodes integer not null,
    CONSTRAINT fk_license
      FOREIGN KEY(license_id) 
	  REFERENCES license(id));