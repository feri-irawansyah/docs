Bro, Lo pernah pergi ke perpustakaan ngga bro? Mesti Lo bakal nemuin buku - buku tersusun rapi di rak dan sesuai kategorinya. Sekarang bandingkan dengan Lemari Lo, baju, celana, mie instan, celana dalem + sendal belom di cuci bahkan foto mantan juga ada di lemari. Itu contoh gambaran Database SQL (Relationl Database) dan NOSQL (No Relationl Database). Eeehh sek... Harusnya Lo udah tau `APA ITU DATABASE DAN DBMS` jadi gue cuma kasih tau kalo database itu tempat buat nyimpen data. Entah itu data keuangan, catatan, transaksi, pesan, bahkan dosa - dosa juga bisa disimpan di Database (Tapi kalo mau akses harus Login ke akhirat dlu bro, ga recomended). Sedangkan `DBMS` atau Database Management System itu aplikasi untuk manajemen database. Dengan DBMS kita bisa mengelola database seperti membuat, menghapus, memperbarui dan mengambil data.

Okeh balik lagi ke SQL dan NOSQL. SQL itu kaya perpustakaan, rapi, data terstruktur di simpan sesuai kelompoknya ada tabel, field dan petunjut buat cari datanya. Sedangkan NOSQL itu kaya lemari Lo bro apa aja bisa Lo masukin dan ngga ada aturan buat nyimpen datanya semua Lo yang control bro. Gue mau bahas database SQL atau Relational karena database SQL paling sering digunakan apalagi buat Lo yang jadi specialist CRUD developer lahir dan batin. Ada banyak DBMS di alam semesta ini, misal MySQL, PostgreSQL, MariaDB, SQLite, Oracle, Microsoft SQL Server, dll. Dan kalo Lo misal searching di Google terus nyasar ke link <a href="https://db-engines.com/en/ranking/relational+dbms" target="_blank">https://db-engines.com</a> Lo bakal ketemu banyak DBMS dan kepopulritasnya.

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/postgres-sql/assets/rangking.png" alt="postgres-sql/assets/rangking.png"/>

Nah PostgreSQL ada di peringkt ke 4 paling populer di dunia dan peringkt 1 untuk free DBMS yang bersahabat sama Lo yang awal bulan belom bayar uang kos. Sedangkan untuk Oracle, MySQL dan SQL Server Lo harus bayar kalo misal pingin dapet full fitur dan dapet suport untuk project beneran. Dan harganya juga lumayan bro mungkin Lo harus jual ginjal kuyang dulu baru bisa beli versi Enterpricenya. Tapi untuk PostgreSQl Lo bisa pake DBMS enterprice tanpa keluar dompet meski dompet Lo isinya poto kenangan. Selain PostgreSQL Lo juga bisa pake SQLite dimana file database itu Lo yang control.

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/postgres-sql/assets/cara-kerja.png" alt="postgres-sql/assets/cara-kerja.png"/>

### Database Client
Database Client itu aplikasi yang dipake untuk berkomunikasi dengan DBMS. Nah biasanya DBMS sudah menyediakan database client sederhana yang bisa Lo pake bro. Jadi misal Lo mau naik kereta bisanya si penyedia layanan kereta udah nyediain tiket dan khusus buat Lo, Lo ga bisa masuk dan pake layanannya kalo Lo ga punya tiketnya. Jadi database client itu sama kaya tiket buat bisa mengakses database yang udah disediain sama DBMS itu sendiri. 

Tapi Lo juga bisa pake tools atu aplikasi lain biar bisa komunikasi sama database misal Lo buat aplikasi pake King PHP, Javascript, Java, Rust, Go, Python, dll. ibarat kaya Lo kalo beli tiket kereta Lo ngga langsung ke stasiun ngantri panjang, tapi Lo bisa beli di penyedia layanan kaya Tokped, Traveloka, KAI Access dll.

### SQL (Structured Query Language)
Lo pernah ga bro naik kereta terus Lo teriak - teriak, terus Lo duduk se enaknya dan yang lebih absurt Lo godain mba - mba Prami (Oramugari Kereta). Kalo Lo ngelauin itu gapapa, paling Lo bakal di keluarin atau bahkan kena pidana xixixi. Nah DBMS juga kaya gitu bro, Lo ngga bisa asal dan melakukan apa aja di database. Jadi Lo perlu tau SQL (Structured Query Language). SQL ini adalah bahasa pemrograman untuk mengakses dan memanipulasi database. Jadi ini adalah tata cara dan sop yang perlu Lo pahami biar Lo ga tersesat di jalan kemusyrikan bro :). Kaya yang sebelumny gue sebutin ada Databse SQL itu artinya database yang pake (Structured Query Language) untuk mengakses dan memanipulasi datanya. Semua perintah SQL itu pada dasarnya sama meski beberapa DBMS punya improvement masing - masing tapi secara garis besar logic dan perintahnya hampir sama.

### Datbase File
Kebayanyakan DBMS itu menyimpan datanya menggunakan file atau DISK fisik, meskipun ada beberapa yang datanya itu disimpen di memory (RAM). Nah tapi meski disimpen di file, tapi filenya ngga bisa Lo buka terus dibaca kaya Lo baca chat gebetan yang harus Lo jawab cepet - cepet. File database itu kompleks bro, ga kaya CSV atau Excel yang bisa Lo baca sambil rebahan santuy. Dan biasanya DBMS itu punya cara masing - masing buat ngelola file - file nya.

<div class="row justify-content-start">
    <div class="col-md-2 col-12">
        <img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/postgres-sql/assets/postgresql.png" alt="postgres-sql/assets/1.png"/>
    </div>
    <div class="col-md-10 col-12">
       <p>Lanjut ke PostgreSQL karena judulnya udah PostgreSQL kan kocak kalo jadinya bahas king PHP atau Javascript. PostgreSQL berakar dari proyek Ingres (Interactive Graphics and Retrieval System) yang dibuat di University of California, Berkeley (UCB) tahun 1970-an. Tahun 1982, Michael Stonebraker (profesor di UCB) merasa Ingres sudah mulai terbatas, lalu dia memulai proyek baru bernama POSTGRES dari kata `post-ingres` (Penerus Ingres). Dan akhirnya ditahun 1996-1997 Versi PostgreSQL 6.0 menjadi rilis komunitas pertama. Untuk lebih lengkapnya Lo bisa baca langsung di <a href="https://www.postgresql.org/docs/current/history.html">https://www.postgresql.org/docs/current/history.html</a>. </p>
       <p>Kenapa PostgreSQL? `Karena gue biasa pake PostgreSQL, kalo biasa pake MySQL paling juga MySQL`. Ga gitu juga bro. Untuk database yang sering gue pake itu Microsoft SQL Server. Karena kebeulan dikantor gue pake MSSQL. Tapi untuk database favorit gue itu PostgreSQL karena fiturnya banyak meskipun kaga dipake semua.</p>
    </div>
</div>
<p>
Untuk DBMS open source (gratis) itu MySQL masih menjadi TOP Tier karena untuk Query `SELECT` dan `INSERT` MySQL memiliki performa yang lebih tinggi. Selain itu MySQL juga ada Enterprise Edition (EE) dan biasanya Corporate lebih memilih DBMS yang ada suport dari Perusahaan pengembangnya. Apalagi sekarang MySQL sudah di Akuisi oleh Oracle. Tapi buat Lo yang pingin jadi specialist CRUD developer, Lo bebas mau memilih DBMS apapun dan untuk kasus ditempat kerja Lo misal Lo ikutin standar dari Perusahaan jangan ngeyel pake yang Lo mau karena kerja tim memerlukan kenyamanan dalam bekerja.
</p>
<p>
Agenda di catatan kecil gue ini ga banyak kok paling 12😁
</p>

1. Installation PostgreSQL
2. Tipe Data
3. Query SQL
4. Database dan Table
5. Select, Insert, Update, Delete
6. Join
7. Relationship
8. Transaction
9. View
10. Function dan Procedure
11. Index
12. Trigger

<details>
<summary><h2>📌 Installation PostgreSQL</h2></summary>

Syarat yang harus dipenuhi buat install PostgreSQL itu OS dan Device bro. Lo coba buka link ini <a href="https://www.postgresql.org/download" target="_blank">https://www.postgresql.org/download/</a>

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/postgres-sql/assets/os.png" alt="postgres-sql/assets/os.png"/>

Ada Linux, MacOS, Windows, BSD, Solaris bebas Lo mau pake yang mana. Yang gue pake di catatan ini Windows 11. Untuk device ga usah gede - gede pake Laptop ram 4GB juga udah bisa bro.

<h4>🪡 Download PostgreSQL</h4>

Untuk link download Lo bisa ke link ini <a href="https://www.enterprisedb.com/downloads/postgres-postgresql-downloads" target="_blank">https://www.enterprisedb.com/downloads/postgres-postgresql-downloads</a> Atau dari halaman sebelumnya Lo klik icon Windows nanti bakal pergi ke halaman ini juga.

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/postgres-sql/assets/download.png" alt="postgres-sql/assets/download.png"/>

Nah Lo klick aja yang Windows x86-64 kalo windows Lo 86/64 bit. Kalo windows 11 kaya punya gue bisanya 64 jadi tinggal klik nanti terdownload yang penting pastikan ada koneksi internet buat downloadnya. Kalo ga ada bisa numpang Wifi tetangga. Kalo udah terdownload nanti ada muncul notifikasi dan di folder download Lo harusnya juga udah ada file installer PostgreSQLnya.

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/postgres-sql/assets/file-postgres.png" alt="postgres-sql/assets/file-postgres.png"/>


<h4>🪡 Setup Installasi PostgreSQL</h4>


</details>