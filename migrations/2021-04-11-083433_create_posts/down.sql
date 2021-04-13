-- This file should undo anything in `up.sql`

alter table customer_license 
drop CONSTRAINT fk_license;
drop table customer_license;
drop table license;