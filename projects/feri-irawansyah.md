### Why Leptos ?

Kenapa Leptos karena Rust itu bahasa pemrograman favorit gue dari awal masuk dunia teknologi dan Rust itu selalu mengingatkan gue tentang kesalahan - kesalahan dan bug yang lebih kompleks jadi gue ga bisa sembarangan nulis code. Selain itu gue deploy Website ini di VPS dengan ram dan cpu 1GB kurang malah. Jadi gue perlu tools yang bisa jalan di kondisi server yang kentang.

### Tools Combination

Web Portfolio ini gue buat dengan kombinasi beberapa Teknologi sesuai dengan kebutuhan

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

---

<div class="text-center">Regards <a href="https://feri-irawansyah.my.id">Feri Irawansyah</a></div>

---
