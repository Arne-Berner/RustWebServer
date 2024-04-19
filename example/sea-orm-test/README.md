# Prepartion
Pull the docker image with:
```bash
sudo docker pull mysql/mysql-server:latest
```
check the name:
```bash
sudo docker images
```
run the image:
```bash
sudo docker run --name=[container_name] -d [image_tag_name]
# in this case
docker run --name <name you want to use> -e MYSQL_ROOT_PASSWORD=thepassword -d -p 3306:3306 mysql:latest
# or if it is running but stopped
docker start <ID>

```
check if it's running
```bash
docker ps
```
check ip:
```bash
docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' <image name>
```
use the ip here:
```bash
mysql -h <mysql-container-ip> -u root -p
```
if your ip is not allowed to access it:
```bash
docker exec -it <image name> mysql -u root -p
use <db_name>; # bakeries_db
desc <table_name>;
select * from <table_name>;
```
and in mysql
```sql
GRANT ALL PRIVILEGES ON *.* TO 'root'@'<MYIP>' IDENTIFIED BY 'your-password' WITH GRANT OPTION;
```

# seaorm
The main file will create a bakery db, if it does not exist. It's currently using async-std-native instead of tokio, as can be seen in the toml. The futures crate would also not be necessary, since I would use tokio for async/await.

## migrations
There are two ways to add migrations to SeaOrm: 
- the client tool
- programatically

I chose to use the client tool, since there does not seem to be much of a difference, other than the API version needing more boilerplate code.
```bash
# Install `sea-orm-cli`
$ cargo install sea-orm-cli

# List all available migration commands that are supported by `sea-orm-cli`
$ sea-orm-cli migrate -h
```
initialize the migrations folder:
```bash
$ sea-orm-cli migrate init

# The folder structure will be as follows:
.
├── Cargo.toml
├── migration
│   ├── Cargo.toml
│   ├── README.md
│   └── src
│       ├── lib.rs
│       ├── m20220101_000001_create_table.rs
│       └── main.rs
└── src
    └── main.rs
```
after that you can go into migration and use: (dont forget to export DATABASE_URL='mysql://root:password@localhost:3306/bakeries_db'
```bash
    cargo run -- generate MIGRATION_NAME
```
Make sure to delete the existing create_table migration. Also remove the mod pub in lib.rs. Sensible names would be something like: create_chef_table. After you have generated a migration, you just need to fill the file, with whatever you wanted to do. You can see all the most important migration statements in the README in migration.

## CAREFULL
The order in the vec in the migration lib is important. It also seems like the migrations that have been done before get cached somewhere. So just wildly changing migrations and trying to up or down anything, does not work. So if you run into any "missing file" errors, just run cargo run -- fresh (although this might drop all your data)

## generate entities
After you have generated a database, you can generate entities from that. Use this in the root directory
```bash
sea-orm-cli generate entity \
    -u mysql://root:password@localhost:3306/bakeries_db \
    -o sea-orm-repository/src/entities
```
You will create a entity file with that:
```bash
bakery-backend
├── Cargo.toml
├── migration
│   └── ...
└── src
    ├── entities
    │   ├── chef.rs
    │   ├── bakery.rs
    │   ├── mod.rs
    │   └── prelude.rs
    └── main.rs
```

### Entities first
This approach can be found in the Entities first folder:

It is also possible to [have Entities first and then create migrations from that](https://www.sea-ql.org/SeaORM/docs/schema-statement/create-table/).
In order to make that work, you need to create an Entities lib which you can use in your main and from your migration binary. There are a few benefits and problems to this approach.  
The problems:
- it's not intuitive to set up and rarely documented
- you need to separate your entities into a lib
- you have to do migrations anyway
- you can't see what changed from one entity to the next in the migrations, if they got changed.
- you have to set indices and similar things anyway
- you can't change a column or something similar without losing all your data or the migration would not be backward compatible

The benefits:
- it's probably slightly less boilerplate in the long run and can even be [shortened like so](https://dev.to/aaronleopold/starter-axum-graphql-and-seaorm-template-3bnf)

It was nice as an idea to get it running, but it's very impractical. SeaORM would need more of an EF approach that keeps track of the structs to make this a good approach. The file structure will look like this at the end:

```bash
bakery-backend
├── Cargo.lock
├── Cargo.toml
├── entities
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       ├── cake.rs
│       └── lib.rs
├── migration
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── README.md
│   └── src
│       ├── lib.rs
│       ├── m20240404_140650_add_cake_table.rs
│       └── main.rs
├── README.md
└── src
    └── main.rs
```
(although I don't know why the migration number starts somewhere in the 100k range)

## CRUD
### Create
```rust
async fn run() -> Result<(), DbErr> {

    ...

    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let res = Bakery::insert(happy_bakery).exec(db).await?;
}
```
Only active models will be affected by any changes. If a value that should be set is not set, it will result in an error when inserting or updating but not when setting up the activemodel. Using the result of the last insert, we can add a chef to a bakery:
```rust
let john = chef::ActiveModel {
    name: ActiveValue::Set("John".to_owned()),
    bakery_id: ActiveValue::Set(res.last_insert_id), // a foreign key
    ..Default::default()
};
Chef::insert(john).exec(db).await?;
```

### Update
Updating this bakery after a big trauma is easy:
```rust
let sad_bakery = bakery::ActiveModel {
    id: ActiveValue::Set(res.last_insert_id),
    name: ActiveValue::Set("Sad Bakery".to_owned()),
    profit_margin: ActiveValue::NotSet,
};
sad_bakery.update(db).await?;
```

### Read
```rust
// Finding all is built-in
let bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
assert_eq!(bakeries.len(), 1);

// Finding by id is built-in
let sad_bakery: Option<bakery::Model> = Bakery::find_by_id(1).one(db).await?;
assert_eq!(sad_bakery.unwrap().name, "Sad Bakery");

// Finding by arbitrary column with `filter()`
let sad_bakery: Option<bakery::Model> = Bakery::find()
    .filter(bakery::Column::Name.eq("Sad Bakery"))
    .one(db)
    .await?;
assert_eq!(sad_bakery.unwrap().id, 1);
```
Finding something is straight forward. But notice that there is also the possibility to find related entities in two ways:
1. Lazy Loading
```rust
// Find a cake model first
let cheese: Option<cake::Model> = Cake::find_by_id(1).one(db).await?;
let cheese: cake::Model = cheese.unwrap();

// Then, find all related fruits of this cake
let fruits: Vec<fruit::Model> = cheese.find_related(Fruit).all(db).await?;
```
This will require 2 IO roundtrips, but is more readable. 
2. Eager Loading
```rust
let fruits_and_cakes: Vec<(fruit::Model, Option<cake::Model>)> = Fruit::find().find_also_related(Cake).all(db).await?;
```
This will load fruits and cakes in one go.
### Delete
Let's destroy some sadness:
```rust
let john = chef::ActiveModel {
    id: ActiveValue::Set(1), // The primary key must be set
    ..Default::default()
};
john.delete(db).await?;

let sad_bakery = bakery::ActiveModel {
    id: ActiveValue::Set(1), // The primary key must be set
    ..Default::default()
};
sad_bakery.delete(db).await?;

let bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;
assert!(bakeries.is_empty());
```
You don't need to set everything for certain actions. That's why active model is preferably used instead of just Model, where you would have to set everything.

## Should you use SeaORM or SQLx for an async db interface?
SeaORM pros:
- complete typesafety with seaquery
- pure rust
- no need to learn SQL in the different dialects
- if you change your db, you can keep the queries and only need to change the endpoint

cons: 
- need to learn a dsl that can only be used in seaorm (not possible to change easily)
- lots of boilerplate
- need to know how relational db's work anyway
- migrating from an old system that used raw sql is hard
- some features of dbs can only be accessed by raw sql

SQLx pros:
- easy to transfer sql knowledge to this
- learning sql will not be obsolete for some time
- vast ressources for the query you are trying to accomplish (no rust knowledge needed.. looking at you chatgpt)
- somewhat lightweight compared to seaorm

cons:
- writing my own SQL is error prone and might be slower
- need to implement some own structures for caching if something is read only for example
- accepts wrong sql code and ide can't help with autocompletion

This decision would be much easier, if seaOrm had any upside when it comes to migrations. But SeaOrm migrations are cumbersome and need a lot of boilerplate. Not only that, you can't generate migrations from entities in a usefull way. Any changes to your entities would break the migration and you mostly need to write your migration anyway with the entity first approach in seaorm! Using a gui tool like pgadmin or mysql workbench can help a lot with setting up constraints and similar things to produce correct SQL code for nice migrations in sqlx. Chatgpt, Stackoverflow and co will be much more usefull for sqlx too. SeaOrm feels like a "more boilerplate" version of SQLx right now and the autocompletion can not weight up to the benefits from SQLx.

TL;DR: At the stage where SeaORM is right now, I would say it's better to use SQLx.
