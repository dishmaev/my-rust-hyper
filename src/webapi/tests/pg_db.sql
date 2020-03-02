CREATE SCHEMA rust AUTHORIZATION postgres;

CREATE TABLE rust.car (
	id int4 NOT NULL GENERATED ALWAYS AS IDENTITY,
	car_name text NOT NULL,
	CONSTRAINT car_pk PRIMARY KEY (id)
);

CREATE UNIQUE INDEX car_car_name_idx ON rust.car USING btree (car_name);

CREATE TABLE rust.error (
	id int4 NOT NULL,
	error_name text NOT NULL,
	CONSTRAINT error_pk PRIMARY KEY (id)
);

CREATE UNIQUE INDEX error_error_name_idx ON rust.error USING btree (error_name);

INSERT INTO rust.error
(id, error_name)
VALUES(-1, 'Database error');

INSERT INTO rust.error
(id, error_name)
VALUES(-100, 'Some items with specified id is not found');

CREATE TABLE rust.usr (
	id int4 NOT NULL GENERATED ALWAYS AS IDENTITY,
	usr_name text NOT NULL,
	usr_password text NOT NULL,
	CONSTRAINT usr_pk PRIMARY KEY (id)
);

CREATE UNIQUE INDEX usr_usr_name_idx ON rust.usr USING btree (usr_name);

INSERT INTO rust.usr
(usr_name, usr_password)
VALUES('user1', 'pass1');

INSERT INTO rust.usr
(usr_name, usr_password)
VALUES('user2', 'pass2');

INSERT INTO rust.usr
(usr_name, usr_password)
VALUES('user3', 'pass3');

INSERT INTO rust.usr
(usr_name, usr_password)
VALUES('test', '1234567890');

CREATE TABLE rust.subscription (
	id int4 NOT NULL GENERATED ALWAYS AS IDENTITY,
	object_name text NOT NULL,
	event_name text NOT NULL,
	call_back text NOT NULL
);

CREATE UNIQUE INDEX subs_obj_env_cb_idx ON rust.subscription USING btree (object_name, event_name, call_back);
