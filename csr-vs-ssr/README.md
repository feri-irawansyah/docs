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

</details>







<style>

img .w-50 {
 width: 100% ! important;

}

</style>