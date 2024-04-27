# Postgres Container

uuid_v7 is not supported out of the box

1. Download the latest [release](https://github.com/fboulnois/pg_uuidv7/releases).
2. Copy pg_uuidv7.so for your Postgres version into the `postgres_dev` docker compose folder.
3. Copy pg_uuidv7--1.5.sql and pg_uuidv7.control into the `postgres_dev` docker compose folder.
4. The docker-compose.yaml will mount the files into the correct folders
5. Set up database `cargo run init_db`


#### Reference

* [pg_uuidv7: Use the new v7 UUIDs in Postgres](https://github.com/fboulnois/pg_uuidv7)