## PostgreSQL

### Docker

```
docker run -d --name postgres \
    -v postgres_data:/var/lib/postgresql16/data \
    -e POSTGRES_USER=postgres \
    -e POSTGRES_PASSWORD=postgres \
    -p 5432:5432 \
    postgres:latest
```

## pgAdmin

```
docker run -d --name pgAdmin \
    -v pgadmin_data:/var/lib/pgadmin \
    -e 'PGADMIN_DEFAULT_EMAIL=user@user.com' \
    -e 'PGADMIN_DEFAULT_PASSWORD=admin' \
    --add-host=host.docker.internal:host-gateway \
    -p 54321:80 \
    dpage/pgadmin4
```

### Open pgAdmin

```
127.0.0.1:54321
user: user@user.com
password: admin
```

### Add database

```
host: localhost
port: 5432
database: postgres
user: postgres
password: postgres
```

### URL

```
postgresql://postgres:postgres@localhost:5432/postgres
```