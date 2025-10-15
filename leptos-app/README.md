Dulu gue kalo mau bikin UI yang interactive di web pilihannya cuma 2 kalo ga React ya Svelte. React punya komunitas yang gede mau cari apa aja ada sedangkan Svelte memberikan kenyamanan dan simple. Tapi sejak tahun 2024 awal Leptos V 0.6 akhirnya rilis versi stabilnya. 
Akhirnya gue coba baca - baca dokumentasinya dan coba bikin project menggunakan Leptos. Nah hasilnya adalah website portfolio gue ini yang selsai pada bulan Juni 2024. 

Pada catatan kali ini gue mau berbagi tentang dasar - dasar Leptos dari yang gue pelajari dan sesuai dengan dokumentasi reminya.

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

<details>
<summary><h2>📌 Get Started Leptos</h2></summary>

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

### Tentang `mount_to_body`

Kita balik lagi ke `main.rs` lalu isikan code ini:

```rust
use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
```

Maksudnya apa? `mount_to_body` itu sama aja kaya di react kaya gini:

```jsx
    ReactDOM.createRoot(document.getElementById('root')).render(
      <p>Hello, world!</p>
    );
```

Jadi Trunk akan membuat sebuah element di body html yang mana element tersebut adalah tag `<p><\p>`. pada `mount_to_body` inilah aplikasi Lo dibuat nantinya bro. 

`mount_to_body` ini menerima parameter berupa `closure` biasanya `callback` atau `anonymous` function kalo javascript atau beberapa bahasa pemrograman lainnya. Dan return dari closure berupa `view! {}`.

```rust
view! {
   // Element atau type data
}
```

`view!` ini adalah `macro` atau syntax magic di rust leptos yang untuk melakukan render berupa `element html` atau tipe data tertentu seperti `String`, `integer`, `boolean`, `array`, `object`, dan lain - lain.

```rust
view! {
   "Hello, world!" 
}
```

Kalo Lo pake VS Code buat folder `.vscode` di root project dan buat file `settings.json` lalu isikan code ini:

```json
{
    "rust-analyzer.procMacro.ignored": {
        "leptos_macro": [
            "component",
        ],
    },

    "emmet.includeLanguages": {
        "rust": "html"
    }

}
```

Configurasi ini untuk ngasih tau si VS Code agar macro `component` tidak di anggap error sama `rust-analyzer` dan ngasih tau `emmet` bahwa `rust` adalah bahasa pemrograman html jadi Lo bisa mengetikkan shortcut untuk membuat tag html. 

Sekarang kita balik lagi ke `main.rs` lalu isikan code ini:

```rust
fn main() {
    leptos::mount::mount_to_body(|| view! {
        <h1>"Hello Leptos"</h1>
        <header>"Header"</header>
        <p>"Welcome to Leptos!"</p>
        <b>Nama: Satria</b>
        <span>Usia : 20 tahun</span>
        <small>Status: Jomblo</small>
        <footer>"Footer"</footer>
    });
}
```

Pada macro `view!` Lo bisa menuliskan tag html apapun. Dan `view!` bisa merender multiple html tidak seperti `jsx in React` yang wajib hanya merender satu tag html saja. Dan kalo Lo inspect/buka devtools di browser, struktur htmlnya bakal sama kaya yang Lo tulis di `main.rs`.

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/leptos-app/assets/inspect.png" class="img-fluid" alt="Hello Leptos 3"/>

### Aturan pada macro `view!`

Tapi tetap aja ada aturan di macro `view!` yaitu jadi Lo ga bisa juga asal sembarangan nulisin code atau element html:

#### Satu node html

Meskipun di Leptos bisa tanpa satu node html, tapi alangkah lebih baik menggunakan satu node sebagai pembungkus, atau menggunakan `<></>` jika tidak memerlukan tag html.

#### Semua expression di html harus pake `{}`

Misal Lo pingin parse atau render data dari variable, itu Lo wajib menggunakan expresiion `{}`.

```rust
let name = "Satria";
view! {
    <h1>{name}</h1>
}
```

#### Text literal di-quote `("text")`

Jadi di Leptos sebaiknya jika menuliskan text di tag html mengunakan quote `"text"`.

```rust
<p>"Halo dunia"</p> // aman✅
<p>Halo dunia</p> // aman✅
<p>"Jum'at"</p> // aman✅
<p>Jum'at</p> // tidak aman❌ lebih baik pake quote "Jum'at"
```

#### Self-closing tag wajib pakai /

```rust
<input type="text" /> // aman✅

<input type="text"> // tidak aman❌ harus pake /
```

#### Loop & kondisi pakai komponen built-in (For, Show, Transition, dll)

Karena view! di-expand compile-time, kamu nggak bisa pakai if atau for langsung di markup.
```rust
// tidak aman❌
let show = true;
view! {
    if show {
        <p>"Tampil"</p>
    } else {
        <p>"Sembunyi"</p>
    }
}

// aman✅
view! {
    <Show when=move || show fallback=|| view! { <p>"Sembunyi"</p> }>
        <p>"Tampil"</p>
    </Show>
}

// aman✅
view! {
    {
        if show {
            view! { <p>"Tampil"</p> }
        } else {
            view! { <p>"Sembunyi"</p> }
        }
    }
}

// aman ✅
let items = vec![];

view! {
    <ul>
        <For
            each=move || items
            key=|item| item.id
            children=move |item| view! { <li>{item.name}</li> }
        />
    </ul>
}

```

#### Semua variable yang dipakai di view! harus 'static atau move

Karena macro ini akan capture closure, jadi kalau kamu pakai signal, event handler, atau variabel luar, biasanya harus:

```rust
view! {
    <button on:click=move |_| log::info!("Clicked!")>
        "Klik Saya"
    </button>
}
```

#### Semua syntax dicek di compile-time, bukan runtime

Nah ini penting bro, Lo bikin UI pake rust dimana Lo ga bisa sembarang nulis code, karna kalo Lo salah dikit aplikasi Lo kaga bakal jalan beda sama JS yang penting jalan tapi soal bug dan error belakangan.

</details>


<details>
<summary><h2>📌 Component dan Props</h2></summary>

Hampir semua frontend Library dan Framework modern sekarang semuanya menggunakan arsitektur component based dimana semua UI adalah kepingan - kepingan Leptos juga termasuk salah satunya. Untuk component di Leptos sama seperti Component di React, Solid dan Qwik artinya component berupa function. Bedanya di Rust perlu beberapa type dan macro:

```rust
use leptos::prelude::*;

#[component]
fn MyComponent() -> impl IntoView {
    view! {
      <p>Hello world</>
    }
}
```

`#[component]` ini adalah macro dari Leptos untuk menandai kalo function rust itu adalah component

`IntoView` adalah struct untuk type wajib di component. Dibelakang layar `IntoView` ini berisi object element html dan atribut html. Artinya function component akan mereturn html.

Karena into View akan mengembalikan html jadi bisa juga menerima macro `view!`. Untuk memanggil component sama seperti di jsx yaitu `<MyComponent/>`

Sekarang kita praktekkan ke aplikasi kita dm biar lebih rapi kita bikin file baru di `src/app.rs` isinya untuk main application atau component parent.

```rust
// src/app.rs
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <main>
        App
      </main>
    }
}


// src/main.rs
use leptos::prelude::*;

mod app;

use crate::app::App;

fn main() {
    leptos::mount::mount_to_body(|| <App/>)
}

```
</details>
