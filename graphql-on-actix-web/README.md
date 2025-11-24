Biasanya gue kalo bikin api pake Actix Web itu response JSON nya segelondongan langsung di kirim, dan biasanya perlu beberapa endpoint untuk mengambil data berbeda.
Selain itu kadang beberapa field kaga dibutuhin suka kebawa dan adalagi misal butuh data dari sumber lain harus request lagi juga. Jadi gue mau nyobain GraphQL yang lebih ringkas, bisa mangkas data, dan hemat bandwidth katanya.

Ouh iya gue ga akan bahas sejarah dan kenapa GraphQL ini bisa ada di alam semesta kalo Lo pingin tau lebih detail Lo bisa pergi ke website nya <a href="https://graphql.org">GraphQL</a> atau bisa baca di mbah <a href="https://en-wikipedia-org.translate.goog/wiki/GraphQL?_x_tr_sl=en&_x_tr_tl=id&_x_tr_hl=id&_x_tr_pto=tc">Wiki Pedia</a>

# Agenda
- Create Rust Project
- Setup Actix Web
- Setup Sqlx + Postgres
- Setup GraphQL
- Create, Read, Update, Delete pakai GraphQL IDE
- Setup Svelte Project
- Setup Apollo Client
- Create, Read, Update, Delete pakai Apollo Client

<hr>

# Create Rust Project

Untuk membuat project rust, pertama harus install `rust` dan `cargo` dulu. Untuk menginstallnya bisa lihat [di sini](https://www.rust-lang.org/tools/install). Install aja tingal next next next dan next ya next aja pokoknya.

Abis itu buka terminal lalu ketik:
```bash
cargo --version
cargo 1.87.0 (99624be96 2025-05-06)

rustc --version
rustc 1.87.0 (17067e9ac 2025-05-09)
```

Ini versi rust yang gue install. Oke lanjut kita buat project rustnya dengan mengetikkan:
```bash
cargo new graphql-on-actix-web
```

Kalo udh nanti bakal ada folder isinya:
```bash
graphql-on-actix-web
├── Cargo.toml
├── src
│   ├── main.rs
└── Cargo.toml
└── .gitignore
```

Kalo udah buka di code editor yang lu pake bro, kalo gue bisanya pake code editor 1 miliar umat (VS Code).

<hr>

# Setup Actix Web
Buka file Cargo.toml lalu tambahkan:
```toml
[dependencies]
actix-cors = "0.7.1"
actix-web = "4.11.0"
async-graphql = "7.0.17"
async-graphql-actix-web = "7.0.17"
chrono = { version = "0.4.41", features = ["serde"] }
dotenvy = "0.15.7"
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
sqlx = { version = "0.8.6", features = ["runtime-tokio-native-tls", "postgres", "chrono"] }
tokio = { version = "1.45.1", features = ["full"] }
```

Kalo versi rustnya sama kaya gue di 1.87.0, harusnya aman tidak terjadi error. Kalo misal ada paling harus install sesuai versi rust yang lu pake bro.

Kalo udah buka file `src/main.rs` lalu tambahkan:
```rust
use actix_web::*;

async fn hello() -> &'static str {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {

    App::new()
        .route("/", web::get().to(hello))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
```

Kalo udah buka terminal lalu ketik:
```bash
cargo run
```
Kalo aman coba buka url http://localhost:8000 di browser, postman atau pake tools lainnya.

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/hello-world.png" alt="Hello World" />

<hr>

# Setup Sqlx + Postgres
Buat file baru di root project namanya `.env` lalu tambahkan:
```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres?sslmode=disable
```
Kemudian buat folder `connection` di src lalu buat file `db.rs` lalu tambahkan:

```bash
graphql-on-actix-web
├── connection
│   ├── db.rs // file baru
├── src
│   ├── main.rs
└── Cargo.toml
└── Cargo.lock
└── .env
└── .gitignore
```

```rust
use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;

pub async fn get_pg_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.expect("Failed to create pool")
}
```

Buka file `src/main.rs` lalu tambahkan:
```rust
use actix_web::*;
use sqlx::*;

use crate::connection::db; // use module db di folder connection

mod connection { // daftarkan folder connection agar menjadi module
    pub mod db;
}

async fn hello(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query("SELECT 1")
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(row) => {
            let value: i32 = row.get(0);
            HttpResponse::Ok().body(format!("Query returned: {}", value))
        },
        Err(err) => {
            eprintln!("DB error: {}", err);
            HttpResponse::InternalServerError().body("DB error")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let connection = db::get_pg_pool().await; // panggil fungsi get_pg_pool

    HttpServer::new(move || {

    App::new()
        .app_data(web::Data::new(connection.clone())) // clone connection masukan ke app data
        .route("/", web::get().to(hello))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
```

Kalo udah matikan terminal lalu jalankan kembali lalu request lagi ke url http://localhost:8000 di browser, postman atau pake tools lainnya.

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/select-1.png" alt="Hello World" />

Ouh iya biar ngga mati dan hidupin ulang servernya. tambahkan di `Cargo.toml`:
```toml
[dependencies]
# ......
cargo-watch = "8.5.3"
```
Kemudian jalankan perintah:
```bash
cargo watch -x run
```
Dengam begini server actix web akan selalu auto reload kalau ada perubahan di file .rs.

Sip, kalo usah kita mulai setup untuk GraphQl nya.

<hr>

# Setup GraphQl
Untuk menggunakan GraphQL kita butuh `async-graphql` dan `async-graphql-actix-web`. sebelumnya kita udah menambahkannya di `Cargo.toml` jadi selanjutnya kita akan setup GraphQl IDE dan Endpoint GraphQL.

## Setup GraphQl IDE

Buka file `src/main.rs` lalu tambahkan function baru:
```rust
async fn graphiql() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/query").finish())
}
```

Dan tambahkan juga di dalam function main:
```rust
App::new()
    .app_data(web::Data::new(connection.clone()))
    .route("/", web::get().to(hello))
    .route("/console/graphql", web::get().to(graphiql)) // tambahkan route /console/graphql
```

Untik url endpointnya bebas apa aja biasanya `/graphiql`. Kalo udah coba buka url http://localhost:8000/console/graphql di browser.

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/graphql-console.png" alt="GraphiQl" />

Kalo tidak ada error harusnya akan muncul halaman GraphiQl seperti code editor. kita bisa menggunakan query GraphQl di GraphiQl ini. Udah bisa pake? Belum lah wkwkwk, kita harus buat dulu endpoint GraphQL nya.

## Setup Endpoint GraphQL

Biar project lebih reausable dan clean, kita buat folder baru di src namanya `graphql` lalu buat file `schema.rs` disini untuk menyimpan configurasi GraphQL.:

```bash
graphql-on-actix-web
├── connection
│   ├── db.rs
├── graphql
│   ├── schema.rs // file baru
├── src
│   ├── main.rs
└── Cargo.toml
└── Cargo.lock
└── .env
└── .gitignore
```

```rust
use async_graphql::{http::GraphiQLSource, *};
use actix_web::*;
use async_graphql_actix_web::*;
use sqlx::*;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str { // buat function hello
        "Hello from Actix + GraphQL!"
    }
}

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>; // buat type AppSchema

pub fn create_schema() -> AppSchema { // buat function create_schema
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .finish()
}

pub async fn graphiql() -> actix_web::HttpResponse { // untuk graphiql ide kita pindahkan ke file schema
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/query").finish())
}

pub async fn graphql_handler( // buat function graphql_handler untuk mengelola request GraphQL
    schema: web::Data<AppSchema>, 
    pool: web::Data<PgPool>, 
    req: GraphQLRequest
) -> GraphQLResponse {
    schema
        .execute(req.into_inner().data(pool.get_ref().clone())) // Inject disini bro!
        .await
        .into()
}
```

Jika sudah selsai kita tambahkan module graphql dan file schema.rs ke main.rs
```rust
use actix_web::*;

use crate::{connection::db, graphql::schema::graphiql};

mod connection {
    pub mod db;
}

mod graphql { // daftarkan folder graphql sebagai module
    pub mod schema;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let connection = db::get_pg_pool().await;

    HttpServer::new(move || {

    App::new()
        .app_data(web::Data::new(connection.clone()))
        .route("/console/graphql", web::get().to(graphiql))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
```

Ouh iya Untuk function `hello` di main.rs dihapus aja karena kita sudah berhasil melakukan setup connection ke database. Kemudian kita panggil function, type dan struct di file schema.rs.
```rust
use actix_web::*;

use crate::{
  connection::db, 
  graphql::schema::{create_schema, graphiql, graphql_handler} // jangan lupa use dulu
};

mod connection {
    pub mod db;
}

mod graphql {
    pub mod schema;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let connection = db::get_pg_pool().await;
    let schema = create_schema(); // buat variable schema

    HttpServer::new(move || {

    App::new()
        .app_data(web::Data::new(connection.clone()))
        .app_data(web::Data::new(schema.clone())) // clone schema ke app data
        .route("/query", web::post().to(graphql_handler)) // buat route /query karena kita membuat endpoint yang di panggil ke GraphIQL IDE adalah /query
        .route("/console/graphql", web::get().to(graphiql))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
```
Kalo aman tidak ada error buka GraphiQl IDE di http://localhost:8000/console/graphql lalu buat query GraphQl seperti ini:

```graphql
query {
    hello
  }
```
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/query-hello.png" alt="Hello World" />
Jika responsenya seperti ini berarti berhasil.

# Create, Read, Update, Delete pakai GraphQL IDE
Selanjutnya kita akan membuat CRUD menggunakan GraphQL IDE. Tapi sebelum itu kita perlu melakukan perubahan dulu di project kita biar lebih reausable dan clean. Arsitekturnya akan seperti ini:
```bash
graphql-on-actix-web
├── connection // berisi konfigurasi connection ke database
│   ├── db.rs
├── handlers // untuk mengelola request GraphQL
│   ├── order_handler.rs
├── graphql // berisi konfigurasi GraphQL
│   ├── schema.rs
├── models // berisi object model
│   ├── orders_model.rs
├── services // berisi logic business
│   ├── orders_service.rs
├── src
│   ├── main.rs
└── Cargo.toml
└── Cargo.lock
└── .env
└── .gitignore
```
Kalo usah kita buka folder `graphql` di file `schema.rs` kita akan ubah beberapa konfigurasi

- Kita tau kalau request graphql dan function untuk handler query requestnya berupa object. Artinya pada file `order_handler.rs` nantinya adalah sebuah object dan misalnya kita buat handler lain maka juga akan berupa object. Jadi kita akan merge object tersebut dengan macro keyword `MergedObject` yang ada di `async_graphql` crate.
- Hapus struct `QueryRoot` dan ganti dengan ini:
```rust
#[derive(MergedObject, Default)]
pub struct QueryRoot(
    // disini nanti untuk handler graphql lainnya
);
```
- Kemudian perbaiki type `AppSchema` dan function `create_schema` seperti ini:
```rust
pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .finish()
}
```
- Kemudian di file `order_handler.rs` tambahkan ini:
```rust
use async_graphql::*;

#[derive(Default)]
pub struct OrderQuery;

#[Object]
impl OrderQuery {
    async fn orders(&self) -> Result<serde_json::Value> {
        Ok(serde_json::json!({"orders": "orders"}))
    }
}
```

Tambahkan module `handlers`, `services` dan `models` di `src/main.rs`.:
```rust
mod handlers {
    pub mod order_handler;
}
mod models {
    pub mod user_model;
}
mod services {
    pub mod order_service;
}
```

Kalau udah tambahkan di `QueryRoot` seperti ini:
```rust
// src/graphql/schema.rs

use crate::handlers::order_handler::OrderQuery;

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    OrderQuery,
);
```

Kalau udah buka lagi url http://localhost:8000/console/graphql lalu buat query GraphQl seperti ini:

```graphql
query {
  orders
}
```

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/query-example-order.png" alt="Hello World" />

Sip waktunya CRUD

## Create Operation

### User Models dan Order Models

Buat file baru di folder `models` lalu buat file `user_model.rs` lalu tambahkan:
```rust
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Debug, Serialize)]
pub struct UserDB { // struct untuk menampung data dari database
    pub user_id: i32,
    pub email: String,
    pub full_name: String
}

#[derive(SimpleObject)]
pub struct User { // struct untuk response graphql
    pub user_id: i32,
    pub email: String,
    pub full_name: String
}

impl From<UserDB> for User {
    fn from(user: UserDB) -> Self {
        User {
            user_id: user.user_id,
            email: user.email,
            full_name: user.full_name
        }
    }
}
```
Alasan kenapa menggunakan 2 struct karena output dari database belum tentu suport dengan graphql. Misalnya output datetime yang di tangkan dengan crate `chrono`, graphql tidak tau apa itu chrono?, jadi kita harus membuat struct baru untuk ngasih tau ke graphql `eh ini datetime loh`.

Kalo udah ketikkan perintah ini untuk menginstall sqlx-cli untuk melakukan migrations di terminal:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

Lalu ketik perintan ini:
```bash
sqlx migrate add create_users_table
```
Nanti akan terbentuk folder baru dengan nama `migrations` lalu ada file `random123_create_users_table.sql` di dalamnya. Masukan query ini dan ketikan ulang `sqlx migrate add create_users_table`:

```sql
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY, 
    email VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL
);
```
Buka database bebas dengan tools apa aja, bisa PgAdmin atau sejenisnya sekarang harusnya sudah terbuat table baru dengan nama `users`. Kalo ga berhasil buat manual aja pakai query di atas.

Oke lanjut...

### Create Service

Sebelum membuat service tambahkan struct `Newuser` di `src/models/user_model.rs` dulu:
```rust
#[derive(Debug, Deserialize, InputObject)]
pub struct NewUser {
    pub email: String,
    pub full_name: String
}
```

Buat file baru di folder `services` lalu buat file `user_service.rs` lalu tambahkan:
```rust
use crate::models::user_model::{NewUser, User, UserDB};
use sqlx::PgPool;

pub struct UserService;

impl UserService {
    pub async fn create_user(pool: &PgPool, request: NewUser) -> Result<User, sqlx::Error> {
        let user_db = sqlx::query_as::<_, UserDB>(
            "INSERT INTO users (email, full_name) VALUES ($1, $2) RETURNING *"
        )
        .bind(request.email)
        .bind(request.full_name)
        .fetch_one(pool)
        .await?;
        
        Ok(User::from(user_db))
    }
}
```

### Create Handler

Buat file baru di folder `handlers` lalu buat file `user_handler.rs` lalu tambahkan: 
```rust
use async_graphql::*;

use crate::models::user_model::{NewUser, User};
use crate::services::user_service::UserService;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create_user(&self, ctx: &Context<'_>, request: NewUser) -> Result<Option<User>> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let user = UserService::create_user(pool, request).await?;
        Ok(Some(user))
    }
}
```

Ouh iya jangan lupa untuk menambahkan struct `UserMutation` di `src/graphql/schema.rs` lalu tambahkan:
```rust
use crate::handlers::user_handler::UserMutation;

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    UserMutation // mutation object
);

// pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>; sebelumnya
pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .finish()
}
```

Karena di GraphQL terdapat 2 operasi yaitu `query` dan `mutation` jadi Schema harus menerima 2 parameter yaitu `Object untuk Query` dan `Object untuk Mutation`.

Dan juga mendaftarkan file `user_handler.rs` dan `user_service.rs` di main.rs. Kalo udah buka lagi url http://localhost:8080/console/graphql di browser lalu masukan query ini:

#### Note: Default key Object Query di GraphQL adalah camelCase, artinya request dan response secara otomatis akan mengubahkan menjadi camelCase meskipun object yang kita buat adalah snake_case, PascalCase, kebab-case, dll.

```graphql
mutation {
  createUser(request: {
    email: "user1@example.com",
    fullName: "User 1"
  }) {
    userId
    email
    fullName
  }
}
```

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/create-user.png" alt="Hello World" />

Kalau tidak ada error dan response nya sesuai dengan data yang di inputkan berarti berhasil dan cek di database apakah datanya sudah masuk atau belum.

Oke lanjut ke next step yaitu Read User.

## Read User

### Simple Query

Tambahkan function untuk `get_users di file `src/services/user_service.rs` seperti ini:
```rust
pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
  let users_db = sqlx::query_as::<_, UserDB>("SELECT * FROM users")
      .fetch_all(pool)
      .await?;
  
  Ok(users_db.into_iter().map(User::from).collect())
}
```

Tambahkan handler untuk `get_users di file `src/handlers/user_handler.rs` seperti ini:
```rust
#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let pool = ctx.data::<PgPool>()?;
        let users = UserService::get_users(pool).await?;
        Ok(users)
    }
}
```

Tambahkan struct `UserQuery` di `src/graphql/schema.rs` seperti ini:
```rust
use crate::handlers::user_handler::UserQuery;

#[derive(MergedObject, Default)]
pub struct QueryRoot(
    OrderHandler // ini nanti kita perbaiki 
    UserQuery // query object
)
```
Kalo udah buka lagi url http://localhost:8080/console/graphql lalu masukan query ini:

```graphql
query {
  users {
    userId
    email
    fullName
  }
}
```

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/select-all-user.png" alt="Hello World" />

Jika berhasil maka akan menampilkan semua data user yang ada di database.

### Nested Query
Di GraphQL kita bisa mengambil data dengan nested query seperti ini:
```graphql
query {
  users {
    userId
    email
    fullName,
    orders {
      orderId
      orderName,
      orderPrice,
      orderStatus,
      orderDate,
      lastUpdate
    }
  }
}
```

Pertama kita buat table orders di database seperti ini:
```sql
-- Add migration script here
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    order_name TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    order_date TIMESTAMPTZ NOT NULL,
    order_price FLOAT NOT NULL,
    order_status TEXT NOT NULL,
    last_update TIMESTAMPTZ NOT NULL
);
```
Karena kita akan mengubah objectnya, jadi kita apan mengubah struct  `User` dengan melakukan impl `Object` seperti ini:
```rust
#[ComplexObject]
impl User {
    async fn orders(&self, ctx: &Context<'_>) -> Result<Vec<Order>, async_graphql::Error> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let orders = OrderService::get_orders(pool, self.user_id).await?;
        Ok(orders)
    }
}
```
Jangan lupa use `ComplexObject` dari `async_graphql`

Lalu ubah struct `User` jadi seperti ini:
```rust
#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub full_name: String
}
```

# Warning ! 🙀
Namun kita harus hati-hati dalam hal ini, kondisi tersebut bisa menyebabkan N+1 Query. Apa tuh? N+1 Query artinya kita melakukan query terhadap database lebih dari satu kali untuk mengambil data yang sama. Query GraphQL diatas kita bisa liat, jadi kita mengambil data users terlebih dahulu, misalnya di database ada 1 users dan kita ingin mengambil data ordersnya. Maka yang terjadi:
```sql
SELECT * FROM users; // 1 query
SELECT * FROM orders WHERE user_idd = 1; // 1 query
```
Nah bayangkan kalo ada 100 users, maka yang akan terjadi adalah:
```sql
SELECT * FROM users; // 1 query

// dapet 100 users
SELECT * FROM orders WHERE user_id = $1; -- query order untuk user 1
SELECT * FROM orders WHERE user_id = $2; -- query order untuk user 2
...
SELECT * FROM orders WHERE user_id = $100; -- query order untuk user 100
```
Mantap ngga? mantap lah wkwkwk, 1 query bisa jadi 100 query. Masih mending kalo satu user ordernya 1 atau 3, kalau 1 user ordernya `1.000` atau `1.000.000` bahkan `1.000.000.000`. Bisa nunggu data muncul sampai hari kiamat kita bro😂. Lalu gimana ngatasinnya? Ada beberapa pendekatan yang bisa dilakukan untuk mengatasi problem N+1 Query ini gue kasih 2 cara aja ya yang gue sering pake dan tanpa menambah tools lain. Kita mulai dari yang paling ribet dulu.

### Pakai DataLoader atau Batch Query bawaan async_graphql
**Note: Tapi DataLoader ini bawaaan async_graphql dari rust ya, kalau di bahasa pemrograman lain mungkin berbeda seperti di Javascript/Typescript ada library seperti `dataloader`, Python ada library seperti `strawberry / graphene`. dll**.

- Pertama buka file `src/services/order_service.rs` lalu buat struct baru seperti ini:
```rust
pub struct OrderLoader {
    pub pool: sqlx::PgPool,
}
``` 
Fungsinya cuma untuk mengambil connection dari database.

- Kemudian impl `Loader` untuk struct `OrderLoader` seperti ini:
```rust
impl Loader<i32> for OrderLoader {
    type Value = Vec<Order>;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        // Ambil semua order yang user_id ada di keys
        let pool = &self.pool;
        let orders_db = sqlx::query_as::<_, OrderDB>(
            "SELECT * FROM orders WHERE user_id = ANY($1)"
        )
        .bind(keys)
        .fetch_all(pool)
        .await?;

        // Build map: user_id => Vec<Order>
        let mut order_map: HashMap<i32, Vec<Order>> = HashMap::new();
        for order in orders_db {
            order_map
                .entry(order.user_id)
                .or_insert_with(Vec::new)
                .push(Order::from(order));
        }

        Ok(order_map)
    }
}
```
Query `SELECT * FROM orders WHERE user_id = ANY($1)` akan menghasilkan data seperti ini:
```sql
SELECT * FROM orders WHERE user_id = ANY(ARRAY[1,2,3]);
```

- Kemudian ubah struct `User` jadi seperti ini:
```rust
#[derive(SimpleObject)]
#[graphql(complex)] // tambahkan ini
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub full_name: String
}
```

- Kemudian ubah impl struct `User` jadi seperti ini:
```rust
#[ComplexObject] // jangan lupa use dulu dari async_graphql
impl User {
    async fn orders(&self, ctx: &Context<'_>) -> Result<Vec<Order>, async_graphql::Error> {
        let loader = ctx.data::<DataLoader<OrderLoader>>()?;
        let orders = loader.load_one(self.user_id).await?;
        Ok(orders.unwrap_or_default())
    }
}
```

- Tambahkan order_loader di function handler `graphql_handler` seperti ini:
```rust
pub async fn graphql_handler(
    schema: web::Data<AppSchema>, 
    pool: web::Data<PgPool>, 
    req: GraphQLRequest
) -> GraphQLResponse {
    let order_loader: DataLoader<OrderLoader> = DataLoader::new(OrderLoader { pool: pool.get_ref().clone() }, tokio::spawn);
    schema.execute(req.into_inner().data(pool.get_ref().clone())
    .data(order_loader)) // Inject disini bro!
    // .data(sales_loader) // misalnya nanti ada loader lagi
    .await
    .into()
}
```
Sudah, dengan begini query yang akan dihasilkan adalah seperti ini:
```sql
SELECT * FROM users;
SELECT * FROM orders WHERE user_id = ANY(ARRAY[user_id_1,user_id_2,user_id_3, user_id_dst]);
```
Artinya kita cukup melakukan 2 query aja. Setupnya emng lumayan ribet bro, Ya begitulah rust, bahasa pemrograman paling perfectionist wkwkwk.

### Pakai JOIN Query
Cara ini paling sederhana bro, karena kita cuma melakukan Query SQL JOIN aja kaya gini misal:
```sql
SELECT 
    u.user_id AS user_user_id,
    u.email AS user_email,
    u.full_name AS user_full_name,
    o.order_id AS order_order_id,
    o.order_name AS order_order_name,
    o.order_price AS order_order_price
FROM users u
LEFT JOIN orders o ON u.user_id = o.user_id
WHERE u.user_id = $1
```
Untuk mengambil data dari query itu, kita perlu menangkapnya dengan struct seperti ini untuk di mapping ke `sqlx::query_as`:
```rust
#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct UserWithOrderRow {
    pub user_id: i32,
    pub user_email: String,
    pub user_full_name: String,
    pub order_id: Option<i32>,
    pub order_name: Option<String>,
    pub order_date: Option<chrono::DateTime<chrono::Utc>>,
    pub order_price: Option<f64>,
    pub order_status: Option<String>,
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(SimpleObject)]
pub struct UserWithOrders {
    pub user_id: i32,
    pub email: String,
    pub full_name: String,
    pub orders: Vec<Order>,
}
```

Kemudian tambahkan function baru di `src/services/order_service.rs` seperti ini:
```rust
pub async fn get_users_with_orders(pool: &PgPool) -> Result<Vec<UserWithOrders>, sqlx::Error> {
    let rows = sqlx::query_as::<_, UserWithOrderRow>(
        r#"
        SELECT 
            u.user_id AS user_id,
            u.email AS user_email,
            u.full_name AS user_full_name,
            o.order_id AS order_id,
            o.order_name AS order_name,
            o.order_price AS order_price,
            o.order_date AS order_date,
            o.order_status AS order_status,
            o.last_update AS last_update
        FROM users u
        LEFT JOIN orders o ON u.user_id = o.user_id
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut map: HashMap<i32, UserWithOrders> = HashMap::new();

    for row in rows {
        // entry per user
        let user_entry = map.entry(row.user_id).or_insert_with(|| UserWithOrders {
            orders: Vec::new(),
            user_id: row.user_id,
            email: row.user_email.clone(),
            full_name: row.user_full_name.clone(),
        });

        // kalau order ada
        if let Some(order_id) = row.order_id {
            user_entry.orders.push(OrderDB {
                order_id,
                order_name: row.order_name.clone().unwrap(),
                order_price: row.order_price.unwrap(),
                order_date: row.order_date,
                order_status: row.order_status.clone().unwrap(),
                last_update: row.last_update,
                user_id: row.user_id,
            }.into());
        }
    }

    Ok(map.into_values().collect())
}
```
Terakhir pada handler `src/handlers/user_handler.rs` tambahkan ini:
```rust
    // Kita comment dulu untuk yang pake DataLoader
    // async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
      //     let pool = ctx.data::<PgPool>()?;
      //     let users = UserService::get_users(pool).await?;
      //     Ok(users)
      // }
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<UserWithOrders>> {
        let pool = ctx.data::<PgPool>()?;
        let users = UserService::get_users_with_orders(pool).await?;
        Ok(users)
    }
```
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/query-users-with-orders.png" alt="Hello World" />

Hasilnya sama aja namun ada kelebihan dan kekurangannya masing-masing.
- Kelebihan DataLoader:
  - Fleksible bisa lazy load fields, cocok buat GraphQL yang field-nya bisa request sebagian.
  - Modular bisa reuse di resolvers lainnya tanpa duplikat SQL
  - No redundant rows karena cuma ambil data yg dipakai, diolah di memory Rust, bukan di DB
- Kekurangannya DataLoader:
  - Minimal 2 query lebih (1 ambil Users + 1 ambil Orders)
  - Butuh Loader setup dan kebayang ribetnya tadi, tapi mungkin jika selain rust bakal lebih mudah
  - Kalau di tingkat relational sederhana loader mungkin lebih lambat dari JOIN

- Kelebihan JOIN
  - Cukup 1 query aja tidak ada N+1, full data di-fetch sekaligus.
  - Efisien mungkin lebih cocok untuk relasi sederhana
  - Minimal Latency karena hanya 1 round-trip ke database
- Kekurangannya JOIN
  - Kurang cocok untuk relasi kompleks kalau ada nested 3-4 table, JOIN bisa berat dan rawan Cartesian explosion (jumlah rows membengkak)
  - Redundant data user info diulang di setiap row order (boros memori di sisi app).
  - Kurang fleksibel GraphQL tidak bisa granular (misal hanya mau ambil field tertentu), harus disusun di query manual

Kalo gue disini lebih milih pake `DataLoader` bro, karena bisa custom response juga jadi tidak kebanyakan field yang di ulang - ulang. Tapi balik lagi sesuai kebutuhan aja.

## Update Operation
Ternyata panjang juga membahas soal query di GraphQL😁. Yaudah lanjut ke update mutation. Kita akan lanjut di orders data. 

Buuat struct baru di `src/models/order_model.rs`:
```rust
#[derive(Debug, Deserialize, InputObject, Serialize)]
pub struct UpdateOrder {
    pub order_id: i32,
    pub user_id: Option<i32>,
    pub order_name: Option<String>,
    pub order_price: Option<f64>,
    pub order_status: Option<String>,
}
```
Kemudian buat logic querynya di `src/services/order_service.rs`:
```rust
pub async fn update_order(pool: &PgPool, request: UpdateOrder) -> Result<Option<Order>, sqlx::Error> {
    let result = sqlx::query_as::<_, OrderDB>(
        r#"
        UPDATE orders 
        SET order_name = COALESCE($2, order_name), 
            user_id = COALESCE($3, user_id),
            order_price = COALESCE($4, order_price),
            order_status = COALESCE($5, order_status)
        WHERE order_id = $1
        RETURNING *
        "#
    )
    .bind(request.order_id)
    .bind(request.order_name)
    .bind(request.user_id)
    .bind(request.order_price)
    .bind(request.order_status)
    .fetch_optional(pool)
    .await?;
    
    Ok(result.map(Order::from))
}
```

Lalu buat handler di `src/handlers/order_handler.rs` masukan ke struct `OrderMutation`:
```rust
async fn update_order(&self, ctx: &Context<'_>, request: UpdateOrder) -> Result<Option<Order>> {
    let pool = ctx.data::<sqlx::PgPool>()?;
    let order = OrderService::update_order(pool, request).await?;
    Ok(order)
}
```

Kalau udah jalankan query GraphQL ini di GraphiQL IDE:

```graphql
mutation {
  updateOrder(request: { 
    orderId: 1
    userId: 1,
    orderStatus: "lunas"
    orderName: "Beli Sabun" 
    orderPrice: 7000
  }) {
    orderId
    orderName
    orderDate
    orderPrice
    orderStatus
  }
}
```

Selebelumnya datanya seperti ini:
```json
{
  "orderId": 1,
  "orderName": "Beli Sabun",
  "orderPrice": 100,
  "orderStatus": "pending",
  "orderDate": "2025-06-19T07:47:27.315848+00:00",
  "lastUpdate": "2025-06-19T07:47:27.315892+00:00"
}
```
Setelah update datanya seperti ini:
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/update-order.png" alt="Update Order" />

## Delete Operation
Untuk delete ini cukup mudah, karena kita tinggal delete data sesuai id mana. 
Buat function baru di `src/services/order_service.rs`:
```rust
pub async fn delete_order(pool: &PgPool, order_id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM orders WHERE order_id = $1")
        .bind(order_id)
        .execute(pool)
        .await?;

    // Mengecek berapa row yang kena delete
    Ok(result.rows_affected() > 0)
}
```

Lalu buat handler di `src/handlers/order_handler.rs` masukan ke struct `OrderMutation`:
```rust
async fn delete_order(&self, ctx: &Context<'_>, order_id: i32) -> Result<bool> {
    let pool = ctx.data::<sqlx::PgPool>()?;
    let result = OrderService::delete_order(pool, order_id).await?;
    Ok(result)
}
```

Kalau udah jalankan query GraphQL ini di GraphiQL IDE:

```graphql
mutation {
  deleteOrder(orderId: 1) 
}
```

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/delete-order.png" alt="Delete Order" />

Kelar bro untuk CRUD nya. Selanjutnya coba kita implementasi ke frontend. Untuk frontend nya gue mau buat pake Sveltekit.

Sebelum setup frontend projectnya, kita harus terlebih dahulu membuka `Cors` di `src/main.rs` ubah main function jadi seperti ini:
```rust
async fn main() -> std::io::Result<()> {

    let connection = db::get_pg_pool().await;
    let schema = create_schema();

    HttpServer::new(move || {

    let cors = actix_cors::Cors::default()
    .allow_any_origin()
    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
    .allowed_headers(vec![http::header::CONTENT_TYPE])
    .max_age(3600)
    .supports_credentials();

    App::new()
        .app_data(web::Data::new(connection.clone()))
        .app_data(web::Data::new(schema.clone()))
        .route("/query", web::post().to(graphql_handler))
        .route("/console/graphql", web::get().to(graphiql))
        .wrap(cors) // cors udah didefinisikan di sini langsung
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
```

# Setup Svelte Project
Untuk membuat svelte project kita memerlukan `Node Package Manager` yaitu `npm`. Untuk menginstallnya bisa lihat [di sini](https://nodejs.org/en/download/). Install aja tingal next next next dan next ya next aja pokoknya. Kalo udah terus cek `npm` nya udh ada atau belum.

```bash
npm --version
```

Okeh kita mulai membuat Sveltekit projectnya dengan mengetikkan perintah:

```bash
npx sv create graphql-on-svelte
```
Kalo udah ketikkan perintah `npm run dev` untuk menjalankan frontend projectnya.

```bash
npm run dev

->   http://localhost:5173
```

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/sveltekit.png" alt="Sveltekit" />

Kita akan menginstall beberapa package terlebih dahulu seperti ini:

- `@apollo/client`
- `graphql`
- `bootstrap`

```bash
npm install @apollo/client graphql bootstrap
```

Untuk struktur frontend projectnya akan seperti ini:
```bash
graphql-on-svelte
├── .svelte-kit // default konfigurasi dari sveltekit
├── node_modules // dependency dari npm
├── src // berisi source code frontend kita
│   ├── components // berisi file untuk component
│   ├── lib // berisi file main function
│   ├── routes // berisi file route
│   ├── app.html
└── static // file - file public
└── .gitignore
└── .npmrc
└── jsconfig.json
└── package-lock.json
└── package.json
└── svelte.config.js
└── vite.config.js
```

## Setup Layout dan Page

Kalo udah buat file baru di `src/routes` dengan nama `+layout.svelte` lalu isi seperti ini:

```js
<script lang="js">
    import 'bootstrap/dist/css/bootstrap.min.css';
    import * as bootstrap from 'bootstrap';
    const { children } = $props();
</script>

<div class="container mt-5">
    {@render children()}
</div>
```

Kemudian di `src/routes/+page.svelte` tambahkan ini:
```js
<script>
  import { slide } from "svelte/transition";
  let expandedRow = $state(null);
  let data = $state([]);

  const toggleRow = (index) => {
    expandedRow = expandedRow === index ? null : index;
  };
</script>

<div class="row gap-3">
  <div class="col-12">
    <div class="card">
      <div class="card-body">
        <button class="btn btn-primary">Create User</button>
        <button class="btn btn-success">Create Order</button>
      </div>
    </div>
  </div>
  <div class="col-12">
    <div class="card">
      <div class="card-body">
        <table class="table table-bordered">
          <thead>
            <tr>
              <th>#</th>
              <th>Email</th>
              <th>Full Name</th>
              <th>Action</th>
            </tr>
          </thead>
          <tbody>
            {#each data as user, i}
              <tr>
                <td>{user.userId}</td>
                <td>{user.email}</td>
                <td>{user.fullName}</td>
                <td>
                  <button
                    class="btn btn-sm btn-primary"
                    onclick={() => toggleRow(i)}
                  >
                    {expandedRow === i ? "Collapse" : "Expand"}
                  </button>
                </td>
              </tr>
              {#if expandedRow === i}
                <tr>
                  <td colspan="4">
                    <div transition:slide>
                      <table class="table table-sm table-striped">
                        <thead>
                          <tr>
                            <th>Order ID</th>
                            <th>Name</th>
                            <th>Price</th>
                            <th>Status</th>
                            <th>Date</th>
                          </tr>
                        </thead>
                        <tbody>
                          {#each user.orders as order}
                            <tr>
                              <td>{order.orderId}</td>
                              <td>{order.orderName}</td>
                              <td>{order.orderPrice}</td>
                              <td>{order.orderStatus}</td>
                              <td>{order.orderDate}</td>
                            </tr>
                          {/each}
                        </tbody>
                      </table>
                    </div>
                  </td>
                </tr>
              {/if}
            {/each}
          </tbody>
        </table>
      </div>
    </div>
  </div>
</div>
```
Ouh iya buat file `+layout.js` di `src/routes` untuk untuk mendisabled `ssr` component, tambahkan ini:
```js
export const ssr = false;
```

## Setup Apollo Client
Untuk setup apollo client buka file `src/lib/index.js` lalu tambahkan code ini:
```js
import { ApolloClient, InMemoryCache, HttpLink } from '@apollo/client/core';

const client = new ApolloClient({
  link: new HttpLink({ uri: 'http://localhost:8080/query' }),
  cache: new InMemoryCache(),
});

export default client;
```

## Read Data Pakai Apollo Client
Kita akan mulai dari Read Data dulu pakai apollo client. dan untuk mengetes apakah apollo client jalan atau tidak. 

Buat file `query.js` di `src/lib` disini untuk menyimpan query-query graphql kita. Kemudian tambahkan code ini:
```graphql
import { gql } from "@apollo/client/core";

export const getUsersWithOrders = gql`
    query {
        users {
            userId
            email
            fullName,
            orders {
                orderId
                orderName,
                orderPrice,
                orderStatus,
                userId,
                orderDate
            }
        }
    }
`;
```
Buka file `src/routes/+page.svelte` lalu tambahkan code ini:
```js
<script>
  import client from "$lib";
  import { getUsersWithOrders } from "$lib/query";
  import { onMount } from "svelte";

  onMount(() => {
    const res = await client.query({
      query: getUsersWithOrders,
    });
    data = await res.data.users;
  });
</script>
```
Kalau sudah buka browser di url `http://localhost:5173` atau url frontend lu bro, biasanya kalo vite urlnya `http://localhost:5173/`.

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/read-data-svelte.png" alt="Hello World" />

Kita bagusin dikit web kita pake icons dari bootstrap dan sweetalert2 agar pop up biar lebih enak dilihat. Tambahkan ini bro:
```bash
npm i bootstrap-icons sweetalert2
```

Kemudian tambahkan ini di `+layout.svelte`:
```js
<script lang="js">
  import "bootstrap-icons/font/bootstrap-icons.css"; // tambahkan ini

  //....
</script>
```

Kemudian kita perbaiki sedikit project frontend kita biar lebih reusable. Buat file `Table.svelte` di `src/components` gue mau pindahin table nya ke file itu. isinya seperti ini:
```js
<script>
    import { slide } from "svelte/transition";

    const { data=[], expandedRow, toggleRow } = $props();
</script>

<table class="table table-bordered">
  <thead>
    <tr>
      <th>#</th>
      <th>Email</th>
      <th>Full Name</th>
      <th>Action</th>
    </tr>
  </thead>
  <tbody>
    {#each data as user, i}
      <tr>
        <td>{user.userId}</td>
        <td>{user.email}</td>
        <td>{user.fullName}</td>
        <td>
          <button
            class="btn btn-sm btn-primary"
            aria-label="Toggle row"
            onclick={() => toggleRow(i)}
          >
            {#if expandedRow === i}
              <i class="bi bi-dash"></i>
            {:else}
              <i class="bi bi-plus"></i>
            {/if}
          </button>
          <button class="btn btn-sm btn-success" aria-label="Edit">
            <i class="bi bi-pencil"></i>
          </button>
          <button class="btn btn-sm btn-danger" aria-label="Delete">
            <i class="bi bi-trash"></i>
          </button>
        </td>
      </tr>
      {#if expandedRow === i}
        <tr>
          <td colspan="4">
            <div transition:slide>
              <table class="table table-sm table-striped">
                <thead>
                  <tr>
                    <th>Order ID</th>
                    <th>Name</th>
                    <th>Price</th>
                    <th>Status</th>
                    <th>Date</th>
                    <th>Action</th>
                  </tr>
                </thead>
                <tbody>
                  {#each user.orders as order}
                    <tr>
                      <td>{order.orderId}</td>
                      <td>{order.orderName}</td>
                      <td>{order.orderPrice}</td>
                      <td>{order.orderStatus}</td>
                      <td>{order.orderDate}</td>
                      <td>
                        <button
                          class="btn btn-sm btn-success"
                          aria-label="Edit"
                        >
                          <i class="bi bi-pencil"></i>
                        </button>
                        <button
                          class="btn btn-sm btn-danger"
                          aria-label="Delete"
                        >
                          <i class="bi bi-trash"></i>
                        </button>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          </td>
        </tr>
      {/if}
    {/each}
  </tbody>
</table>
```

Untuk `+page.svelte` hanya seperti ini:
```js
<script>
  import client from "$lib";
  import { getUsersWithOrders } from "$lib/query";
  import { onMount } from "svelte";
  import Table from "../components/Table.svelte";

  let expandedRow = $state(null);
  let data = $state([]);

  const toggleRow = (index) => {
    expandedRow = expandedRow === index ? null : index;
  };

  onMount(async () => {
    const res = await client.query({
      query: getUsersWithOrders,
    });
    data = await res.data.users;
  })
</script>

<div class="row gap-3">
  <div class="col-12">
    <div class="card">
      <div class="card-body">
        <button class="btn btn-primary">
          <i class="bi bi-cart-plus"></i> Tambah Userr
        </button>
        <button class="btn btn-success">
          <i class="bi bi-cart-plus"></i> Tambah Order
        </button>
      </div>
    </div>
  </div>
  <div class="col-12">
    <div class="card">
      <div class="card-body">
        <Table {data} {toggleRow} {expandedRow} />
      </div>
    </div>
  </div>
</div>
```

Kemudian gue mau buat 2 file di `src/lib` yaitu `user.js` dan `order.js` disini untuk menyimpan function untuk create, edit dan delete untuk order dan user.

## Create Operation
### Tambah User
Pertama ambil query mutation yang sebelumnya kita lakukan di GraphIQL. Kemudian buat function `createUserMutation` seperti ini di file `src/lib/query.js`:
```js
export const createUserMutation = (email, fullName) => gql`
    mutation {
        createUser(request: {
            email: "${email}",
            fullName: "${fullName}"
        }) {
            userId
            email
            fullName
        }
    }
`;
```

Buka file `src/lib/user.js` lalu tambahkan code ini:
```js
import Swal from "sweetalert2";
import { createUserMutation, getUsersWithOrders } from "./query";
import client from "$lib";

export const createUser = (email, fullName) => {

  return Swal.fire({
    icon: 'question',
    title: 'Kamu yakin?',
    text: `Kamu akan membuat user ${fullName} dengan email ${email}`,
    showCancelButton: true,
    confirmButtonText: 'Yoi, Tambah Aja!',
    cancelButtonText: 'Ga, Ga Jadi!',
    preConfirm: async () => {
      const query = createUserMutation(email, fullName);
      const res = await client.mutate({
        mutation: query,
        refetchQueries: [{ query: getUsersWithOrders }]
      });

      if (res.data) {
        return Swal.fire({
          icon: 'success',
          title: 'User berhasil dibuat',
          text: `User ${res.data.createUser.fullName} berhasil dibuat`,
          showConfirmButton: false,
          timer: 1500
        })
      } else {
        return Swal.fire({
          icon: 'error',
          title: `User ${fullName} gagal dibuat`,
          text: `Error: ${res.errors[0].message}`,
          showConfirmButton: false,
          timer: 1500
        })
      }
    }
  })
}
```

Lalu buat file dengan nama `ModalCreateEditUser.svelte` ini modal untuk create dan edit user dan isi begini:
```js
<script>
  import { createUser } from "$lib/user";

    const { data, icons="bi bi-cart-plus", color="primary", text="" } = $props();

    let formData = $state({
        email: "",
        fullName: "",
    })

    const submit = async (e) => {
        e.preventDefault();
        await createUser(formData.email, formData.fullName);
    }
</script>

<button type="button" class="btn btn-sm btn-{color}" data-bs-toggle="modal" data-bs-target="#exampleModal">
    <i class={icons}></i> {text}
</button>

<div class="modal fade" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
    <div class="modal-dialog">
        <div class="modal-content">
            <div class="modal-header">
                <h5 class="modal-title" id="exampleModalLabel">{text}</h5>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body">
                <form onsubmit={(e) => submit(e)}>
                    <div class="mb-3">
                        <label for="email" class="form-label">Email</label>
                        <input type="email" class="form-control" id="email" bind:value={formData.email} required>
                    </div>
                    <div class="mb-3">
                        <label for="fullName" class="form-label">Full Name</label>
                        <input type="text" class="form-control" id="fullName" bind:value={formData.fullName} required>
                    </div>
                    <div class="d-flex justify-content-end gap-3">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Ga Jadi</button>
                        <button type="submit" class="btn btn-primary">Gas</button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</div>
```
Kemudian buka file `src/routes/+page.svelte` lalu ganti combol tambah user menjadi ini:
```js
<ModalCreateEditUser data={null} text="Tambah User" /> // ganti tombol tambah user
<button class="btn btn-sm btn-success">
  <i class="bi bi-cart-plus"></i> Tambah Order
</button>
```

Terakhir kita coba di browser untuk menambahkan users baru

<div class="d-flex gap-3">
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/input-add-user.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/add-user.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />
</div>

Jika sudah berhasil maka nanti akan langsung auto refresh data dan terdapat data user baru dengan orders nya yang kosong. 

</img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/success-add-user.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />

### Tambah Order
Untuk menambah order mirip dengan tambah user, hanya saja kita perlu memilih user mana yang ingin menambah order.

Pertama buat buat function baru untuk query create order di `src/lib/query.js`:
```js
export const createOrderMutation = (orderName, userId, orderPrice) => {

  if (userId === 0 || !orderName || !orderPrice) {
    return Swal.fire({
      icon: 'error',
      title: 'Error',
      text: 'Email dan Nama harus diisi',
    });
  }

  return gql`
    mutation {
        createOrder(request: {
            orderName: "${orderName}",
            userId: ${userId},
            orderPrice: ${orderPrice},
        }) {
            orderId
            orderName
            userId
            orderPrice
            orderStatus
        }
    }
`;
};
```

Lalu buat function bari di `src/lib/order.js` untuk menambah order:
```js
import Swal from "sweetalert2";
import { createOrderMutation, getUsersWithOrders } from "./query";
import client from "$lib";

export const createOrder = (orderName, userId, orderPrice) => {

  return Swal.fire({
    icon: 'question',
    title: 'Kamu yakin?',
    text: `Kamu akan membuat order ${orderName} dengan harga ${orderPrice} untuk user ${userId}`,
    showCancelButton: true,
    confirmButtonText: 'Yoi, Tambah Aja!',
    cancelButtonText: 'Ga, Ga Jadi!',
    preConfirm: async () => {
      const query = await createOrderMutation(orderName, userId, orderPrice);
      const res = await client.mutate({
        mutation: query,
        refetchQueries: [{ query: getUsersWithOrders }]
      });

      if (res.data) {
        return Swal.fire({
          icon: 'success',
          title: 'Order berhasil dibuat',
          text: `Order ${res.data.createOrder.orderName} berhasil dibuat`,
          showConfirmButton: false,
          timer: 1500
        })
      } else {
        return Swal.fire({
          icon: 'error',
          title: `Order ${orderName} gagal dibuat`,
          text: `Error: ${res.errors[0].message}`,
          showConfirmButton: false,
          timer: 1500
        })
      }
    }
  })
}
```
Kemudian buat component baru di `src/components/ModalCreateEditOrder.svelte` untuk fprm tambah order:
```js
<script>
    import { createOrder } from "$lib/order";

    const { data, icons="bi bi-cart-plus", color="primary", text="", modalId } = $props();

    let formData = $state({
        orderName: "",
        userId: 0,
        orderPrice: ""
    })

    const submit = async (e) => {
        const price = parseInt(formData.orderPrice);
        e.preventDefault();
        await createOrder(formData.orderName, formData.userId, price);
    }
</script>

<button type="button" class="btn btn-sm btn-{color}" data-bs-toggle="modal" data-bs-target="#{modalId}">
    <i class={icons}></i> {text}
</button>

<div class="modal fade" id="{modalId}" tabindex="-1" aria-labelledby="{modalId}Label" aria-hidden="true">
    <div class="modal-dialog">
        <div class="modal-content">
            <div class="modal-header">
                <h5 class="modal-title" id="{modalId}Label">{text}</h5>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body">
                <form onsubmit={(e) => submit(e)}>
                    <div class="mb-3">
                        <label for="text" class="form-label">Order Name</label>
                        <input type="text" class="form-control" id="text" bind:value={formData.orderName} required>
                    </div>
                   <div class="mb-3">
                        <label for="userId" class="form-label">User</label>
                        <select required id="userId" class="form-control" bind:value={formData.userId}>
                            <option value="">Pilih User</option>
                            {#each data as user}
                                <option value={user.userId}>{user.fullName}</option>
                            {/each}
                        </select>
                   </div>
                    <div class="mb-3">
                        <label for="orderPrice" class="form-label">Price</label>
                        <input type="text" class="form-control" id="orderPrice" bind:value={formData.orderPrice} required>
                    </div>
                    <div class="d-flex justify-content-end gap-3">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Ga Jadi</button>
                        <button type="submit" class="btn btn-primary">Gas</button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</div>
```
Terakhir ganti tombol edit pada `src/routes/+page.svelte` jangan lupa untuk import:
```js
<ModalCreateEditOrder data={data} text="Tambah Order" color="success" modalId="order" />
```
Terakhri kita coba di browser untuk menambahkan order baru

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/input-add-order.png" alt="graphql-on-actix-web/assets/1.png" width="50%" />
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/add-order.png" alt="graphql-on-actix-web/assets/1.png" width="50%" />

Jika sudah berhasil maka nanti akan langsung auto refresh data dan terdapat data order untuk User 3. 

</img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/success-add-order.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />

## Update Operations

### Update User
Untuk update user mirip dengan tambah user, kita juga akan menggunakan `ModalCreateEditUser`, hanya saja kita perlu memilih user mana yang ingin di update.

Pertama buat function baru untuk query update user di `src/lib/query.js`:
```js
export const updateUserMutation = (userId, email, fullName) => {

  if (!userId || !email || !fullName) {
    return Swal.fire({
      icon: 'error',
      title: 'Error',
      text: 'UserId, Email dan Nama harus diisi',
    });
  }

  return gql`
    mutation {
        updateUser(request: {
            userId: ${userId}
            email: "${email}"
            fullName: "${fullName}"
        }) {
            userId
            email
            fullName
        }
    }
`;
};
```

Lalu buat function bari di `src/lib/user.js` untuk update user:
```js
export const updateUser = (userId, email, fullName) => {

  return Swal.fire({
    icon: 'question',
    title: 'Kamu yakin?',
    text: `Kamu akan mengupdate user ${fullName} dengan email ${email}`,
    showCancelButton: true,
    confirmButtonText: 'Yoi, Edit Aja!',
    cancelButtonText: 'Ga, Ga Jadi!',
    preConfirm: async () => {
      const query = updateUserMutation(userId, email, fullName);
      const res = await client.mutate({
        mutation: query,
        refetchQueries: [{ query: getUsersWithOrders }]
      });

      if (res.data) {
        return Swal.fire({
          icon: 'success',
          title: 'User berhasil diupdate',
          text: `User ${res.data.updateUser.fullName} berhasil diupdate`,
          showConfirmButton: false,
          timer: 1500
        })
      } else {
        return Swal.fire({
          icon: 'error',
          title: `User ${fullName} gagal diupdate`,
          text: `Error: ${res.errors[0].message}`,
          showConfirmButton: false,
          timer: 1500
        })
      }
    }
  })
}
```

Buka file `src/components/Table.svelte` Ganti tombol edit dengan ini:
```js
<ModalCreateEditUser data={user} icons="bi bi-pencil" color="success" modalId={`edit-user-${user.userId}`} />
```

Terakhir buka component `src/components/ModalCreateEditUser.svelte` tambahkan kode ini:
```js
onMount(() => {
    if(modalId?.includes("edit")) {
        formData.email = data.email;
        formData.fullName = data.fullName;
    }
});
```

Dan ubah function `submit` menjadi ini:
```js
const submit = async (e) => {
  e.preventDefault();

  if(modalId?.includes("edit")) {
      await updateUser(data.userId, formData.email, formData.fullName); // jangan lupa import updateUser
  } else {
      await createUser(formData.email, formData.fullName);
  }
}
```

Lalu kita coba di browser untuk update user

<div class="row">
<div class="col-md-6">
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/input-edit-user.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />
</div>
</div>

Ketika klik tombol edit maka form akan otomatis terisi dengan data user yang ingin di update. Kita akan mengubah domail email menjadi gmail.com.

<div class="d-flex gap-3">
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/update-edit-user.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/edit-user.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />
</div>

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/success-edit-user.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />

Data berhasil di update, data berada di bawah karena kita tidak memberikan urutan pada query untuk get datanya.

### Update Order
Untuk order harusnya ngga ada perbedaan jauh dengan update users, hanya saja kita perlu menambahkan field `orderStatus` pada form input. Karena sebelumnya kita set saat order di input statusnya `pending` jadi kita tidak perlu menambahkan field tersebut saat tambah order.

Pertama buat function baru untuk query update order di `src/lib/query.js`:
```js
export const updateOrderMutation = (orderId, userId, orderName, orderStatus, orderPrice) => {

  if (!orderId || !userId || !orderName || !orderStatus || !orderPrice) {
    return Swal.fire({
      icon: 'error',
      title: 'Error',
      text: 'UserId, Email dan Nama harus diisi',
    });
  }

  return gql`
    mutation {
        updateOrder(request: {
            orderId: ${orderId}
            userId: ${userId}
            orderName: "${orderName}"
            orderStatus: "${orderStatus}"
            orderPrice: ${orderPrice}
        }) {
            userId
            userId
            orderName
            orderStatus
            orderPrice
        }
    }
`;
};
```

Lalu buka file `src/components/ModalCreateEditOrder.svelte` tambahkan kode ini:
```js
$effect(() => {
    if(modalId.includes("edit")) {
        formData.orderName = data.orderName;
        formData.userId = data.userId;
        formData.orderPrice = data.orderPrice;
        formData.orderStatus = data.orderStatus;
    }
})
```
Dan ubah functiom `submit` menjadi ini:
```js
const submit = async (e) => {
    e.preventDefault();
    const price = parseInt(formData.orderPrice);

    if(modalId?.includes("edit")) {
        await updateOrder(data.orderId, formData.userId, formData.orderName, formData.orderStatus, price);
    } else {
        await createOrder(formData.orderName, formData.userId, price);
    }

}
```
Selain itu tambahkan props baru yaitu `users` untuk mengirim data list users yang akan digunakan di `select` user.

Tambahkan element baru yang di beri kondisi `if(modalId?.includes("edit"))` yaitu:
```js
{#if modalId?.includes("edit")}
<div class="mb-3">
    <label for="orderStatus" class="form-label">Status</label>
    <select required id="orderStatus" class="form-control" bind:value={formData.orderStatus}>
        <option value="">Pilih Order Status</option>
        <option value="pending">PENDING</option>
        <option value="settlement">SETTLEMENT</option>
        <option value="cancelled">CANCELLED</option>
    </select>
</div>
{/if}
```

Terakhir ubah tombol edit pada row order di `src/components/Table.svelte` menjadi ini:
```js
<ModalCreateEditOrder data={order} icons="bi bi-pencil" color="success" modalId={`edit-order-${order.orderId}`} users={data} />
```

Okeh, kemudian kita coba di browser.

<div class="d-flex gap-3">
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/input-edit-order.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/edit-order.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />
</div>

Jika sudah berhasil maka nanti akan langsung auto refresh data dan terdapat data order Beli Bebek dengan harga 19.000 dengan status settlement.

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/success-edit-order.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />

## Delete Operations
Untuk delete kita tidak memerlukan form input lagi karena kita hanya membutuhkan id untuk memberikan tanpa data mana yang akan di delete.

### Delete User
Untuk delete users, sebelumnya backend tidak gue contohkan kan? Yaiyalah buat experimen lo sendiri bro wkwkwk. Gue kasih info aja kalo delete user itu juga delete semua order yang terkait dengan user tersebut. Untuk di `real case` sebenernya menghapus data itu tidak dianjurkan, karena historynya akan hilang. Lebih baik memberikan flag aja dan jangan menghapus datanya.

Pertama buat function baru untuk query delete user di `src/lib/query.js`:
```js
export const deleteUserMutation = (userId) => {

  if (!userId) {
    return Swal.fire({
      icon: 'error',
      title: 'Error',
      text: 'UserId tidak ditemukan',
    });
  }

  return gql`
    mutation {
        deleteUser(userId: ${userId})
    }
`;
};
```

Lalu buat function di `src/lib/user.js` untuk delete user:
```js
export const deleteUser = (user) => {

  return Swal.fire({
    icon: 'question',
    title: 'Kamu yakin?',
    text: `Kamu akan menghapus user ${user.fullName} dengan email ${user.email}`,
    showCancelButton: true,
    confirmButtonText: 'Yoi, Hapus Aja!',
    cancelButtonText: 'Ga, Ga Jadi!',
    preConfirm: async () => {
      const query = deleteUserMutation(userId);
      const res = await client.mutate({
        mutation: query,
        refetchQueries: [{ query: getUsersWithOrders }]
      });

      if (res.data) {
        return Swal.fire({
          icon: 'success',
          title: 'User berhasil dihapus',
          text: `User ${fullName} berhasil dihapus`,
          showConfirmButton: false,
          timer: 1500
        })
      } else {
        return Swal.fire({
          icon: 'error',
          title: `User ${fullName} gagal dihapus`,
          text: `Error: ${res.errors[0].message}`,
          showConfirmButton: false,
          timer: 1500
        })
      }
    }
  })
}
```
Terakhir panggil function deleteUser di `src/components/Table.svelte`:
```js
<button class="btn btn-sm btn-danger" aria-label="Delete" onclick={() => deleteUser(user)}>
  <i class="bi bi-trash"></i>
</button>
```

Kalau udah kita coba di browser.

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/hapus-user.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/success-hapus-user.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />

### Delete Order
Untuk delete order cukup mudah, karena kita tinggal delete data sesuai id mana. Dan sebelumnya udah di coba di GraphIQL kan. 

Pertama biasa kita buat function baru untuk query delete order di `src/lib/query.js`:
```js
export const deleteOrderMutation = (orderId) => {

  if (!orderId) {
    return Swal.fire({
      icon: 'error',
      title: 'Error',
      text: 'OrderId tidak ditemukan',
    });
  }

  return gql`
    mutation {
        deleteOrder(orderId: ${orderId})
    }
`;
};
```

Lalu buat function di `src/lib/order.js` untuk delete order:
```js
export const deleteOrder = (order) => {

  return Swal.fire({
    icon: 'question',
    title: 'Kamu yakin?',
    text: `Kamu akan menghapus order ${order.orderName} dengan harga ${order.orderPrice} untuk user ${order.userId}`,
    showCancelButton: true,
    confirmButtonText: 'Yoi, Hapus Aja!',
    cancelButtonText: 'Ga, Ga Jadi!',
    preConfirm: async () => {
      const query = deleteOrderMutation(order.orderId);
      const res = await client.mutate({
        mutation: query,
        refetchQueries: [{ query: getUsersWithOrders }]
      });

      if (res.errors) {
        return Swal.fire({
          icon: 'success',
          title: 'Order berhasil dihapus',
          text: `Order ${order.orderName} berhasil dihapus`,
          showConfirmButton: false,
          timer: 1500
        })
      } else {
        return Swal.fire({
          icon: 'error',
          title: `Order ${order.orderName} gagal dihapus`,
          text: `Error: ${res.errors[0].message}`,
          showConfirmButton: false,
          timer: 1500
        })
      }
    }
  })
}
```
Terakhir panggil function deleteOrder di `src/components/Table.svelte`:
```js
<button class="btn btn-sm btn-danger" aria-label="Delete" onclick={() => deleteOrder(order)}>
  <i class="bi bi-trash"></i>
</button>
```

Kalau udah kita coba di browser.

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/hapus-order.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/graphql-on-actix-web/assets/success-hapus-order.png" alt="graphql-on-actix-web/assets/1.png" width="100%" />

Udah catatan nya sampe sini dulu yak bro biar kaga kepanjangan. Thank you bro udah baca👍

---

<div class="d-flex flex-row justify-content-center align-items-center">Regards <a href="https://feri-irawansyah.my.id"><img witdh="1rem" src="https://feri-irawansyah.my.id/favicon.ico" alt="Feri Irawansyah"> Feri Irawansyah</a></div>

---
