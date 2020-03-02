#!/bin/sh
docker run -it --rm --name test-pg -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres