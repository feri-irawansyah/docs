Pada catatan kali ini, kita akan mempelajari bagaimana menggunakan Rust untuk mengakses database Microsoft SQL Server. Berikut adalah tools yang digunakan:

- [Rust](https://www.rust-lang.org/)
- [Microsoft SQL Server](https://www.microsoft.com/en-us/sql-server)
- [Tokio](https://tokio.rs/)
- [Tiberius](https://docs.rs/tiberius/latest/tiberius/)

## Create a new project

Kita akan membuat project baru menggunakan `cargo new rust_sqlserver` untuk memulai. Lalu kita akan menambahkan `Cargo.toml` dan `main.rs` seperti berikut:

```bash
$ cargo new rust_sqlserver
$ cd rust_sqlserver
```
Kemudian buka dengan menggunakan code editor, disini saya menggunakan Visual Studio Code. Secara default, kita akan dibuatkan project baru seperti berikut:

```bash
.
├── Cargo.toml
├── .gitignore
├── src
│   └── main.rs
└── README.md
```

#### Isi file `main.rs`
```rust
fn main() {
    println!("Hello, world!");
}
```
#### Isi file `Cargo.toml`
```toml
[package]
name = "rust_sqlserver"
version = "0.1.0"
edition = "2021"

[dependencies]
```
## Preparation
Sebelum menggunakan Rust untuk mengakses database Microsoft SQL Server, kita perlu melakukan beberapa persiapan terlebih dahulu:

- Menginstal Tokio
- Menginstal Tiberius
- Mengubah `main.rs` menjadi `lib.rs` karena kita akan menjalankan program saja (tidak membuat aplikasi executable)

### Install Dependency

Tambahkan dependency seperti berikut pada file `Cargo.toml`:
```toml
tiberius = "0.12.3"
tokio = { version = "1.45.0",features = ["net", "macros", "rt-multi-thread"] }
tokio-util = {version = "0.7.15",features = ["compat"]}
```

### Change main.rs to lib.rs
```bash
mv main.rs lib.rs
```
Atau bisa langsung rename file `main.rs` menjadi `lib.rs` pada code editor.

## Run the program

Untuk menjalankan program, kita bisa menggunakan perintah `cargo test --exact nama_function --show-output` untuk menampilkan output dengan unit test. 

### Get Connection Host and Port
- Untuk terkoneksi ke database SQL Server jika login menggunakan Windows Authentication biasanya jalan di host `localhost` atau `127.0.0.1` dan port `1433`. Namun pada saat installasi bisa terjadi perubahan portnya.
- Kita perlu mencari lokasi host dan port dimana SQL Server berjalan dengan perintah berikut:

Jika menggunakan SQL Server Management Studio:
```bash
EXEC xp_readerrorlog 0, 1, N'Server is listening on';
```
Jika menggunakan Command Prompt:
```bash
sqlcmd -S localhost\SQLEXPRESS -E -Q "EXEC xp_readerrorlog 0, 1, N'Server is listening on';"
```
| LogDate | ProcessInfo | Text |
| -------- | -------- | -------- |
| 2025-05-18 08:14:45.670   | Server   | Server is listening on [ ::1 <ipv6> 1434] accept sockets 1.   |
| 2025-05-18 08:14:45.680   | Server   | Server is listening on [ 127.0.0.1 <ipv4> 1434] accept sockets 1. |

Di local komputer saya SQL Server berjalan di host `127.0.0.1` dan port `1434`.

## Let's Code it!
Setelah persiapan selesai, kita bisa menulis kode untuk terkoneksi ke database SQL Server seperti berikut:

### Connect ke SQL Server dengan Windows Authentication `host` and `post`
```rust
pub async fn connect_with_host_port() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();

    // Windows Authentication (SSPI)
    config.authentication(AuthMethod::Integrated);

    // Ganti ini sesuai alamat & port servermu
    config.host("127.0.0.1");
    config.port(1434); // atau port hasil dari konfigurasi
    config.trust_cert(); // opsional jika SSL tidak disertifikasi

    let tcp = TcpStream::connect(config.get_addr()).await?; // koneksi ke server
    let client = Client::connect(config, tcp.compat_write()).await?; 
    println!("Connected with Windows Authentication!");
    client.close().await?;

    Ok(())
}

// Test the function
#[tokio::test]
async fn connect_to_sql_server_using_host_port() {
    let result = connect_with_host_port().await;
    assert_eq!(result.is_ok(), true);
}
```
Setelah itu, kita bisa menjalankan program dengan perintah `cargo test --exact connect_to_sql_server_using_host_port --show-output` untuk menampilkan output.
```bash
$ cargo test --exact connect_to_sql_server_using_host_port --show-output
...
........

running 1 test
test connect_to_sql_server_using_host_port ... ok

successes:

---- connect_to_sql_server_using_host_port stdout ----
Connected with Windows Authentication!


successes:
    connect_to_sql_server_using_host_port

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

   Doc-tests rust_sqlserver

running 0 tests

successes:

successes:

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Hasil output `test result: ok. 1 passed;` artinya kita berhasil terkoneksi ke database SQL Server menggunakan Windows Authentication.

---

## Agenda Pada Catatan Ini
> - Connect SQL Server menggunakan Username dan Password.

> - Connect SQL Server menggunakan Connection String.

> - CRUD pada table.

> - CRUD dengan stored procedure.

> - CRUD dengan function.

### Connect ke SQL Server dengan SQL Server Authentication `host` and `post`
Sebelumnya kita telah terkoneksi ke database SQL Server menggunakan Windows Authentication. Sekarang kita akan mempelajari bagaimana menggunakan Username dan Password Authentication.

Tambahkan function connect_with_host_port_username_password di file `lib.rs`.

```rust
// Connect with SQL Server Authentication
pub async fn connect_with_host_port_username_password() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();

    // Use SQL Server Authentication (user name and password)
    config.authentication(AuthMethod::sql_server("sa", "Snakesystem"));

    config.host("127.0.0.1");
    config.port(1434);
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    let client = Client::connect(config, tcp.compat_write()).await?;
    println!("Connected to SQL Server");
    let _ = client.close().await?;

    Ok(())
}
```

Kemudian buat test function di bawahnya seperti ini di file `lib.rs`:

```rust
#[tokio::test]
async fn connect_to_sql_server_using_host_port_username_password() {
    let result = connect_with_host_port_username_password().await;
    assert_eq!(result.is_ok(), true);
}
```
Dan jalankan program dengan perintah `cargo t connect_to_sql_server_using_host_port_username_password`

```bash
$ cargo t connect_to_sql_server_using_host_port_username_password
...
running 1 test
test connect_to_sql_server_using_host_port_username_password ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.04s
```

Hasil output `test result: ok. 1 passed;` artinya kita berhasil terkoneksi ke database SQL Server menggunakan Username dan Password.

---
