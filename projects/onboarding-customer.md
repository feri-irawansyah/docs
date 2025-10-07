---

Aplikasi ini adalah cloning dari aplikasi yang gue buat di kantor dengan design, tech, logic, bisnis, dan arsitecture yang berbeda jauh termasuk dengan database dan dummy data karena jika mengcopy langsung project existing merupakan pelanggaran dan plagiarisme bro.

### Tech Stack Reason
Website ini hanya sebuah web client aja bro. Untuk Backend nya gue pake Actix Web Rust pake api Internal gue. Nah jadi tentunya website ini dibuat dengan Javascript.

#### Svelte + Sveltekit
Framework frontend yang gue pake adalah Sveltekit + Svelte Library kenapa? Ya karena svelte framework frontend favorit gue kalo buat aplikasi pake Javascript karena Svelte itu ringan, mudah dan simple terus ga brutal kaya framework sebelah yang kalo error `undefined is not a function` halaman langsung blank. Tapi svelte ini masih kalah jauh dari segi kepopuleran, komunitas blm terlalu gede, dan belum banyak library-library pendukung.

#### Vite
Gue pake Vite mungkin karena gue ini Programmer muda dan ga kenal sama paradigma dan arsitektur jaman dulu, jadi ketika gue masuk frontend langsung kenalnya sama Vite untuk modul bundler. Tapi selain itu dikantor gue pake gulp, jquery, angularjs dan beberapa framework jaman batu yang legendaris lain.

#### Bootstrap + Sass
Karena hanya aplikasi kecil untuk form pendaftaran aja, UI nya itu - itu aja maksudnya jadi gue hanya butuh kestabilan design aja makanya gue pake Bootstrap + Sass. Tapi kalau UI nya dinamis dan banyak custom mungkin gue bakal pake Tailwind CSS.

### Chalange
#### Membuat Business Logic Baru

Karena ini aplikasi internal, jadi gue ga bisa mentah - mentah nyamain bisnisnya karena meskipun santuy dan suka bercanda. Profesional harus teteap jadi nomor satu bro jadi logic untuk membuat logic bisnis yang berbeda lumayan efort karena banyak yang mis juga dan flownya berbeda.

#### Database Yang Berbeda

Aplikasi ini menggunakan DBMS PostgreSQL sedangkan dikantor tidak menggunakan Postgre tapi menggunakan Database lain. Untuk membuat tabel dan dummy data sangat memakan banyak waktu selain harus berimajinasi gue juga harus meenyesuaikan relasi antar data dan bisnisnya.

