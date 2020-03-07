#!/bin/sh
docker run -it --rm --name test-mysql -e MYSQL_ROOT_PASSWORD=password -e MYSQL_DATABASE=webapi -p 3306:3306 mysql