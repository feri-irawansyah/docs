Kalo Lo ga sengaja nemu catatan ini lebih baik Lo jangan terlalu dalam atau terlalu dihayati. Catatan ini khusus gue buat untuk Lo yang kuat lahir dan batin. Karena dari depan dan belakang semuanya pake Rust. Ya Rust itu dosen kiler bro yang kalo Lo salah dikit langsung di coret - coret sama Compiler dan Lo harus siap di maki - maki sama compiler Rust.

`Leptos` is a fine grained, reactive, full-stack web framework for building fast and interactive web applications in Rust. It leverages fine-grained reactivity to offer a highly efficient and modern development experience, drawing inspiration from frameworks like `SolidJS`, and `Sycamore`.

Leptos itu reactive framework terinspirasi dari `SolidJS` katanya atau bisa di bilang Leptos itu Solid JS nya Rust. Jadi Leptos ini berjalan di Client atau sebagai User Interface (UI). Karena ditulis dengan Rust (Low Level Programming Language) Leptos ini akan di compile menjadi `Wasm` (Web Assembly) agar bisa berjalan di browser. Untuk lebih detailnya Lo bisa baca artikel gue tentang Wasm dan JS <a href="https://feri-irawansyah.my.id/catatan/frontend/antara-lo-web-assembly-dan-javascript-buat-frontend-web-application" target="_blank" rel="noopener noreferrer">di sini</a>.

<details open>
<summary><h2>📌 Dokumentasi Leptos</h2></summary>

Sebelum mulai bikin app ada beberapa tempat keramat yang bisa Lo kunjungi bro.

- <a href="https://leptos.dev/" target="_blank" rel="noopener noreferrer">https://leptos.dev</a> Dokumentasi Leptos
- <a href="https://github.com/leptos-rs/leptos" target="_blank" rel="noopener noreferrer">https://github.comleptos-rs/leptos</a> Repository Leptos
- <a href="https://book.leptos.dev" target="_blank" rel="noopener noreferrer">https://book.leptos.dev</a> Buku Keramat Leptos
- <a href="https://github.com/leptos-rs/awesome-leptos/" target="_blank" rel="noopener noreferrer">https://github.com/leptos-rs/awesome-leptos</a> Awesome Leptos (Template Aplikasi Leptos)
- <a href="https://github.com/leptos-rs/" target="_blank" rel="noopener noreferrer">https://github.com/leptos-rs</a> Komunitas Open Source Leptos

Selain Lo perlu tau tempat - tempat keramat itu Lo juga perlu siapin sesajen (pre requisites) untuk memulai bikin aplikasi Leptos.

### Pre Requisites CSR

- <a href="https://rust-lang.org/" target="_blank" rel="noopener noreferrer">https://rust-lang.org</a> Rust
- `wasm32-unknown-unknown` Target
- <a href="https://trunkrs.dev/" target="_blank" rel="noopener noreferrer">https://trunkrs.dev</a> Trunk

### Pre Requisites SSR

- <a href="https://rust-lang.org/" target="_blank" rel="noopener noreferrer">https://rust-lang.org</a> Rust
- `wasm32-unknown-unknown` Target
- <a href="https://github.com/leptos-rs/cargo-leptos/" target="_blank" rel="noopener noreferrer">https://github.com/leptos-rs/cargo-leptos</a> `cargo-leptos`
- <a href="https://github.com/rustwasm/wasm-bindgen/" target="_blank" rel="noopener noreferrer">https://github.com/rustwasm/wasm-bindgen</a> Wasm Bindgen
- <a href="https://github.com/leptos-rs/cargo-leptos/" target="_blank" rel="noopener noreferrer">https://github.com/leptos-rs/cargo-leptos</a> Cargo Leptos

Banyakan SSR ya sesajen-nya? Iya bro krna Leptos SSR perlu http server dan di catatan ini V0.6 baru suport <a href="https://actix.rs" target="_blank" rel="noopener noreferrer">Actix Web</a> dan <a href="https://github.com/tokio-rs/axum" target="_blank" rel="noopener noreferrer">Axum</a>.

</details>

<details open>
<summary><h2>📌 Leptos CSR (Client Side Rendering)</h2></summary>

Sebenarnya ada banyak cara untuk membuat CSR Leptos, namun cara paling mudah, simple dan rapi kita bisa buat pake <a href="https://trunkrs.dev/" target="_blank" rel="noopener noreferrer">Trunk</a> ini mirip <a href="https://vitejs.dev/" target="_blank" rel="noopener noreferrer">Vite</a> kalau di Javascript tapi Rust punya.

### Installation Setup

Okeh gue anggep Lo udah punya Rust di komputer lu, jadi yang Lo kakuin jalankan perintah ini di cmd/terminal lu:
```bash
$ rustup target add wasm32-unknown-unknown
```

Ini untuk memasang target Wasm di komputer Lu. jadi nanti Rust bakal compile code Lo ke wasm.

Kalo udah sekarang Lo perlu install Trunk pake perintah ini:
```bash
$ cargo install trunk
```

Tunggu dulu karena Rust harus compile terlebih dahulu dan memerlukan waktu yang lama untuk di compile.

### Membuat Aplikasi Leptos CSR

Kita ikutin aja langkah - langkah yang ada di dokumentasi Leptos cpba Lo pergi ke halaman ini <a href="https://book.leptos.dev/getting_started/index.html" target="_blank" rel="noopener noreferrer">https://book.leptos.dev/getting_started/index.html</a>

```bash
$ cargo init leptos-csr

    Creating binary (application) package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

$ cd leptos-csr

$ ls
Cargo.toml  src/ .gitignore
```

Kemudian ketikkan perintah ini di folder `leptos-csr` untuk menginstal package Leptos:
```bash
$ cargo add leptos --features=csr
```

Jika success nanti Cargo akan membuatkan kita project baru di folder `leptos-csr` dimana ada File src/main.rs dan Cargo.toml. Coba Lo buka projectnya di code editor favorit Lo gue pake VS Code. Tetap di posisi terminal sebelumnya lalu ketik perintah `code .` nanti akan terbuka projectnya di VS Code.

Langkah selanjutnya buat file index.html root project kalo Lo pake VS Code ketik tanda seru `(!) + enter/tab` nanti akan di buatkan strucktur html sama VS Code. 

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/leptos-app/assets/index-html.png" class="img-fluid" alt="Leptos CSR"/>

Lalu buka `main.rs` di folder `src` lalu isikan code ini:

```rust
use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
```

Kemudian ketik perintah ini di terminal:
```bash
$ trunk serve
```

Tunggu sampai code `rust` di kompilasi dulu kalo sudah nanti akan terbentuk folder baru dengan nama `dist` didalamnya ada:

```bash
dist
├── index.html
├── leptos-csr-12f7940d90c3b1ac_bg.wasm # nama file bisa beda
└── leptos-csr-12f7940d90c3b1ac.js # nama file bisa beda
```

Dan di terminal Lo akan ada info seperti ini:

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/leptos-app/assets/hello-leptos.png" class="img-fluid" alt="Hello Leptos"/>

Coba lo pergi ke alamat ini <a href="http://localhost:8080/" target="_blank" rel="noopener noreferrer">http://localhost:8080</a> atau Lo tahan tombol Shift + Arahkan cursor ke alamat ini <a href="http://localhost:8080/" target="_blank" rel="noopener noreferrer">http://localhost:8080</a> di terminal dan click nanti akan langsung dibuka di browser default Lo.

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/leptos-app/assets/hello-leptos-2.png" class="img-fluid" alt="Hello Leptos 2"/>

#### Summary

Nah mungkin sampe sini banyak muncul pertanyaan dihati Lo kalo misalnya Lo itu anak Javascript.
1. Tadi buat index html kok kita ga bikin element yang ada id `root`/`app`?
2. Kalo ga ada id `root`/`app` terus kenapa ga ada juga tag `<script></script>` di index html?
3. Kok bisa langsung ada port 8080 dan bisa buka file `index.html` di browser apa pake live server?

Tapi sayangnya kita ga pake Javascript bro kita pake wasm. Jadi konsepnya ga kaya gitu kita ga pake tag `script lalu src="index.js"` atau pake element yang ada `id="root"`. Tapi untuk menghubungkan html dengan wasm itu udah dilakukan sama `Trunk`. Kalo Lo kepo sama Trunk Lo bisa baca artikel gue yang ini <a href="https://feri-irawansyah.my.id/catatan/frontend/bekerja-dengan-trunk-buat-frontend-web-application" target="_blank" rel="noopener noreferrer">Bekerja Dengan Trunk Buat Frontend Web Application</a>. 

Kita balik lagi ke `main.rs` lalu isikan code ini:

```rust
use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
```

Maksudnya apa?
</details>
