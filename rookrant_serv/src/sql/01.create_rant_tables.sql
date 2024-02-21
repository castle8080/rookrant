drop table if exists rant_user;

create table rant_user (
    id varchar(32) not null primary key,
	name varchar(32) not null,
	email varchar(32) not null
);

create unique index i_rant_user_email on rant_user (email);

drop table if exists rant_entry;

create table rant_entry (
    id varchar(32) not null primary key,
	user_id varchar(32) not null,
	content text not null,
	created timestamp not null,
	modified timestamp not null
);

create index i_rant_entry_user_created on rant_entry (user_id, created);