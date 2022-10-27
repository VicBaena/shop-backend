#!bin/sh

docker create --ip 10.0.0.2 -p 3306:3306 -e MYSQL_ALLOW_EMPTY_PASSWORD=yes --name rustDb mysql:8.0
docker container cp entrypoint.sql rustDb:/docker-entrypoint-initdb.d/
docker start rustDb