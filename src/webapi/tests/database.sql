-- Drop table

-- DROP TABLE public.car;

CREATE TABLE public.car (
	id int4 NOT NULL GENERATED ALWAYS AS IDENTITY,
	car_name text NOT NULL,
	CONSTRAINT car_pk PRIMARY KEY (id)
);
CREATE UNIQUE INDEX car_car_name_idx ON public.car USING btree (car_name);