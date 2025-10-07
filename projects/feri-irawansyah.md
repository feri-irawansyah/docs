---
### Tech Stack Reason

Web Portfolio ini gue buat dengan kombinasi beberapa Teknologi sesuai dengan kebutuhan, favorit dan yang gue bisa aja😁.

#### Leptos (Fullstack Rust Framework)

Kenapa Leptos karena Rust itu bahasa pemrograman favorit gue dari awal masuk dunia teknologi dan Rust itu selalu mengingatkan gue tentang kesalahan - kesalahan dan bug yang lebih kompleks jadi gue ga bisa sembarangan nulis code. Selain itu gue deploy Website ini di VPS dengan ram dan cpu 1GB kurang malah. Jadi gue perlu tools yang bisa jalan di kondisi server yang kentang dengan singgle source & bahasa pemrograman yang sama untuk Frontend dan Backend nya.

#### Actix Web

Di dunia Rust dev untuk Http sebenarnya ada banyak Framework. Terutama Axum yang sangat populer akhir - akhir ini di tahun 2025 karena dengan async modern dan disuport langsung sama tim tokio.
Sebenarnya framework web di Rust pertama yang gue pake itu `Rocket` tapi setelah tahun 2022 gue pindah ke Actix Web karena gue rasa lebih ringan dan untuk async nya lebih modern. Selain itu diwebsite ini ada fitur <a href="https://feri-irawansyah.my.id/coffee-room">Chating<a/> disini gue pake `Actor Model` dari Actix yang bisa menghandle setiap pengunjung yang sedang online jadi pengunjung akan memiliki asistem virtual pribadi jika actif dalam chat.

#### Database PostgreSQL

PostgreSQL itu free dan open source, selain itu fiturnya banyak. Seperti untuk menampilkan Portfolio yang punya Tech Stack dihalaman <a href="https://feri-irawansyah.my.id/portfolio">Portfolio</a> itu gue pake 

```sql
SELECT kolom_di_portfolio_table
COALESCE(
    (
        SELECT json_agg(json_build_object('judul', s.kolom_judul, 'id', s.kolom_id, 'img', s.kolom_gambar))
        FROM tabel_tech_stack s
        WHERE s.kolom_id = ANY(p.kolom_tech)
    ),
    '[]'::json
) AS tech
```
Dimana design table untuk portfolio kolom tech stack itu type `array[integer]` dan bisa buat ambil `id` dari table teck stack dan result langsung dalam bentuk array berisi object ke Rust. Namun query seperti itu bagus hanya untuk satu record dari table parent (portfolio) jika record banyak maka akan berat karena menimbulkan N+1.

#### Redis

Kenapa gue pake Redis karena gue pingin bikin app yang punya performa tinggi meskipun resource terbatas, karena web portfolio tidak setiap menit bahkan detik adanya perubahan data. Selain itu gue juga pake crod jobs yang jalan sehari sekali untuk clear redis dan mengisinya kembali dengan data dari PostgreSQL sehingga pengunjung akan selalu mendapatkan performa baik.

#### Bootstrap & Sass

Bootstrap dan Sass itu kombinasi tech styling yang bagus, karena dengan sass gue bisa melakukan customisasi untuk style bootstrap secara global. Kenapa ga Tailwind CSS ? Karena di web portfolio gue ini tidak terlalu banyak style element yang berbeda, kebanyakan memiliki komponent yang sama, jadi jika pake tailwind itu akan bikin gue lebih sibuk maintain style dibanding performance.

#### Nginx 

Setup Nginx + systemd + Actix Web di Ubuntu ngajarin gue pentingnya error handling, log rotation, dan security header biar app tetap stabil 24/7 serta sangat cocok untuk compress file `wasm` dengan `gzip` supaya cepat diload oleh browser.

### Chalange

#### Minim Dokumentasi Leptos (waktu awal pengembangan)

Karena framework-nya masih baru, dokumentasi terbatas. Banyak bagian harus dicoba manual dan dibaca langsung dari source code atau Discord community. Tapi gue yakin beberapa tahun kedepan Leptos bakal sangat mature karena sekarang juga sudah banyak bisa di integrasi dengan Supabase, Cloudflare, DaisyUI, TailwindCSS dll.

#### Interaksi antara WASM dan DOM

Beberapa behavior browser perlu penyesuaian ketika render dari WebAssembly, terutama saat transisi dan progress bar antar halaman. Selain itu perlu effort untuk memahami konsep `Fine Grained Reactivity` pada Leptos.

#### Hight Performance di server Kentang (1GB Ram & Cpu)

Deploy aplikasi di server dengan Ram 1GB itu serasa pergi berlayar pake rakit bambu. Rawan & Cuma sedikit resource untuk berlayar, namun bukan berarti untuk mendapatkan Hight Performance harus keluarin biaya lebih. Dengan melakukan beberapa optimasi assets terutama `wasm`, caching, dan markdown render. Akhirnya gue bisa deploy website ini di server minim namun response time <1s serta penerapan Rate Limitter agar tidak mudah dispam.

---

<div class="d-flex flex-row justify-content-center align-items-center">Regards <a href="https://feri-irawansyah.my.id"><img witdh="1rem" src="https://feri-irawansyah.my.id/favicon.ico" alt="Feri Irawansyah"> Feri Irawansyah</a></div>

---
