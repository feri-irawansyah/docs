---

### Tech Stack Reason
Aplikasi REST Full API ini bisa di bilang global api yang digunakan di beberapa project pribadi gue seperti Onboarding Customer, Export Import File, dll.

#### Rust + Actix Web
Rust merupakan bahasa pemrograman Favorit gue udah banyak gue pake dibeberapa kasus terkadang gue juga bikin aplikasi CLI untuk membantu gue dikantor. Jadi untuk web api pribadi gue juga pake Rust karena untuk framework lain seperti Laravel, Springboot, Dotnet, Django, FastApi, NestJs dll gue kurang menguasai mungkin yang lebih dekat Dotnet karena dipake dikerjaan.

#### Shuttle 
Nah selain karena gue suka pake Rust, ada juga cloud platform yang menyediakan layanan gratis always untuk deploy Rust App Web Framework yaitu Shuttle. Jadi dari pada gue ribet - ribet dan efort keluarin dompet mendingan cari alternatif gratisan.

#### Json Web Token (JWT)
Karena api ini dipake di beberapa aplikasi, jadi gue perlu membuat otentikasi dan otorisasi yang berbeda disetiap applikasi. Jadi beberapa aplikasi punya fitur Authentication dan diakses bersamaan oleh satu orang session bisa dimanage dengan baik dan tidak akan bentrok.

#### Swagger Open API
Backend Dev/Engineer yang baik adalah seorang backend yang mau membuat dokumentasi API nya, tidak perlu jauh - jauh ke frontend dev atau ke orang lain dulu, ke disi sendiri dulu aja dengan adanya docs api akan lebih mempermudah dalam penggunaan nya.

#### PostgreSQL
PostgreSQL itu udah kaya asisten yang siap nyimpen berbas dan data - data yang kita perlukan. Gue pribadi memang paling suka pake PostgreSQL karena fiturnya banyak, ga nguras dompet, selain itu untuk indexing fulltext search menurut gue postgre ini paling bagus.

### Chalange

#### Shuttle Minim Resource
Karena gratisan jadi cuma sedikit space server / container yang didapatkan Shuttle hanya menyediakan 0.25GB untuk free tiernya. Jadi meskipun pake Rust yang punya management memory, tapi optimasi juga perlu biar api tetep bagus performanya seperti:
- Tidak `SELECT *` ke semua tabel, tapi khusus kolom - kolom yang dibutuhkan aja
- Penggunakan `COUNT(nama_colom)` tidak `COUNT(*)`
- Membuat pagination agar tidak terlalu banyak data yang di load
- Banyak penggunaan `Reference` dan `Pointer` pada Rust, karena jika terlalu sering pake `clone()` akan menambah beban meskipun tidak signifikan.
- Menggunakan `pooling connection` dan meminimalkan penggunaan `global state` atau `Arc<Mutex>` agar data tidak selalu ada di memory.

#### Swagger Harus Bikin Manual
Swagger ini kalo di Rust masih lumayan manual bro bikin nya. Karena harus bikin endpoint sendiri. Jadi setiap endpoint misal:
```rust
    #[utoipa::path(
        responses(
            (status = 200, description = "Hello from api 1", body = str)
        )
    )]
    #[get("/hello")]
    pub(super) async fn hello1() -> &'static str {
        "hello from api 1"
    }
```
Jadi harus define diatas function handlernya. Atau kalo mau lebih clean bisa bikin file terpisah dan bikin dummy function. Tapi itu lumayan efort karena harus memasukan object yang dipake dihandler function aslinya.

#### Custom Header Http
Karena Shuttle ini berbasis docker file jadi untuk http header dari server sudah mereka custom sendiri dan misal gue mau custom sendiri hanya bisa dari sisi Actix Web nya bukan dari Web Server yang disediakan Shuttle. Tapi Shuttle menjanjika untuk Encryption header sudah aman dan optimal, jadi ga perlu khawatir jadi gue hanya mikir optimasi dari sisi Rust dan Actix Web nya aja.

---

<div class="d-flex flex-row justify-content-center align-items-center">Regards <a href="https://feri-irawansyah.my.id"><img witdh="1rem" src="https://feri-irawansyah.my.id/favicon.ico" alt="Feri Irawansyah"> Feri Irawansyah</a></div>

---
