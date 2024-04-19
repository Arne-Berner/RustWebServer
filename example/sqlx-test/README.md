# SQLx
## Docker
```bash
docker pull postgres
docker run --name <process name> -e POSTGRES_PASSWORD=mypassword -d postgres
docker run -d -e "PGADMIN_DEFAULT_EMAIL=admin@admin.com" -e "PGADMIN_DEFAULT_PASSWORD=password" -p 5050:80 dpage/pgadmin4
docker inspect <postgres container id>
```
This will start the postgres and pgadmin in different container. That way, you will need to use the inspect dialog to get the ipaddress, when adding a new server in pgadmin. 
To check out the postgres database (management system), you can use docker compose to start pgadmin with postgres together.
```bash
# docker compose -f docker-compose.yaml up
# is the same as the following line
docker-compose up
```
Use the email+password to log in into pgadmin (on port 5050 or whatever you've chosen). This time you can use the container name instead of the ip, because they are in the same network. Name it whatever you want. You can also use your credentials from your docker-compose.yaml in the connection window.

## Migrations
SQLx has it's own migration system. It uses the DATABASE_URL env variable to connect to the db. This is also used to check the sql queries at compile time.
The DATABASE_URL can be exported by hand, but should be exported in some more reliable way. The ip can be seen by running "docker inspect <container id>" and then looking for the ipaddress.
```bash
export DATABASE_URL=postgres://postgres:password@172.17.0.2:5432/newsletter
```


To create a db you simply:
```bash
sqlx database create
sqlx migrate run
```
Whereas the last statement will run all the migrations in the "migrate" folder. To create tables you can simply write
```bash
sqlx migrate add <migration_name>
```
If it is not there yet, this command will creaete a migrations folder like before and add an empty migration with that name in there (ending with an .sql). A possible migration for creating a table looks like this:
```sql
CREATE TABLE subscriptions(
id uuid NOT NULL,
PRIMARY KEY (id),
email TEXT NOT NULL UNIQUE,
name TEXT NOT NULL,
subscribed_at timestamptz NOT NULL
);
```
There seems to be no way, to "refresh" the migrations done. The migrations that you have run are saved in a table in the db.

## Constraints
"Database constraints are useful as a last line of defence from application bugs but they come at a cost - the
database has to ensure all checks pass before writing new data into the table. Therefore constraints impact
our write-throughput, i.e. the number of rows we can INSERT/UPDATE per unit of time in a table" -zero2production
For example using a unique key, will ad another b-tree column, which needs to be updated on every insert/update/delete. This is not very important for such a small size, but it could be important for bigger applications. So even though it's a nice idea to add constraints to the db itself as a last defense, it has a cost.

## query! and query_as!
In order for query! and query_as! to work, the DATABASE_URL needs to be set, so that sqlx can connect to the db. It uses this URL to connect in compile time to the DB and check for any syntax error of the used query. This makes the process of building queries a lot easier. Query will return an anonymous record, but since this is not supported by rust yet, you can't access the fields returned. Therefore you can use a Struct that matches your table columns with query_as! in order to fill your struct with the table data.

### Offline Building
If you don't want to have your test db connected all the time, you can use the built in offline mode. It uses a cached version of your db to make the syntax checking work. In order to cache your current DB you need to use:
```bash
cargo sqlx prepare
```
This will save the metadata to the .sqlx file in the current work directory and you can just run your project without a connection from there on. If you check this into version control, there will not be a need for an active database connection while developing anymore.

## Optimization
Verifying the queries at compile time takes up a lot of ressources. In order to make this a bit faster you can add
```toml
[profile.dev.package.sqlx-macros]
opt-level = 3
```
to your cargo.toml.

## build.rs
In order to trigger recompilation when a new migration is added, you can use a "build.rs" script and type 
```bash
sqlx migrate build-script
```
