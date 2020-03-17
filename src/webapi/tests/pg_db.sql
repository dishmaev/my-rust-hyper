CREATE SCHEMA webapi AUTHORIZATION postgres;
/
SET search_path = webapi;
/
CREATE TABLE car (
	id int4 NOT NULL GENERATED ALWAYS AS IDENTITY,
	car_name text NOT NULL,
	CONSTRAINT car_pk PRIMARY KEY (id)
);
/
CREATE UNIQUE INDEX car_car_name_idx ON car USING btree (car_name);

CREATE TABLE error (
	id int4 NOT NULL,
	error_name text NOT NULL,
	CONSTRAINT error_pk PRIMARY KEY (id)
);
/
CREATE UNIQUE INDEX error_error_name_idx ON error USING btree (error_name);
/
INSERT INTO error
(id, error_name)
VALUES(-1, 'Database error');
/
INSERT INTO error
(id, error_name)
VALUES(-100, 'Some items with specified id is not found');
/
CREATE TABLE usr (
	id int4 NOT NULL GENERATED ALWAYS AS IDENTITY,
	usr_name text NOT NULL,
	usr_password text NOT NULL,
	CONSTRAINT usr_pk PRIMARY KEY (id)
);
/
CREATE UNIQUE INDEX usr_usr_name_idx ON usr USING btree (usr_name);
/
INSERT INTO usr
(usr_name, usr_password)
VALUES('user1', 'pass1');
/
INSERT INTO usr
(usr_name, usr_password)
VALUES('test', '1234567890');
/
CREATE table service (
	"name" text NOT NULL,
	"priority" int4 NOT NULL,
	http_helth text NOT NULL,
	mq_helth text NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, 
	CONSTRAINT service_pk PRIMARY KEY ("name")
);
/
CREATE table command (
	service_name text NOT NULL,
	object_type text NOT NULL,
	http_to text NULL,
	mq_to text NULL,
	CONSTRAINT command_pk PRIMARY KEY (service_name, object_type),
	CONSTRAINT command_service_name_fk FOREIGN KEY (service_name) REFERENCES webapi.service("name")
);
/
CREATE table subscription (
	service_name text NOT NULL,
	object_type text NOT NULL,
	http_to text NULL,
	mq_to text NULL,
	CONSTRAINT subscription_pk PRIMARY KEY (service_name, object_type),
	CONSTRAINT subscription_service_name_fk FOREIGN KEY (service_name) REFERENCES webapi.service("name")
);
/
CREATE OR REPLACE VIEW v_command
AS SELECT c.service_name,
    s.priority,
    c.object_type,
    c.http_to,
    c.mq_to
   FROM webapi.command c
     JOIN webapi.service s ON s.name = c.service_name;
/
CREATE OR REPLACE VIEW v_subscription
AS SELECT ss.service_name,
    sv.priority,
    ss.object_type,
    ss.http_to,
    ss.mq_to
   FROM webapi.subscription ss
     JOIN webapi.service sv ON sv.name = ss.service_name;
/
