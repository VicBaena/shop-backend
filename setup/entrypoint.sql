create schema shopdb;
use shopdb;
create table if not exists items (id int, quantity int, product_name  varchar(50));
create table if not exists payments (id int not null auto_increment,ticket int ,product_id int, quantity int, product_name  varchar(50), primary key (id));

insert into items values (01,1000,"Pokeballs");
insert into items values (02,1000,"Superballs");
insert into items values (03,1000,"Ultraballs");
insert into items values (04,1000,"Honorballs");
insert into items values (05,1000,"Potions");
insert into items values (06,1000,"Super Potions");
insert into items values (07,1000,"Hiper Potions");
insert into items values (08,1000,"Max Potions");
insert into items values (09,1000,"Revive");
insert into items values (10,1000,"Max Revive");
insert into payments (ticket,product_id, quantity, product_name) values (-1,-1,-1,"Test entry");