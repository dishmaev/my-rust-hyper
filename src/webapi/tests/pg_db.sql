DROP SCHEMA IF EXISTS webapi CASCADE;
/
CREATE SCHEMA webapi AUTHORIZATION postgres;
/
SET search_path = webapi;
/
-- TABLES
CREATE TABLE car (
	id int4 NOT NULL GENERATED ALWAYS AS IDENTITY,
	car_name text NOT NULL,
	CONSTRAINT car_pk PRIMARY KEY (id)
);
/
CREATE UNIQUE INDEX car_car_name_idx ON car USING btree (car_name);
/
CREATE TABLE usr (
	id int4 NOT NULL GENERATED ALWAYS AS IDENTITY,
	usr_name text UNIQUE NOT NULL,
	usr_password text NOT NULL,
	CONSTRAINT usr_pk PRIMARY KEY (id),
	CONSTRAINT usr_usr_name_key UNIQUE (usr_name)
);
/
INSERT INTO usr
(usr_name, usr_password)
VALUES('user1', 'pass1');
/
INSERT INTO usr
(usr_name, usr_password)
VALUES('test', '1234567890');
/
CREATE TABLE client_access (
	source_service_name text NOT NULL,
	destination_service_name text NOT NULL,
	usr_name text NOT NULL,
	usr_password text NOT NULL,
	CONSTRAINT client_access_pk PRIMARY KEY (source_service_name, destination_service_name)
);
/
INSERT INTO client_access
(source_service_name, destination_service_name, usr_name, usr_password)
VALUES('*', '*', 'test', '1234567890');
/
CREATE TABLE sended_async_command (
	id text NOT NULL,
	object_type text NOT NULL,
	proto text NOT NULL,
	"state_to" text NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT sended_async_command_pk PRIMARY KEY (id)
);
/
CREATE INDEX sended_async_command_created_at_idx ON sended_async_command USING btree (created_at, object_type);
/
CREATE TABLE received_async_command (
	id text NOT NULL,
	object_type text NOT NULL,
	body text NOT NULL,
	proto text NOT NULL,
	"reply_to" text NOT NULL,
	"state" text NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT received_async_command_pk PRIMARY KEY (id)
);
/
CREATE INDEX received_async_command_created_at_idx ON received_async_command USING btree (created_at, object_type);
/
CREATE table "service" (
	"name" text NOT NULL,
	"description" text NOT NULL,
	"priority" int4 NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT service_pk PRIMARY KEY ("name")
);
/
CREATE table service_path (
	"service_name" text NOT NULL,
	proto text NOT NULL,
	helth text NOT NULL,
	"schema" text NOT NULL,
	"reply_to" text NOT NULL,
	"error" text NOT NULL,
	CONSTRAINT service_path_pk PRIMARY KEY ("service_name", proto),
	CONSTRAINT service_path_service_name_fk FOREIGN KEY ("service_name") REFERENCES webapi.service("name")
);
/
CREATE table service_command (
	"service_name" text NOT NULL,
	object_type text NOT NULL,
	"description" text NOT NULL,
	reply_type text NOT NULL,
	CONSTRAINT service_command_pk PRIMARY KEY ("service_name", object_type),
	CONSTRAINT service_command_service_name_fk FOREIGN KEY ("service_name") REFERENCES webapi.service("name")
);
/
CREATE table service_command_path (
	"service_name" text NOT NULL,
	object_type text NOT NULL,
	proto text NOT NULL,
	"to" text NOT NULL,
	CONSTRAINT service_command_path_pk PRIMARY KEY ("service_name", object_type, proto),
	CONSTRAINT service_command_path_sn_ot_fk FOREIGN KEY ("service_name", object_type) 
		REFERENCES webapi.service_command("service_name", object_type)
);
/
CREATE table service_event (
	"service_name" text NOT NULL,
	object_type text NOT NULL,
	"description" text NOT NULL,
	CONSTRAINT service_event_pk PRIMARY KEY ("service_name", object_type),
	CONSTRAINT service_event_service_name_fk FOREIGN KEY ("service_name") REFERENCES webapi.service("name")
);
/
CREATE table service_subscription (
	"service_name" text NOT NULL,
	object_type text NOT NULL,
	CONSTRAINT service_subscription_pk PRIMARY KEY ("service_name", object_type),
	CONSTRAINT service_subscription_service_name_fk FOREIGN KEY ("service_name") REFERENCES webapi.service("name")
);
/
CREATE table service_subscription_path (
	"service_name" text NOT NULL,
	object_type text NOT NULL,
	proto text NOT NULL,
	"to" text NOT NULL,
	CONSTRAINT service_subscription_path_pk PRIMARY KEY ("service_name", object_type, proto),
	CONSTRAINT service_subscription_path_sn_ot_fk FOREIGN KEY ("service_name", object_type) 
		REFERENCES webapi.service_subscription("service_name", object_type)
);
/
-- VIEWS
CREATE OR REPLACE VIEW v_sended_async_command
AS SELECT id,
	object_type,
	proto,
	"state_to",
	created_at
		FROM webapi.sended_async_command
			ORDER BY created_at, object_type;
/
CREATE OR REPLACE VIEW v_received_async_command
AS SELECT id,
	object_type,
	body,
	proto,
	"reply_to",
	"state",
	created_at,
	updated_at
		FROM webapi.received_async_command
			ORDER BY created_at, object_type;
/
CREATE OR REPLACE VIEW v_service
AS SELECT s."name", s."description", s.priority
	FROM webapi.service s
		ORDER BY s."name";
/
CREATE OR REPLACE VIEW v_service_command
AS SELECT c.service_name,
    s.priority,
    c.object_type,
	c.description,
	c.reply_type
   FROM webapi.service_command c
     JOIN webapi.service s ON s.name = c.service_name
	 	ORDER BY c.object_type, s.priority;
/
CREATE OR REPLACE VIEW v_service_path
AS SELECT p."service_name", p.proto, p.helth, p."schema", p."error", p.reply_to
   FROM webapi.service_path p
	 	ORDER BY p.proto;
/
CREATE OR REPLACE VIEW v_service_command_path
AS SELECT p."service_name", p.object_type, p.proto, p.to
   FROM webapi.service_command_path p
	 	ORDER BY p.proto;
/
CREATE OR REPLACE VIEW v_service_event
AS SELECT e.service_name,
    e.object_type,
	e.description
   FROM webapi.service_event e;
/
CREATE OR REPLACE VIEW v_service_subscription
AS SELECT ss.service_name,
    ss.object_type
   FROM webapi.service_subscription ss
     JOIN webapi.service sv ON sv.name = ss.service_name
	 	ORDER BY ss.object_type;
/
CREATE OR REPLACE VIEW v_service_subscription_path
AS SELECT p."service_name", p.object_type, p.proto, p.to
   FROM webapi.service_subscription_path p
	 	ORDER BY p.proto;
/
