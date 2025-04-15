
show databases;

create database `todo-list`;

create table `todo-list`.users(
    id int not null auto_increment ,
    name varchar(100),
    primary key (id)
)engine InnoDB;

# drop table `todo-list`.users;

select * from `todo-list`.users;






