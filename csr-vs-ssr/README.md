<style>

img.w-50 {
 width: 100% ! important;

}

</style>

Woi Bro, Kalo Lo programmer jaman batu kalo mau buat website simple tinggal buat folder terus Lo buat file inde.html, style.css dan script.js lalu Boom.

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/horor.png" class="img-fluid w-50" alt="Horor"/>

Dan asal Lo tau website jaman dulu itu stabil bro ga banyak fitur, ga ada animasi yang kalo Lo sorot tombol `Dislike👎` tapi tombolnya malah kabur dan Lo mau ga mau harus tekan `Like 👍`. Atau kalo Lo ga bisa coding dan ga mau coding Lo bisa pergi ke www.wordpress.com dan Lo bisa bikin website tapi kaya bikin PPT di canva.

Jaman sekarang? Lo kalo mau bikin website atau jadi frontend dev banyak persyaratan bro. Lo harus tau CSR, SSR, SSG, Hidration, Component, Layout, Pages, Routes, Utility, Middleware, State, Hooks, Virtual DOM, Resumable, Fine Grained Reactivity dan banyak lagi. Frontend sekarang lebih ribet dari backend bro.

Di artkel gue kali ini gue mau bahas antara CSR (Client Side Render) dengan SSR (Server Side Render).

<details>
<summary><h2>📌 Apa itu CSR dan SSR</h2></summary>

`CSR` atau `Client Side Render` itu artinya UI di render atau di bentuk di browser. Jadi ketika membuka website ada 1 atau lebih file html (umumnya 1) kemudian javascript akan membentuk UI nya sesuai dengan yang Lo suruh. Sedangkan `SSR` atau `Server Side Render` UI di render atau di bentuk dari server, jadi ketika Lo mengakses website server sudah mengirimkan halam berupa html yang udah lengkap.

Analoginya misal Lo minta mobil remote control ke bapak Lo nah karena emang bapak Lo itu tipikal orang CSR jadi dia cuma ngasih duit dan Lo di suruh beli sendiri. Sedangkan kalo bapak Lo itu tipikal SSR nah dia langsung ngasih mobil mainannya ke Lo bro. Kerja bro biar bisa beli mobil mevah kek Lambo.
</details>

<details>
<summary><h2>📌 Filosofi & Skalabilitas</h2></summary>

Tadi udah gue analogiin kan nah kalo secara filosofinya.

CSR = Application 

SSR = Document 

Jadi kalo website Lo di buat dengan CSR artinya Lo kaya ada aplikasi sendiri di client, meskipun website yang jalan di server tapi karena CSR akan membuat UI di browser artinya ada aplikasi yang jalan di browser.

Sementara SSR itu document, artinya yang di kasih ke client atau di browser itu beneran document file jadi.

Jadi CSR lebih scalable dong? Bisa iya bisa tidak tergantung context. Misalnya aplikasi Lo khusus aplikasi yang merender banyak text atau content yang statis dan jarang berubah SSR lebih bagus, karena `google bot` atau `bing` itu sangat cinta sama kontent document yang udah jelas. Tapi kalo aplikasi yang Lo buat itu banyak data atau dinamis dan internal app CSR lebih oke. Kenapa?

Aplikasi dengan data yang dinamis berubah setiap detiknya misalnya ada banyak tabel, real-time chat, web trading, app offline dan lain lain yang intinya dimanis lah. Itu bakal sering bnyk perubahan UI dan sering request ke server. Kalo aplikasi begitu Lo pakein SSR kasian server Lo bro bakal ngos - ngosan dia selain ngurus request response api ad juga ngurusin render halaman. Jadi bakal kerja double si server. Makanya website - website yang dibuat dengan SSR itu biasanya reload ketika pindah halaman karena dia akan melakukan request ulang ke server.

</details>

<details>
<summary><h2>📌Performance & Biaya</h2></summary>

Soal performance gimana bro? Tergantung juga bro. Balik lagi ke use case nya gue kasih contoh website gue ini. Ini gue buat pake SSR kenapa? Karena suka - suka gue dong wkwkwk.

<h3>Performance SSR</h3>

Kaga bro gue bikin SSR karena di website ini gue banyak artikel dan catatan - catatan receh dan murah meriah gue banyak render text, data statis bahkan text panjang, dan biar SEO friendly juga jadi kalo di buat pake SSR ini lebih oke. Tapi keliatan dinamis dan ga reload dan dinamis? Ini karena gue pake fitur hydration dan fine grained reactivity milik `Leptos` jadi keliatan kaya CSR tapi sebenarnya ini SSR.

Terus kenapa gue ga pake CSR aja? Pada awalnya gue bikin pake CSR tapi karena CSR itu membuat UI di browser jadi perlu ada loading atau placeholder ketika halaman awal blank. Selain itu banyak text panjang yang kaya Lo baca ini, perlu lazzy load untuk merendernya makanya gue pake SSR biar content langsung jadi document html di browser.

Jadi untuk use case seperti ini sangat cocok menggunakan SSR dan performa nya jauh lebih baik karena content statis akan langsung japan di browser tanpa ada lazzy loading.

<h3>Biaya Infrastruktur untuk SSR</h3>

Karena data di artikel ini tidak banyak dan statis. Selain itu update data ke database jarang apalagi buat orang sibuk kaya gue dan memang datanya jarang berubah jadi penggunaan resource server sangat murah. Karena cuma render text artikel, text lagi dan text tidak ada tabel yang update tiap detik. Jadi server tetep santai sambil liburan.

Tapi beda bro kalo Lo pake SSR untuk website dengan data dinamis misalnya untuk website dashboard admin yang datanya selalu update, ketika data barubah SSR perlu request ulang datanya ke server ketika data berubah maka server akan melakukan re render ulang document nya kemudian si kembalikan lagi ke browser. Jika proses ini terjadi detiknya maka server akan sangat sibuk menangani request api, response api, re render ulang dan mengembalikan document terupdate nya ke browser. Ini mahal bro.

<h3>Performance CSR</h3>

Kalo use case Lo itu aplikasi yang dinamis CSR lebih cocok untuk itu karena UI akan di render di client, server hanya akan merender file html polos ke browser kemudian javascript di browser akan membuat UI nya secara interaktif jika ada perubahan pada data maka javascript CSR juga akan melakukan render ulang tanpa melakukan request lagi ke server. Performa CSR ini stabil bro, kecepatan render akan mengikuti response time dari api yang dipake.

Tapi untuk SEO friendly CSR ini kurang bagus karena yang dikirim oleh server ke browser itu html kosong belum ada content nya, jadi `google bot` atau `bing` akan menganggap content nya tidak ada karena belum di render karena CSR memerlukan waktu untuk membuat UI nya.

<h3>Biaya Infrastruktur CSR</h3>

Karena rendering UI di lakukan di browser artinya server tidak perlu banyak bekerja karena server hanya akan mengurus request/response dari backend saja dan tidak perlu bekerja untuk render document. Ini akan sangat murah untuk servernya semua proses render atau pembuatan UI di lakukan di browser.

</details>

