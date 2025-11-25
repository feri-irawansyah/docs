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
        max-width: 50px;
    }

    .img-app-item {
        display: flex;
        flex-direction: column;
        align-items: center;
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

Analoginya misal Lo minta mobil remote control ke bapak Lo nah karena emang bapak Lo itu tipikal orang CSR jadi dia cuma ngasih duit dan Lo di suruh beli sendiri. Sedangkan kalo bapak Lo itu tipikal SSR nah dia langsung ngasih mobil mainannya ke Lo bro. Kerja bro biar bisa beli mobil mevah kek Lambo.
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
<summary><h2>📌Performance & Biaya</h2></summary>

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
<summary><h2>📌Security</h2></summary>

Keamanan ? Kalo di lihat dari implementasinya SSR terasa lebih aman karena datanya tersentralisasi di server dan tidak ada data di browser. Gue breakdown beberapa masalah security CSR dan SSR.

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

Tapi balik lagi bro, security itu bukan di frontend tapi di backend dan server Lo. Frontend hanya UI dan template bukan bussines logic. 
1. Bussines logic tetap ada di backend
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

</details>

<details open>
<summary><h2>📌 Contoh Aplikasi CSR dan SSR</h2></summary>

Jadi ga semua aplikasi itu harus di pukul rata **POKOKNYA HARUS SSR** atau **POKOKNYA HARUS CSR** pakelah teknologi sesuai kebutuhan dan diskusi dengan tim untuk menentukan keputusan bersama. Mungkin kalo misal Lo bikin aplikasi sendiri itu terserah Lo bro, mau pake apa aja itu terserah Lo. Tapi kalo Lo kerja secara tim di perusahaan atau ada project dimana yang mengerjakan bukan Lo dan Ego Lo sendiri, itu harus dengan kesepakatan bersama.

<h3>Category Aplikasi CSR</h3>

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

<h3>Category Aplikasi SSR</h3>

SSR biasanya digunakan untuk aplikasi yang cendering memiliki kontent statis, event handler sedikit, branding dan sebagainya.

<h4> Online Store (Product Page)</h4>

Seperti yang gue bahas sebelumnya pada website online store itu menggunakan hybrid yaitu CSR dan SSR. Khususnya pada product page karena product page membutuhkan SEO untuk pemasaran product. 

Contohnya seperti `Amazon`, `Shopee`, `Tokopedia`, `Blibli` dan sebagainya.

<h4> News & Media </h4>

Aplikasi berita ini banyak menggunakan SSR bahkan seperti diharuskan kenapa? Karena berita ini bersifat text content dan harus sesegera mungkin dipublikasikan ke halayak umum. Jadi memerlukan performa SEO yang tinggi untuk memasarkan konten makanya menggunakan SSR sebagai renderingnya.

Contohnya seperti `Wikipedia`, `Detik`, `Kompas` dan sebagainya.

<h4>Blog</h4>

Aplikasi blog ini mirip seperti website news yaitu memuat konten text yang panjang bedanya secara penyampaian kontennya. Kontent blog lebih ke personal notes yaitu catatan dan opini pribadi seseorang bukan suatu berita yang ada di internet. Nah aplikasi yang merender kontent text ini memerlukan SEO juga agar dapat di crawl oleh `google bot` atau `bing` agar mudah dicari dicari di mesin pencarian.

Contohnya seperti `Medium`, `Hashnode`, `Dev.to` dan sebagainya.

<h4> Corporate & Company Profile </h4>

Aplikasi company profile biasanya memuat konten statis berupa informasi perusahaan seperti deskripsi, kontak, laporan keuangan atau CSR (Corporate Social Responsibility) dan sebagainya. Company profile juga membantu menjelaskan identitas, visi misi, produk/layanan, dan pencapaian perusahaan kepada klien, investor, dan calon karyawan. Namun untuk website company profile tidak sekompleks seperti website news dan blog. Ada juga beberapa website company profile yang menggunakan static html render karena hanya sebagai identitas aja ga perlu database, state management, atau bahkan backend.

Contohnya seperti `Google`, `Microsoft`, `Apple` dan sebagainya.

<h4> Forum / Community </h4>

Forum - forum diskusi online yang publik biasanya juga menggunakan SSR sebagai rendering nya karena memuat konten text argument dan komentar yang membutuhkan SEO. Biasanya pada website forum ini tidak menggunakan realtime update atau koneksi dua arah. Kenapa? Karena forum diskusi ini bersifat public discussion dan disediakan oleh community non provit yang tidak memiliki sumber untuk menyediakan server yang besar untuk menghandle realtime update. Jadi penyedia hanya menggunakan SSR sebagai renderingnya + backend 1 arah agar tetep mendapatkan SEO yang baik tapi performa stabil.

Contohnya seperti `Stack Overflow`, `Quora`, `Reddit` dan sebagainya.

</details>