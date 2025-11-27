<style>

    @media (min-width: 768px) {
        .img-fluid {
            max-width: 50%;
        }
    }

    .img-app {
        display: flex;
        justify-content: start;
        flex-direction: row;
        gap: 1rem;
    }

    .img-app img {
        max-height: 50px;
    }

    .img-app img[alt="notion"] {
       max-width: 50px;
    }
    .img-app img[alt="jira"] {
       max-width: 50px;
    }
    .img-app img[alt="slack"] {
       max-width: 50px;
    }
    .img-app img[alt="zoom"] {
       max-width: 50px;
    }
    .img-app img[alt="whatsapp"] {
       max-width: 50px;
    }
    .img-app img[alt="gmeet"] {
       max-width: 50px;
    }
    .img-app img[alt="powerbi"] {
       max-width: 50px;
    }
    .img-app img[alt="grafana"] {
       max-width: 50px;
    }
    .img-app img[alt="shopee"] {
       max-width: 50px;
    }
    .img-app img[alt="blibli"] {
       max-width: 50px;
    }
    .img-app img[alt="google"] {
       max-width: 50px;
    }
    .img-app img[alt="reddit"] {
       max-width: 50px;
    }
    .img-app img[alt="apple"] {
       max-width: 50px;
    }


    .img-app-item {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-between;
    }

    .img-code {
        max-width: 1.3rem;
    }

    .title-code {
      display: flex;
      flex-direction: row;
      align-items: center;
      font-size: 1.3rem;
      gap: 1rem;
    }

</style>

Woi Bro, Kalo Lo programmer jaman batu kalo mau buat website simple tinggal buat folder terus Lo buat file inde.html, style.css dan script.js lalu Boom.

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/horor.png" class="img-fluid" alt="Horor"/>

Dan asal Lo tau website jaman dulu itu stabil bro ga banyak fitur, ga ada animasi yang kalo Lo sorot tombol `Dislike👎` tapi tombolnya malah kabur dan Lo mau ga mau harus tekan `Like 👍`. Atau kalo Lo ga bisa coding dan ga mau coding Lo bisa pergi ke www.wordpress.com dan Lo bisa bikin website tapi serasa kaya bikin PPT di canva.

Jaman sekarang? Lo kalo mau bikin website atau jadi frontend dev banyak persyaratan bro. Lo harus tau CSR, SSR, SSG, Hydration, Component, Layout, Pages, Routes, Utility, Middleware, State, Hooks, Virtual DOM, Resumable, Fine Grained Reactivity dan banyak lagi. Frontend sekarang lebih ribet dari backend bro.

Di artkel gue kali ini gue mau bahas antara CSR (Client Side Render) dengan SSR (Server Side Render).

<details open>
<summary><h2>📌 Apa itu CSR dan SSR</h2></summary>

`CSR` atau `Client Side Render` itu artinya UI di render atau di bentuk di browser. Jadi ketika membuka website ada 1 atau lebih file html (umumnya 1) kemudian javascript akan membentuk UI nya sesuai dengan yang Lo suruh. Sedangkan `SSR` atau `Server Side Render` UI di render atau di bentuk dari server, jadi ketika Lo mengakses website server sudah mengirimkan halam berupa html yang udah lengkap.

Analoginya misal Lo minta mobil remote control ke bapak Lo nah karena emang bapak Lo itu tipikal orang CSR jadi dia cuma ngasih duit dan Lo di suruh beli sendiri. Sedangkan kalo bapak Lo itu tipikal SSR nah dia langsung ngasih mobil mainannya ke Lo bro. Kerja bro biar bisa beli mobil mevvah kek Lambo.
</details>

<details open>
<summary><h2>📌 Filosofi & Skalabilitas</h2></summary>

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/filosofi.png" class="img-fluid" alt="filosofi"/>

Tadi udah gue analogiin kan nah kalo secara filosofinya.

CSR = Application 

SSR = Document 

Jadi kalo website Lo di buat dengan CSR artinya Lo kaya ada aplikasi sendiri di client, meskipun website yang jalan di server tapi karena CSR akan membuat UI di browser artinya ada aplikasi yang jalan di browser.

Sementara SSR itu document, artinya yang di kasih ke client atau di browser itu beneran document file jadi.

Jadi CSR lebih scalable dong? Bisa iya bisa tidak tergantung context. Misalnya aplikasi Lo khusus aplikasi yang merender banyak text atau content yang statis dan jarang berubah SSR lebih bagus, karena `google bot` atau `bing` itu sangat cinta sama kontent document yang udah jelas. Tapi kalo aplikasi yang Lo buat itu banyak data atau dinamis dan internal app CSR lebih oke. Kenapa?

Aplikasi dengan data yang dinamis berubah setiap detiknya misalnya ada banyak tabel, web admin, web office internal dan lain lain yang intinya dimanis lah. Itu bakal sering bnyk perubahan UI dan sering request ke server. Kalo aplikasi begitu Lo pakein SSR kasian server Lo bro bakal ngos - ngosan dia selain ngurus request response api ad juga ngurusin render halaman. Jadi bakal kerja double si server. Makanya website - website yang dibuat dengan SSR itu biasanya reload ketika pindah halaman karena dia akan melakukan request ulang ke server.

</details>

<details open>
<summary><h2>📌 Performance & Biaya</h2></summary>

Soal performance gimana bro? Tergantung juga bro. Balik lagi ke use case nya gue kasih contoh website gue ini. Ini gue buat pake SSR kenapa? Karena suka - suka gue dong wkwkwk.

<h3>Performance SSR</h3>

Kaga bro gue bikin SSR karena di website ini gue banyak artikel dan catatan - catatan receh dan murah meriah gue banyak render text, data statis bahkan text panjang, dan biar SEO friendly juga jadi kalo di buat pake SSR ini lebih oke. Tapi keliatan dinamis dan ga reload dan dinamis? Ini karena gue pake fitur hydration dan fine grained reactivity milik `Leptos` jadi keliatan kaya CSR tapi sebenarnya ini SSR.

Terus kenapa gue ga pake CSR aja? Pada awalnya gue bikin pake CSR tapi karena CSR itu membuat UI di browser jadi perlu ada loading atau placeholder ketika halaman awal blank. Selain itu banyak text panjang yang kaya Lo baca ini, perlu lazzy load untuk merendernya makanya gue pake SSR biar content langsung jadi document html di browser.

Jadi untuk use case seperti ini sangat cocok menggunakan SSR dan performa nya jauh lebih baik karena content statis akan langsung japan di browser tanpa ada lazzy loading.

<h3>Biaya Infrastruktur untuk SSR</h3>

Karena data di artikel ini tidak banyak dan statis. Selain itu update data ke database jarang apalagi buat orang sibuk kaya gue dan memang datanya jarang berubah jadi penggunaan resource server sangat murah. Karena cuma render text artikel, text lagi dan text tidak ada tabel yang update tiap detik. Jadi server tetep santai sambil liburan.

Tapi beda bro kalo Lo pake SSR untuk website dengan data dinamis misalnya untuk website dashboard admin yang datanya selalu update, ketika data barubah SSR perlu request ulang datanya ke server ketika data berubah maka server akan melakukan re render ulang document nya kemudian si kembalikan lagi ke browser. Jika proses ini terjadi detiknya maka server akan sangat sibuk menangani request api, response api, re render ulang dan mengembalikan document terupdate nya ke browser. Ini mahal bro.

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/ssr-flow.png" class="img-fluid" alt="ssr-flow"/>

<h3>Performance CSR</h3>

Kalo use case Lo itu aplikasi yang dinamis CSR lebih cocok untuk itu karena UI akan di render di client, server hanya akan merender file html polos dan beberapa asset lain ke browser kemudian javascript akan membuat UI nya secara interaktif di browser. Jika ada perubahan pada data maka javascript CSR juga akan melakukan render ulang tanpa melakukan request lagi ke server. Performa CSR ini stabil bro, kecepatan render akan mengikuti response time dari api yang dipake. Tapi untuk load awal CSR mungkin akan terasa lebih berat karena semua file akan di download ke browser kemudian javascript akan membentuk UI nya ini akan membutuhkan beberapa waktu.

Tapi untuk SEO friendly CSR ini kurang bagus karena yang dikirim oleh server ke browser itu html kosong belum ada content nya, jadi `google bot` atau `bing` akan menganggap content nya tidak ada karena belum di render karena CSR memerlukan waktu untuk membuat UI nya.

<h3>Biaya Infrastruktur CSR</h3>

Karena rendering UI di lakukan di browser artinya server tidak perlu banyak bekerja karena server hanya akan mengurus request/response dari backend saja dan tidak perlu bekerja untuk render document. Ini akan sangat murah untuk servernya semua proses render atau pembuatan UI di lakukan di browser.

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/csr-flow.png" class="img-fluid" alt="csr-flow"/>

</details>


<details open>
<summary><h2>📌 Security</h2></summary>

Keamanan ? Kalo di lihat dari implementasinya SSR terasa lebih aman karena datanya tersentralisasi di server dan tidak ada data di browser. Gue breakdown beberapa masalah security CSR dan SSR.

<div class="table-responsive">
  <table class="table">
    <thead>
      <tr>
        <th scope="col">#</th>
        <th scope="col">Security Aspect</th>
        <th scope="col">CSR (Client-Side Rendering)</th>
        <th scope="col">SSR (Server-Side Rendering)</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <th scope="row">1</th>
        <td>Exposure of Business Logic</td>
        <td>❌ Business logic & API interaction ada di browser → bisa dicuri, dipelajari, di-manipulate</td>
        <td>✅ Logic tetap di server → lebih sulit direkayasa</td>
      </tr>
      <tr>
        <th scope="row">2</th>
        <td>Data Sensitivity</td>
        <td>❌ Semua data dikirim via API → bisa di-inspect</td>
        <td>✅ Hanya hasil HTML dikirim → data sensitif tetap di server</td>
      </tr>
      <tr>
        <th scope="row">3</th>
        <td>API Key/Secrets Safety</td>
        <td>❌ Jika keliru, keys bisa bocor di FE</td>
        <td>✅ Semua secret aman di server</td>
      </tr>
      <tr>
        <th scope="row">4</th>
        <td>Attack Surface</td>
        <td>❌ Endpoint API banyak & terbuka</td>
        <td>⚠️ Fewer endpoints but still must be secured</td>
      </tr>
      <tr>
        <th scope="row">5</th>
        <td>XSS (Cross-Site Scripting)</td>
        <td>⚠️ Rentan karena full DOM di browser</td>
        <td>⚠️ Tetap bisa terjadi kalau sanitasi buruk</td>
      </tr>
      <tr>
        <th scope="row">6</th>
        <td>DDoS Impact</td>
        <td>🟢 Lebih kuat → beban berat ada di client</td>
        <td>🔴 Sangat rentan → rendering dilakukan tiap request</td>
      </tr>
      <tr>
        <th scope="row">7</th>
        <td>SEO Injection / HTML Tampering</td>
        <td>⚠️ Bisa di override via DevTools</td>
        <td>🟢 Sudah pre-render → lebih aman</td>
      </tr>
      <tr>
        <th scope="row"></th>
        <td>Authentication</td>
        <td>⚠️ Banyak butuh localStorage/sessionStorage → bisa dicuri via XSS</td>
        <td>🟢 HTTP-Only Cookies lebih aman, Session validated server-side</td>
      </tr>
      <tr>
        <th scope="row">9</th>
        <td>Role + Access Control</td>
        <td>❌ Logic di client bisa di-bypass</td>
        <td>🟢 Full access control di server</td>
      </tr>
    </tbody>
  </table>
</div>

Tapi balik lagi bro, security itu bukan di frontend tapi di backend dan server Lo. Frontend hanya UI dan template bukan bussines logic. 
1. Bussines logic tetap ada di backend dan server
2. Jangan expose data sensitif ke frontend
3. Semua secret key jangan dikirim ke frontend, bisa gunakan serialize/encrypt jika memang butuh di frontend
4. Berikan akses terbatas ke API (authorization header)
5. Berikan validasi kuat di request header dan body agar tidak bisa inject script ke backend
6. Bisa pake rate limiter agar tidak mudah dispam
7. Bisa matikan DevTools ketika production atau blokir akses seperti iframe, sandbox, dan sebagainya
8. Authentication tetep di Cookie dan backend yang handler, frontend hanya menerima response APi yang valid aja
9. Role + Access Control tetep di server dan backend yang handler. Frontend hanya membuat logic dari response API yang valid aja

Jadi mau CSR atau SSR itu sebenarnya sama aja bro, karena security itu di ranah backend bukan di frontend. Bedanya SSR itu lebih tertutup aja untuk data exposure karena data tetep di server.

Asal Lo tau bro, gue ambil contoh lagi dari website tempat Lo baca article ini itu juga sama implementasinya. Meskipun frontend dibuat dengan `Rust` yang `type safe`, `memory safe`, `thread safe`, `compile time check` dan `wasm bundle` yang susah dibongkar ga kaya javascript bundle. Tapi tetep aja bro frontend hanyalah template bukan bussines logic. Gue teteap taro bussines logic di backend terpisah, tidak langsung raw query di `Leptos` karena bukan tempatnya.

Dan katanya `Rust + Actix` itu tahan banting? Mau di spam tetep kek beton? Kaga juga bro, VM gue ini kecil cuma 500MB ram, jadi tetep gue pake rate limiter juga biar kaga sembarangan di spam. Karena keamanan itu bukan di teknologi, tapi dari diri Lo sendiri bro yang selalu hati - hati dan tetep berbuat kebaikan.

Notes: Backend disini maksud gue itu `Optional` terpisah. Sveltekit SSR itu jalan di runtime nodejs, jadi Lo juga bisa bikin layer sendiri untuk backend di satu project. Lo bisa pake express atau bisa pake framework lain. Kalo terpisah juga lebih bagus, karena tersentralisasi.

</details>

<details open>
<summary><h2>📌 Contoh Aplikasi CSR dan SSR</h2></summary>

Jadi ga semua aplikasi itu harus di pukul rata **POKOKNYA HARUS SSR** atau **POKOKNYA HARUS CSR** pakelah teknologi sesuai kebutuhan dan diskusi dengan tim untuk menentukan keputusan bersama. Mungkin kalo misal Lo bikin aplikasi sendiri itu terserah Lo bro, mau pake apa aja itu terserah Lo. Tapi kalo Lo kerja secara tim di perusahaan atau ada project dimana yang mengerjakan bukan Lo dan Ego Lo sendiri, itu harus dengan kesepakatan bersama.

<h3>Category Aplikasi CSR</h3>

Aplikasi CSR umumnya di design untuk aplikasi private page yang muncul ketika user login. Berikut beberapa contoh aplikasi CSR:

<h4> SaaS (Software as a Service) </h4>

Kenapa `SaaS` cocok dengan CSR?  
1. Aplikasi SaaS biasanya memiliki banyak UI yang kompleks
2. SasS juga mengunakan data yang dinamis
3. CSR punya state local yang tidak hilang, jadi state bisa digunakan lagi oleh user ketika user pindah halaman
4. Banyak event, websocket, data berubah cepat
5. Banyak user dalam 1 platform shared infra jadi beban server lebih kecil

Contohnya seperti `Notion`, `Jira`, `Slack Web` dan sebagainya.

<div class="img-app">
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/notion.png" class="img-fluid" alt="notion"/>
    <p>Notion</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/jira.svg" class="img-fluid" alt="jira"/>
    <p>Jira</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/slack.svg" class="img-fluid" alt="slack"/>
    <p>Slack</p>
  </div>
</div>

<h4> Back Office, ERP & CRM </h4>

Fitur pada back office biasanya kurang lebih mirip dengan SaaS yaitu banyak event dan data dinamis. Selain itu aplikasi back office juga memiliki penggunaan data dengan jumlah yang banyak. Bisa memiliki banyak table yang sering diupdate tiap detik. Jadi aplikasi back office cocok dengan CSR.

Conohnya seperti `Mekari`, `Odoo UI` `Admin Dashboard` dan sebagainya.

<div class="img-app">
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/mekari.png" class="img-fluid" alt="mekari"/>
    <p>Mekari</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/odoo.svg" class="img-fluid" alt="odoo"/>
    <p>Odoo</p>
  </div>
</div>

<h4> Realtime & Collaborative App </h4>

Aplikasi dengan realtime update biasanya menggunakan koneksi dua arah seperti WebSocket, WebRTC, WebTrasport dan sebagainya. Jadi aplikasi realtime cocok dengan CSR. Karena perubahan data dilakukan secara realtime dan akan langsung ke pengguna lainnya. Jika menggunakan SSR akan menambahkan beban yang tinggi ke server dan bahkan bisa berbahaya untuk aplikasi yang menggunakan WebTransport karena menggunakan streaming video atau audio dalam mengirimkan event atau data.

Contohnya seperti `Zoom`, `Google Meet` `Google Docs`, `WhatsApp` dan sebagainya.

<div class="img-app">
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/zoom.svg" class="img-fluid" alt="zoom"/>
    <p>Zoom</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/gmeet.svg" class="img-fluid" alt="gmeet"/>
    <p>Google Meet</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/gdocs.svg" class="img-fluid" alt="gdocs"/>
    <p>Google Docs</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/whatsapp.svg" class="img-fluid" alt="whatsapp"/>
    <p>WhatsApp</p>
  </div>
</div>

<h4> Data Analytics & Visualization Tools </h4>

Website dengan fitur menampilkan grafik dan visualisasi biasanya akan menggunakan data yang cukup besar, penggunaan logika agregation membutuhkan event handler dan data yang dinamis. Jadi aplikasi ini cocok jika menggunakan rendering CSR karena akan banyak action di sisi client.

Contohnya seperti `Google Data Studio`, `Power BI`, `Grafana` dan sebagainya.

<div class="img-app">
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/gstudio.svg" class="img-fluid" alt="gstudio"/>
    <p>Google Data Studio</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/powerbi.svg" class="img-fluid" alt="powerbi"/>
    <p>Power BI</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/grafana.svg" class="img-fluid" alt="grafana"/>
    <p>Grafana</p>
  </div>
</div>

<h4>Email & Productivity</h4>

Untuk aplikasi productivity ini tidak memerlukan SEO, selain itu aplikasi ini tergolong `Private content` jadi content pada aplikasi ini sangat sensitif dan tidak bisa dishare ke orang lain bahkan ke `google bot` atau `bing`. Interaktifitasnya juga sangat cepat dan sering mengubah halaman. Jadi aplikasi ini cocok dengan CSR.

Contohnya seperti `Gmail`, `Outlook`, `Google Calendar` dan sebagainya.

<div class="img-app">
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/gmail.svg" class="img-fluid" alt="gmail"/>
    <p>Google Mail</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/outlook.svg" class="img-fluid" alt="outlook"/>
    <p>Microsoft Outlook</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/gcalendar.svg" class="img-fluid" alt="gcalendar"/>
    <p>Google Calendar</p>
  </div>
</div>

<h4>Online Store</h4>

Aplikasi online store menggunakan CSR ? Jawabannya tidak sepenuhnya bro, jadi aplikasi online store ini di buat hybrid CSR + SSR. Karena online store ini sangat membutuhkan SEO untuk memasarkan penjualan tapi memiliki UI yang kompleks dan realtime update juga. 

Lo mesti tidak asing dengan nama **Eko Kurniawan Khannedy** beliau adalah kontent creator youtube <a href="https://www.youtube.com/@ProgrammerZamanNow" target="_blank">Programmer Zaman Now</a> dan juga seorang Teknik Architect di <a href="https://www.blibli.com/" target="_blank">Blibli</a>. Pada salah satu Videonya yang berjudul <a href="https://www.youtube.com/watch?v=HG7_HKzmjtA" target="_blank">Server side render lemot</a> beliau pernah menyatakan bahwa Blibli dan E-Commerce lainnya menggunakan CSR dan SSR. Jadi content yang digunakan di client itu menggunakan SSR karena online store memerlukan SEO. Namun ketika user login, halaman yang di berikan ke user adalah CSR.

Jadi aplikasi online store cocok dengan CSR + SSR. Contohnya seperti `Shopee`, `Tokopedia`, `Blibli` dan sebagainya.

<div class="img-app">
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/shopee.svg" class="img-fluid" alt="shopee"/>
    <p>Shopee</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/tokopedia.svg" class="img-fluid" alt="tokopedia"/>
    <p>Tokopedia</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/blibli.png" class="img-fluid" alt="blibli"/>
    <p>Blibli</p>
  </div>
</div>

<h3>Category Aplikasi SSR</h3>

SSR biasanya digunakan untuk aplikasi yang cendering memiliki kontent statis, event handler sedikit, branding dan sebagainya.

<h4> Online Store (Product Page)</h4>

Seperti yang gue bahas sebelumnya pada website online store itu menggunakan hybrid yaitu CSR dan SSR. Khususnya pada product page karena product page membutuhkan SEO untuk pemasaran product. 

Contohnya seperti `Amazon`, `Shopee`, `Tokopedia`, `Blibli` dan sebagainya.

<div class="img-app">
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/amazon.svg" class="img-fluid" alt="amazon"/>
    <p>Amazon</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/shopee.svg" class="img-fluid" alt="shopee"/>
    <p>Shopee</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/tokopedia.svg" class="img-fluid" alt="Tokopedia"/>
    <p>Tokopedia</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/blibli.png" class="img-fluid" alt="blibli"/>
    <p>Blibli</p>
  </div>
</div>

<h4> News & Media </h4>

Aplikasi berita ini banyak menggunakan SSR bahkan seperti diharuskan kenapa? Karena berita ini bersifat text content dan harus sesegera mungkin dipublikasikan ke halayak umum. Jadi memerlukan performa SEO yang tinggi untuk memasarkan konten makanya menggunakan SSR sebagai renderingnya.

Contohnya seperti `Wikipedia`, `Detik`, `Kompas` dan sebagainya.

<div class="img-app">
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/wikipedia.svg" class="img-fluid" alt="wikipedia"/>
    <p>Wikipedia</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/detik.png" class="img-fluid" alt="detik"/>
    <p>Detik News</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/kompas.png" class="img-fluid" alt="kompas"/>
    <p>Kompas</p>
  </div>
</div>

<h4>Blog</h4>

Aplikasi blog ini mirip seperti website news yaitu memuat konten text yang panjang bedanya secara penyampaian kontennya. Kontent blog lebih ke personal notes yaitu catatan dan opini pribadi seseorang bukan suatu berita yang ada di internet. Nah aplikasi yang merender kontent text ini memerlukan SEO juga agar dapat di crawl oleh `google bot` atau `bing` agar mudah dicari dicari di mesin pencarian.

Contohnya seperti `Medium`, `Blogger`, `Ghost` dan sebagainya.

<div class="img-app">
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/medium.svg" class="img-fluid" alt="medium"/>
    <p>Medium</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/blogger.svg" class="img-fluid" alt="blogger"/>
    <p>Bloger</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/gost.png" class="img-fluid" alt="gost"/>
    <p>Ghost</p>
  </div>
</div>

<h4> Corporate & Company Profile </h4>

Aplikasi company profile biasanya memuat konten statis berupa informasi perusahaan seperti deskripsi, kontak, laporan keuangan atau CSR (Corporate Social Responsibility) dan sebagainya. Company profile juga membantu menjelaskan identitas, visi misi, produk/layanan, dan pencapaian perusahaan kepada klien, investor, dan calon karyawan. Namun untuk website company profile tidak sekompleks seperti website news dan blog. Ada juga beberapa website company profile yang menggunakan static html render karena hanya sebagai identitas aja ga perlu database, state management, atau bahkan backend.

Contohnya seperti `Google`, `Microsoft`, `Apple` dan sebagainya.

<div class="img-app">
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/google.svg" class="img-fluid" alt="google"/>
    <p>Google</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/microsoft.svg" class="img-fluid" alt="microsoft"/>
    <p>Microsoft</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/apple.svg" class="img-fluid" alt="apple"/>
    <p>Apple</p>
  </div>
</div>

<h4> Forum / Community </h4>

Forum - forum diskusi online yang publik biasanya juga menggunakan SSR sebagai rendering nya karena memuat konten text argument dan komentar yang membutuhkan SEO. Biasanya pada website forum ini tidak menggunakan realtime update atau koneksi dua arah. Kenapa? Karena forum diskusi ini bersifat public discussion dan disediakan oleh community non provit yang tidak memiliki sumber untuk menyediakan server yang besar untuk menghandle realtime update. Jadi penyedia hanya menggunakan SSR sebagai renderingnya + backend 1 arah agar tetep mendapatkan SEO yang baik tapi performa stabil.

Contohnya seperti `Stack Overflow`, `Quora`, `Reddit` dan sebagainya.

<div class="img-app">
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/stack-overflow.svg" class="img-fluid" alt="stack-overflow"/>
    <p>Stack Overflow</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/quora.svg" class="img-fluid" alt="quora"/>
    <p>Quora</p>
  </div>
  <div class="img-app-item">
    <img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/csr-vs-ssr/static/reddit.svg" class="img-fluid" alt="reddit"/>
    <p>Reddit</p>
  </div>
</div>

</details>

<details open>
<summary><h2>📌 Developer Experience</h2></summary>

Ini adalah penghujung catatan CSR + SSR gue buat ini. Untuk Dev Exp atau pengalaman developer ini beda tools/framework beda cara. Tapi pada intinya disemua framework itu perlu skill dan mainset full-stack developer kalo mau bangun SSR app. Sedangkan untuk CSR app Lo cukup tau dasar - dasarnya seperti HTML, CSS, Javascript DOM, State, Routing, Fetch (Axios atau Fetch API) ini sudah cukup untuk membangun CSR app.

Gue kasih contoh dan implementasi dari 2 framework frontend favorit gue yaitu Svelte dan Leptos.

<h3  class="title-code"> <img src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/svg/skills/svelte.svg" class="img-code" alt="svelte"/> <span>Svelte</span> </h3>

Svelte ini bisa gue bilang framework minimalis tapi lengkap dan gue ga perlu nambah - nambah third party banyak udah bisa bikin aplikasi web yang lumayan compleks. Buat catatan tentang svelte Lo bisa baca di sini bro <a href="https://feri-irawansyah.my.id/catatan/frontend/catatan-ringan-ini-tentang-svelte-frontend-framework-yang-minimalis" target="_blank" rel="noopener noreferrer">Catatan Ringan Ini Tentang Svelte - Frontend Framework Yang Minimalis</a>. 

<h4> Svelte CSR App </h4>

Kalo Lo mau bikin CSR app pake Svelte ada beberapa skil yang harus Lo tau yaitu:

- Javascript (Wajib) karena Svelte menggunakan Javascript
- Javascript DOM
- Typescript (Jika backend di satu project Sveltekit, optional untuk frontend SSR pake Svelte aja)
- Build Tools (Vite, Rollup, Webpack, etc)
- State
- Component
- Rune
- Lifecycle State
- State Management
- Routing (svelte-spa-router atau sveltekit CSR) dan Authorization Routing
- Fetch (Axios atau Fetch API) dan Asynchronous JavaScript
- Skill lain (aksesibilitas, component design, animation, styling, semanitik html dan lain-lain)

<h4> Svelte SSR App </h4>

Kalo Lo mau bikin SSR app pake Svelte Lo wajib punya mainset seorang full-stack developer karena aplikasi frontend Lo udah ga jalan di client lagi, sekarang dia jalan di server. Nah ada beberapa skill lanjutan dari Svelte yang haru Lo tau yaitu:

<h5> 1. Full Web Fundamentals </h5> 

Kalo Lo mau buat SSR app Lo wajib banget punya pemahaman tentang Full Web Development seperti HTML Semantik, CSS, Js (DOM + Async), Http protocol, Cookies, Headers dan Lo juga harus paham tentang data yang akan dikirim ke halaman sebelum sampe ke user.

<h5> 2. Sveltekit (Core) </h5>

Kalo Lo mau buat SSR pake svelte Lo harus paham tentang Sveltekit seperti Routing (File based), Load function (Meskipun bisa di CSR tapi beda perilaku), Server Action, Streaming SSR dan Error Handling.

```js
// src/routes/+page.server.js
export async function load() {

  const res = await fetch('https://jsonplaceholder.typicode.com/posts');

  if (res.ok) {

    const data = await res.json();
    return { posts: data };
  }

  throw error(404, 'Not Found');
}
```

Seperti contohnya load function ini. Load function akan di jalankan sebelum page ditampilkan ke browser.

<h5> 3. Authentication & Authorization </h5>

Di svelte CSR mungkin Lo hanya bisa ngasih batasan user masuk ke halaman tententu. Di svelte SSR Lo harus paham tentang session store dan cookies (httponly). Meskipun menggunakan API terpisah untuk Auth, tapi SSR punya cookies dan http sendiri. Jadi Lo perlu handle session store dan cookies di server apakah bisa join ke API atau tidak. Kalo bisa join Lo juga harus hati - hati jangan sampai aplikasi lain juga bisa mengakses API yang Lo consume di SSR app.

Selain itu perlu juga pemahaman tentang Route security (hooks.server.js) ini adalah semacam magic function di sveltekit yang akan jalan ketika ada request dan sebelum masuk ke route halaman.

```js
// src/hooks.server.js

import { redirect } from '@sveltejs/kit';

export function handle({ event, resolve }) {

  const session = event.locals.session;

  if (!session) {

    throw redirect(302, '/login');
  }
}
```

<h5>4. Deployment & Infrastructure</h5>

Deployment SSR tidak semudah deployment kaya CSR yang tinggal upload static file kemudian jalankan web server. SSR itu tidak di render di browser, tidak ada file html. Svelte SSR itu full javascript. Jadi javacsript akan membuat document html nya kemudian kirim ke browser.

Karena UI atau halaman di render dari server artinya memerlukan http runtime kalo Lo misal build pake nodejs + npm, artinya ketika deployment ke server Lo juga harus install nodejs di server dan kalo resource server Lo kecil ini mahal banget karena nodejs akan menggunakan memory untuk runtime nya. Selain itu Lo nambah resource ke server ketika Lo ada update dependensi atau versi svelte Lo juga harus menyesuaikan compatible atau tidak dengan nodejs yang ada di server.

Alternative lain kalo Lo ga mau install runtime di server Lo bisa pake `Bun` agar bisa compile ke single file executable tapi perlu skill Bun untuk menggunakan nya artinya Lo perlu belajar teknologi lain di luar sveltekit.

Kemudian setelah menjalankan Svelte SSR nanti ada port default misal 3000 nah kemudian Lo perlu reverse proxy kaya Nginx/Caddy agar port 3000 jangan di expose di internet. Kalo Lo pingin lebih scallable Lo bisa run di background pake PM2 atau Docker tapi balik lagi itu membutuhkan resource server yang cukup.

<h5> Management Performance & Streaming SSR </h5>

Kalo di frontend misal Lo pingin nampilin data dari api Lo tinggal fetch => dapet response => lalu buat UI dari data api, sesimple itu. Kalo response time cepat maka UI Lo akan cepat di render, kalo api lambat maka UI akan lambat dirender cara mengatasinya biasanya menggunakan lazy loading atau placeholder. 

Di CSR ga sesederhanya itu ketika load data Lo berbasis page scope kalo response api lama, maka halaman tidak akan muncul sampai api response nya keluar. Untuk mengatasinya Svelte dan framework modern lain menggunakan hydration dan partial render. Jadi ketika load awal halaman akan ditampilkan secara bertahap dari komponent yang tidak menggunakan data dari api kemudian setelah api response nya keluar kemudian akan di render secara bertahap.

Selain itu untuk memaksimalkannya Lo juga perlu paham tentang TTFB (Time to First Byte) optimization artinya Waktu dari user request dikirim sampai byte pertama dari server diterima. Optimisasi ini melibatkan juga performa dari backend yang dipake di frontend seperti:

- Caching server side atau di backend
- Database tuning
- Penggunakan CDN untuk asset besar (jika server tidak ada internat tidak bisa digunakan)
- Lazy data loading
- Jika server backend terpisah dan lokasinya jauh optimasi akan sulit.

<h5> Testing & Debug </h5>

Svelte SSR debuging tidal di UI atau DOM jadi kalo Lu `console.log("halo")` itu akan muncul di CLI aau terminal yang Lo pake, karena halaman bersifat server bukan browser. Selain itu di Svelte SSR tidak ada `window` object karena server tidak tau window di browser.

```js
// src/routes/+page.server.js
export async function load() {

  const res = await fetch('https://jsonplaceholder.typicode.com/posts');

  console.log(res);

  return res.json();
}
```

```bash
# output console

> fetch('https://jsonplaceholder.typicode.com/posts')
Response {url: 'https://jsonplaceholder.typicode.com/posts', status: 200, statusText: 'OK', headers: Headers, type: 'cors', body: ReadableStream, size: 0}
```

Selain itu untuk Unitest kaya `Vitest` atau `Jest` akan beda perilaku, ketika Lo pake unitest untuk frontend CSR maka Vites atau Jest akan melakukan testing Debug UI, Mock API, Component dan Performance Lighthouse. Vites atau Jest akan melakukan testing untuk Debug Server Load, Server logic test, Playwright atau Integration api test, Performance TTFB, Query dan Cache serta Security.

<h3 class="title-code"><img class="img-code" src="https://vjwknqthtunirowwtrvj.supabase.co/storage/v1/object/public/feri-irawansyah.my.id/assets/img/skills/leptos.png" alt="leptos"/> <span>Leptos</span></h3>

Contoh kedua ini rada mainstream karena bikin frontend pake bahasa pemrograman yang terkenal galak😅. Iya Leptos adalah framework frontend yang dibangun untuk ekosistem Rust dan framework frontend yang paling gue suka karena type safety nya bikin gue tidak sembarangan nulisin code jadi kemungkinan bug itu kecil. Leptos ini sebenernya di design khusus untuk SSR. Tapi bisa juga untuk CSR hanya saja fitur partial hydration nya tidak berguna dan jadi aplikasi CSR biasa.

<h4> Leptos CSR App </h4>

Untuk bangun aplikasi pake Leptos CSR Lo bener - bener ga perlu pemahaman tentang Rust backend serius, karena meskipun pada dasarnya Rust itu untuk backend karena berada di low level programming language, tapi Leptos ini bener - bener ngasih Lo jadi Frontend Dev. Nah ada beberapa hal yang perlu dipahami:

- Rust (Wajib) karena ngodingnya pake Rust
- Build Tools (Trunk, Wasm Pack)
- Wasm (Web Assembly)
- Leptos Component
- Signal untuk state
- Lifecycle State
- Fetch (leptos::fetch, leptos::prelude::create_resource, gloo_net)
- State Management
- Routing (leptos_router)
- Skill lain (aksesibilitas, component design, animation, styling, semanitik html dan lain-lain)

Kurang lebih sama kaya Frontend modern prerequisite frontend biasnaya berkaitan dengan build tool, component, state dan routing. Gue juga ada catatan tentang Leptos di sini <a href="https://feri-irawansyah.my.id/catatan/frontend/frontend-web-single-page-aplication-pake-rust-leptos-csr" target="_blank" rel="noopener noreferrer">Frontend Web Single Page Aplication Pake Rust (Leptos CSR)</a>.

<h4> Leptos SSR App </h4>

Sama seperti Svelte SSR, Leptos SSR juga membutuhkan pemahaman tentang Rust backend. Nah ada beberapa hal yang perlu dipahami:

<h5> 1. Full Web Fundamentals </h5>

Seperti Svelte SSR app, Leptos SSR app juga Lo wajib banget punya pemahaman tentang Full Web Development seperti HTML Semantik, CSS, Js (DOM), Http protocol, Cookies, Headers dan Lo juga harus paham tentang data yang akan dikirim ke halaman sebelum sampe ke user. Meskipun Lo nantinya pake Rust tapi pemahaman Js (DOM) ini dibutuhkan, karena yang ngerti DOM cuma javascript.

<h5> 2. Authentication & Authorization </h5>

Auth di Leptos pada dasarnya sama seperti teknologi lainnya. Jika dibandingkan dengan Sveltekit Leptos ini bukan file base routing. Jadi buat authorize suatu halaman di SSR, Lo harus lakukan di backend.

<h5> 3. Deployment & Infrastructure </h5>

Seperti yang gue bilang di awal Leptos ini memang di design untuk SSR (server app) jadi berbeda dengan Sveltekit untuk deployment Leptos SSR ini sedikit lebih murah dibanding CSR. Kok bisa?

<h6> - Leptos compile ke wasm </h6>

Web Asembly (wasm) biasanya ukurannya besar. Default compile pake Trunk build tools tanpa minify bisa 5MB untuk aplikasi hello world aja. Jadi untuk CSR mungkin akan lambat untuk load pertama karen browser akan download file wasm yang besar. Kalo pake leptos SSR server function akan dicompile ke native binary file. Dan binary file Rust ukurannya sangat kecil sehingga akan mengurangi beban di file wasm sehingga ukuran wasm akan jauh lebih kecil karena hanya dipake buat hydration aja bukan untuk runtime.

<h6> - Rust low level programming </h6>

Rust native binary ini native file tidak ada runtime tambahan dan bener - bener murni machine code dan siap di jalankan tanpa runtime. Selain itu Rust tidak punya garbage collector dan tidak membutuhkan garbage collector. Jadi meskipun requestnya banyak tidak ada ada proses stop the world untuk menghapus data di memory. Tapi Rust tidak mudah dipelajari konsep borrow checker dan ownership serta syntax yang sulit di baca menjadi bottleneck terhadap skill issue.

<h6> - Perlu HTTP server </h6>

Leptos bisa digunakan langsung di Rust dengan HTTP server rust/tokio (async runtime rust). Tapi buat integrasinya tidak mudah namun Leptos sudah suport dengan Framework HTTP besar di ekosistem rust terutama Actix dan Axum. Keduanya masih menduduki peringkat 10 besar framework tercepat di Techempower benchmark dunia. Inilah alasan kenapa SSR Leptos itu kuat bahkan cocok untuk game server yang latensi nya tinggi.

<h6> - Partial Hydration tidak mudah digunakan </h6>

Di Leptos ketika load website untuk pertamakalinya, browser akan download aset yang diperlukan termasuk wasm file yang akan digunakan untuk hydration. Leptos punya partial/lazy hydration jadi hydration dilakukan secara paralel ke UI terkecil yang benar-benar ada action atau perubahan. Nah proses hydration ini tidak semudah hydration di javascrpt. Di Leptos ketika semantik HTML tidak sesuai bisa menyebabkan hydrartion gagal. Contohnya seperti membuat tag `<li></li>` tanpa dibungkus `<ul></ul>` atau `<ol></ol>`.

```rs
use leptos::prelude::*;

// Benar ✅
#[component]
fn List() -> impl IntoView {

    view! {
        <ul>
            <li>"Item 1"</li>
            <li>"Item 2"</li>
        </ul>
    }
}

// Salah ❌
#[component]
fn List() -> impl IntoView {

    view! {
        <li>"Item 1"</li>
        <li>"Item 2"</li>
    }
}
```

<h6> - Pake `cargo_leptos` untuk mempermudah hidup </h6>

Ketika membuat CSR Leptos pake Trunk Lo cukup jalankan `trunk server` udah bisa hot reload. Di Leptos SSR tidak ada trunk, karena pake http server. Nah untuk menjalankan project Leptos SSR perlu banyak mantra dan ritual bro. Dengan pake cargo_leptos akan mempermudah hidup Lo.

- CSS Hidration
- Auto Reload
- Build binding asset
- Prerender
- Dx lainnya enak

Itu beberapa keuntungan Lo pake cargo_leptos tanpa cargo_leptos, Lo bakal jadi dukun yang banyak baca mantra dan banyak ritual yang bakal numbalin waktu Lo.

<h6> - Untitest & Debugging</h6>

Belum ada Unitest khusus untuk Leptos, tapi Rust punya unitest bawaan yaitu dengan macro `#[test]`. 

```rs
#[component] // Leptos component
pub fn Counter(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    view! { cx, <button on:click=move |_| set_count.update(|c| *c += 1)>{count}</button> }
}

#[cfg(test)] // unitest
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn counter_initial_value() {
        leptos::create_runtime();
        let cx = leptos::Scope::new();

        let view = Counter(cx).into_view(cx);
        let html = leptos::ssr::render_to_string(cx, move || view.clone());

        assert!(html.contains(">0<")); 
    }
}
```

Lo bisa jalankan perintah
```bash
cargo test --exact counter_initial_value --show-output
```

Dan hasilnya
```bash
running 1 test
test tests::counter_initial_value ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.04s
```

Jika debugging di level component lo bisa jalankan projectnya lalu tambahkan code ini di bagian yang ini Lo debug.

```rs
use leptos::prelude::*;

#[component]
pub fn Counter(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    console_log!("count: {}", count);

    view! { cx, <button on:click=move |_| set_count.update(|c| *c += 1)>{count}</button> }
}
```

Nanti di browser devtools console akan tampil `count: 0` dan `count: 1` ketika klik button. 

</details>

<details open>
<summary><h2>📌 Summary</h2></summary>

CSR (Client Side Render) artinya Lo membuat aplikasi yang berjalan di browser, sedangkan SSR (Server Side Render) artinya Lo membuat aplikasi yang berjalan di server dan dikirim ke browser. 

Development CSR jauh lebih mudah dibandingkan dengan SSR karena CSR hanya berkaitan dengan bagaimana UI itu interactive dan menyesuaikan pengalaman user. Sedangkat SSR selain bagaimana UI itu interactive juga berkaitan dengan bagaimana data itu di render dan dikirim ke client.

CSR dan SSR mempunyai kelebihan dan kekurangan masing-masing. Jadi ketika memilih antara CSR dan SSR, harus mempertimbangkan kebutuhan dan kebutuhan Lo. Semoga bermanfaat.

</details>

---

<div class="d-flex flex-row justify-content-center align-items-center">Regards <a href="https://feri-irawansyah.my.id"><img witdh="1rem" src="https://feri-irawansyah.my.id/favicon.ico" alt="Feri Irawansyah"> Feri Irawansyah</a></div>

---