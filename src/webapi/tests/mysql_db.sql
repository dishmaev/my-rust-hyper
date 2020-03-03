CREATE TABLE `rust`.`car` (
  `id` int NOT NULL AUTO_INCREMENT,
  `car_name` varchar(100) NOT NULL,
  CONSTRAINT car_pk PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE UNIQUE INDEX  car_car_name_IDX USING BTREE ON rust.car (car_name);

CREATE TABLE `rust`.`error` (
  `id` int NOT NULL,
  `error_name` varchar(100) NOT NULL,
  CONSTRAINT error_pk PRIMARY KEY (id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE UNIQUE INDEX error_error_name_IDX USING BTREE ON rust.error (error_name);

INSERT INTO `rust`.`error`
(id, error_name)
VALUES(-1, 'Database error');

INSERT INTO `rust`.`error`
(id, error_name)
VALUES(-100, 'Some items with specified id is not found');

CREATE TABLE `rust`.`usr` (
  `id` int NOT NULL AUTO_INCREMENT,
  `usr_name` varchar(100) NOT NULL,
  `usr_password` varchar(100) NOT NULL,
  CONSTRAINT usr_pk PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE UNIQUE INDEX usr_usr_name_IDX USING BTREE ON rust.usr (usr_name);

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

CREATE TABLE `rust`.`subscription` (
  `id` int NOT NULL AUTO_INCREMENT,
  `object_name` varchar(100) NOT NULL,
  `event_name` varchar(100) NOT NULL,
  `call_back` varchar(100) NOT NULL,
  CONSTRAINT usr_pk PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE UNIQUE INDEX subs_obj_env_cb_idx USING BTREE ON rust.subscription (object_name, event_name, call_back);

