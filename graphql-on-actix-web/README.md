Biasanya gue kalo bikin api pake Actix Web itu response JSON nya segelondongan langsung di kirim, dan biasanya perlu beberapa endpoint untuk mengambil data berbeda.
Selain itu kadang beberapa field kaga dibutuhin suka kebawa dan adalagi misal butuh data dari sumber lain harus request lagi juga. Jadi gue mau nyobain GraphQL yang lebih ringkas, bisa mangkas data, dan hemat bandwidth katanya.

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

Kalo udah buka di code editor yang lu pake bang, kalo gue bisanya pake code editor 1 miliar umat (VS Code).

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

Kalo versi rustnya sama kaya gue di 1.87.0, harusnya aman tidak terjadi error. Kalo misal ada paling harus install sesuai versi rust yang lu pake bang.

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
......
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
Buka file `src/main.rs` lalu tambahkan:























Install `@apollo/client` dan `graphql` di frontend.

app/index.js:
```js
// src/lib/apolloClient.js
import { ApolloClient, InMemoryCache, HttpLink, gql } from '@apollo/client/core';

const client = new ApolloClient({
  link: new HttpLink({ uri: 'http://localhost:8080/query' }),
  cache: new InMemoryCache(),
});

export const USER = gql`
    query GetUser($id: Int!) {
      user(userId: $id) {
        userId
        email
        handphone
        registerDate
        disableLogin
      }
    }
  `;

export default client;
```

Code dasar:
```ts
<script>
  import client, { USER } from '../app';

  let userPromise = client.query({
    query: USER,
    variables: { id: 1 },
    fetchPolicy: 'network-only'
  }).then(res => res.data.user);

  function refetch() {
    userPromise = client.query({
      query: USER,
      variables: { id: 1 },
      fetchPolicy: 'network-only'
    }).then(res => res.data.user);
  }
</script>

<button on:click={refetch}>Refetch User (Fresh)</button>

{#await userPromise}
  <p>Loading...</p>
{:then user}
  <p>User ID: {user.userId}</p>
  <p>Email: {user.email}</p>
  <p>Handphone: {user.handphone}</p>
  <p>Register Date: {user.registerDate}</p>
  <p>Disable Login: {user.disableLogin ? 'Yes' : 'No'}</p>
{:catch error}
  <p>Error: {error.message}</p>
{/await}
```

### GraphQL Query

Method `GET`:
```js
query GetUser {
  user(userId: 1) {
    userId
    email
    handphone
    registerDate
  }
}
```

Method `POST`:
```js
mutation {
  createUser(input: {
    email: "abc@test.com"
    handphone: "08123456789"
    password: "rahasia"
  }) {
    userId
    email
  }
}
```

Method `PUT`:
```js
mutation {
  updateUser(id: 1, input: {
    email: "newemail@test.com"
    handphone: "08987654321"
  }) {
    userId
    email
  }
}
```

Method `DELETE`:
```js
mutation {
  deleteUser(id: 1) {
    userId
    email
  }
}
```