<style>
    @media screen and (min-width: 768px) {
        img[alt="ssr-dashboard"] {
        width: 70% !important;
        }
    }
</style>

Di jaman modern sekarang banyak framework untuk membuat aplikasi khusuanya website. Kalo Lo pasukan king PHP ada Laravel + Livewire, kalo Lo pasukan Java ada Springboot atau kalo Lo pasukan kepiting yang demen di omelin compiler ada Leptos dan masih banyak lagi. 

Di catatan gue kali ini gue mau bahas tentang salah satu framework didunia Java Script yaitu pasukan anak yang hype, egie dan frameworker yang tiap hari kek ada aja gitu. Framework yang akan gue bahas yaitu `Sveltekit`. Catatan ini adalah lanjutan dari [Catatan Ini Tentang Svelte Frontend Library Yang Minimalis](https://feri-irawansyah.my.id/catatan/frontend/catatan-ringan-ini-tentang-svelte-frontend-framework-yang-minimalis). Dimana sekarang Lo bakal baca tulisan gue yang suka typo ini yang membahas tentang Framework dari Svelte. Karena di catatan sebelumnya pernah gue bahas **Framework itu perlu banget kalo Lo kerja secara tim, biar ga ada yang asal nulisin kode - kode nuklir yang bisa bikin aplikasi Lo meledak dan bug jadi numpuk kaya utang Lo**. Lo Bisa kunjungi documentasinya Sveltekit disini

- Sveltekit <a href="https://svelte.dev/docs/kit" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit</a>.

Tapi sebelum Lo baca kebawah gue merekomendasikan buat Lo baca dulu catatan tentang Svelte biar ga kaya bocah TK di ajak main biliar. Dan juga mingkin Lo perlu baca catatan gue tentang [Pilih CSR (Client Side Render) Atau SSR (Server Side Render) Untuk Website?](https://feri-irawansyah.my.id/catatan/frontend/pilih-csr-client-side-render-atau-server-side-render-untuk-frontend). Karena pada catatan tentang Sveltekit ini bakal membahas tentang CSR dan SSR. Tapi kalo Lo udah paham dan gue rasa Lo juga udah jago skip aja atau dari pada Lo baca tulisan gue mending chattingan sama gebetan.

<details>
<summary><h2>Kenalan Sama Sveltekit 📚</h2></summary>

Sebelumnya gue bilang Sveltekit adalah framework dari Svelte. Iyes bro karena di Svelte ini Lo udah buatin fitur - fitur sakti untuk bikin website dan udah ada architecturenya. Jadi Lo dan tim Lo ga bisa bikin website tanpa overthingking dan adu argument, adu ketangkasan, adu mekanik sampe gelut dah gelut wkwkwk.

### Fitur - Fitur Sveltekit

Sveltekit punya beberapa fitur yang yang bisa Lo pake pas bikin website:
- Routing
- Server Side Render (SSR)
- Data Fetching
- Pre-rendering
- Single Page Application (CSR)
- Library packaging
- Deployment
- Testing
- Dan lain - lain.

### Architecture Sveltekit

Nah ini nih yang paling cakep architecture yang bikin Lo dan tim kaga adu mekanik pas awal bikin project. Lo bisa baca lebih lengkapnya disini. 

- Architecture Sveltekit <a href="https://svelte.dev/docs/kit/project-structure" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/project-structure</a>.

Atau kalo Lo males baca dokumentasinya gue jelasin deh.

```bash
my-project/
├ src/
│ ├ lib/
│ │ ├ server/
│ │ │ └ [code server biasanya buat db]
│ │ └ [Bisa Lo taro component - componentnya disini]
│ ├ params/
│ │ └ [Parameter dari URL]
│ ├ routes/
│ │ └ [Routing di Sveltekit]
│ ├ app.html
│ ├ error.html
│ ├ hooks.client.js
│ ├ hooks.server.js
│ ├ service-worker.js
│ └ tracing.server.js
├ static/
│ └ [static assets]
├ tests/
│ └ [ Ini buat unitest tests]
├ package.json
├ svelte.config.js
├ tsconfig.json
└ vite.config.js
```

Ini architecture lengkapnya tapi ga semuanya kepake kok, sesuaikan lagi sama kebutuhan Lo bro.

#### Folter `src`

Ini folder utama project Sveltekit disini Lo nanti ngoding di folder ini. Kaya mau bikin component, Rune, State Management, Routing, dll.

##### Folder `src/lib`

Ini folder buat Lo naro logic business di Sveltekit.

- `src/lib/server` Disini Lo bisa naro backend connectdb, filesystem, auth logic dll. Aman ga bakal bocor ke client.
- `src/lib/folder-apa-aja` Disini Lo bisa naro component, state management, dll, dll misal `src/lib/components`.

##### Folder `src/params`

Disini Lo bisa naro buat ambil parameter misalnya Lo bikin url nya `http://localhost:3000/user/feri-irawansyah` maka `src/params/username` bisa naro `feri-irawansyah`. Nanti kita coba implementasinya karena ada opsi lainnya juga buat nangkep parameter.

##### Folder `src/routes`

Disinilah Sveltekit Off Heart. Karena disini Lo bisa bikin halaman web cuma tinggal bikin folder + file dengan nama `+page.svelte` Lo isikan tag html 1 biji langsung tampil. Dan banyak lagi gitur di folder sakti ini.

##### File `src/app.html`

Ini template utama jadi sebenernya website Lo ini cuma 1 halaman. Tapi tergantung nanti Type Project nya apa Sveltekit yang urus buat compilasinya. Enak? Enak lah udah dibikinin sama bapak <a href="https://x.com/rich_harris" target="_blank">Rich Harris</a> dkk. Isinya kek gini doang.

```html
<!doctype html>
<html lang="en" >
	<head>
		<meta charset="utf-8" />
		<meta name="viewport" content="width=device-width, initial-scale=1" />
		%sveltekit.head%
	</head>
	<body data-sveltekit-preload-data="hover">
		<div style="display: contents">%sveltekit.body%</div>
	</body>
</html>
```

Bagian `%sveltekit.head%` disini buat naro tag - tag yang ada di head kek `link`, metadata, dll. Dan bagian `%sveltekit.body%` disini buat naro tag - tag yang ada di body kek `script`, `style`, `noscript`, dll.

##### File `src/error.html`

Ini buat naro fallback HTML error misalnya file JS gagal di load, server error, ya terkait error pokoknya, jadi misalnya sveltenya bisa compile tapi pas di prod ternyata error karena Lo kurang teliti nanti di `error.html` ini bakal handle.

##### File `src/hooks.client.js`

Ini buat global client lifecycle hooks misal Lo mau redirect, buat analytics atau logging.

##### File `src/hooks.server.js`

Ini buat global server middleware namanya mirip kaya `hooks.client.js` tapi buat server lifecycle hooks. Jadi file ini cuma jalan di server jadi codenya ga bakal keliatan di browser. Lo bisa naro auth, session, inject user, rate limit dll.

##### File `src/service-worker.js`

Ini buat service worker. Kaya misal Lo pingin bikin Progressive Web App (PWA) atau Offline App, atau cache assets. Bisa Lo register manual atau via adapter. Nanti bakal gue jelasin apa itu adapter.

##### File `src/tracing.server.js`

Ini buat tracing & observability (Advanced Level Bro). Misalnya Lo pingin bikin monitoring, Performance Trace, Request Timeing, OpenTelemetry, dll. Tapi ini lebih khusus kalo Lo bikin fullstack web. Karena ini lebih ke server side.

##### Folder `static`

Disini Lo bisa bikin static assets misalnya gambar, robots.txt, manifest.json, web.config (misal Lo deploy di iis), dll. Intinya disini buat static file yang ga bakal di optimisasi. Disini defaultnya akan langsung ke root url website Lo. misal ada file `static/img/avatar.png` maka Lo bisa akses `http://localhost:3000/img/avatar.png` langsung.

##### Folder `tests`

Disini Lo bisa bikin unit test, integration test, end to end test, dll. Biasanya pake Playwright, Jest atau Vites.

##### File `package.json`

Tempat dimana Lo control folder dependensi yang gede banget kek dosa `node_modules`. Dan di sini juga Lo bisa bikin script yang bisa Lo eksekusi pake nodejs dan npm.

##### File `svelte.config.js`

Disini Lo bisa bikin config pake svelte. Misalnya config css, preprocessor, adapter, dll.

##### File `tsconfig.json` atau `jsconfig.json`

Di era modern ini project javascript lebih direkomendasikan pake typescript karena ada type safety nya. Tapi Lo tetep bisa bikin pake Javascript juga nanti filenya akan berubah jadi `jsconfig.json`.

##### File `vite.config.js`

Yang terakhir adalah file configurasi buat Vite Js yaitu module bundler biar code - code Svelte yang Lo tulis itu bisa di baca sama browser dan pas Lo lagi mode development disini juga konfigurasi Server local Lu. Jadi misal Lo mau connect ke api external dan api nya ternyata ga bisa karena di block sama CORS (Cross-Origin Resource Sharing) maka disini bakal ada konfigurasi sebagai proxy.

### Project Type

Selain architecture Lo juga bisa buat banyak macem aplikasi di Svelte fullstack, Single Page Application, Offline App, Mobile App (Pake Tauri/Capacitor), Desktop App (Pake Tauri, Wails, Electron) dll Lo bisa baca lebih lengkapnya disini. 

- Project Type Sveltekit <a href="https://svelte.dev/docs/kit/project-types" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/project-types</a>.

Di catatan ini gue mau fokus ke project type Single Page Application (SPA/CSR) dan project type Server Side Render (SSR). Tapi kalo Lo pingin buat project type lainnya Lo bisa baca salah satu catatan gue yang ini [Bikin Mobile & Desktop App Yang Bau Kepiting (Rust 🦀) Dengan Tauri + Svelte](https://feri-irawansyah.my.id/catatan/frontend/bikin-mobile-desktop-app-yang-bau-kepiting-rust-dengan-tauri-svelte).

### Web Standard

Karena Sveltekit ini bisa bikin fullstack web, jadi Sveltekit sudah lengkap untuk standar - standar website kaya fetch, request, response, cookies, headers, dll. Lo bisa baca lebih lengkapnya disini.

- Web Standard Sveltekit <a href="https://svelte.dev/docs/kit/web-standards" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/web-standards</a>.

</details>

<details>

<summary><h2>Get Started Sveltekit 📚</h2></summary>

Lo udah capek belum baca pembukaan gue yang panjang ? Sama gue juga capek. Jadi yaudah ngoding langsung aja deh dari pada nanti makin banyak typo. Btw gue pake nodejs dengan versi 22 jadi Lo boleh nyesuaiin versinya biar kalo ada konfigurasi bisa sesuai sama catatan ini.

Untuk membuat project Sveltekit Lo cukup buka terminal dan arahkan ke folder mana Lo pingin nyimpen aplikasi dan ketikkan

```bash
npx sv create sveltekit-app
```

1. Nanti Lo bakal ada wizard CLI buat memilih konfigurasi project Sveltekit Lo.

```bash
F:\project>npx sv create sveltekit-app

T  Welcome to the Svelte CLI! (v0.11.1)
|
*  Which template would you like?
|  > SvelteKit minimal (barebones scaffolding for your new app)
|    SvelteKit demo
|    Svelte library
—
```	

2. Kemudian bakal dikasih pilihan mau pake bahasa apa.

```bash
*  Add type checking with TypeScript?
|    Yes, using TypeScript syntax
|    Yes, using JavaScript with JSDoc comments
|  > No
—
```

3. Disini ada beberapa konfigurasi yang bisa Lo pilih.

```bash
*  What would you like to add to your project? (use arrow keys / space bar)
|  [•] prettier (formatter - https://prettier.io)
|  [ ] eslint
|  [ ] vitest
|  [ ] playwright
|  [+] tailwindcss
|  [ ] sveltekit-adapter
|  [ ] devtools-json
|  [ ] drizzle
|  [ ] lucia
|  [ ] mdsvex
|  [ ] paraglide
|  [ ] storybook
|  [ ] mcp
—
```

Kalo gue cuma ada pake <a href="https://tailwindcss.com/" target="_blank" rel="noopener noreferrer">Tailwind CSS</a> untuk styleling biar ga ribet nulisin CSS dan tinggal pake utility nya.

4. Kalo Lo pilih tailwind nanti akan ada pilihan gini gue pilih semua

```bash
*  Which plugins would you like to add?
|  [+] typography (@tailwindcss/typography)
|  [+] forms
—
```

5. Project berhasil dibuat dan menambahkan tailwindcss sekarang tinggal pilih package manager dan mau dibuat diatas runtime apa, karena Sveltekit ini adalah framework fullstack. Disini gue pilih npm artinya gue pake runtime nodejs.

```bash
*  Project created
|
*  Successfully setup add-ons: tailwindcss
|
*  Which package manager do you want to install dependencies with?
|    None
|  > npm
|    yarn
|    pnpm
|    bun
|    deno
—
```

6. Selamat project Sveltekit Lo udah jadi

```bash
*  Successfully installed dependencies with npm
|
o  What's next? -------------------------------+
|                                              |
|  📁 Project steps                            |
|                                              |
|    1: cd sveltekit-app                       |
|    2: npm run dev -- --open                  |
|                                              |
|  To close the dev server, hit Ctrl-C         |
|                                              |
|  Stuck? Visit us at https://svelte.dev/chat  |
|                                              |
+----------------------------------------------+
|
—  You're all set!
```

Kalo udah tinggal masuk ke foldernya lalu jalanin `npm run dev -- --open` lalu ketikkan `localhost:5173` di browser. Nanti akan tampil halaman awal Sveltekit.

<img class="img-fluid" alt="svelte-app" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/sveltekit-app.png" />

</details>

<details>

<summary><h2>Routing 📚</h2></summary>

Kalo Lo pilih yang Sveltekit minimal nanti Lo akan dikasih folder `src/routes` yang didalamnya ada file `+layout.svelte` dan file `+page.svelte`. Sedangkan `layout.css` ini buat naro directive Tailwind CSS biarin aja jangan diubah. Dan Lo tau bro folder `routes` ini adalah folder keramat, sakral dan paling penting. Disilah Sveltekit bakal menyediakan ritual - ritual sakti yang bisa bikin Lo kena pelet karena simplifynya.

### Pages (Halaman)

Di Sveltekit untuk membuat routing ini menggunakan filesystem-based router. Artinya path atau url akan dipetakkan dalam filesystem. Misalnya Lo pingin bikin halaman About `localhost:5173/about` maka Lo perlu membuat folder `about` lalu bikin file `+page.svelte` didalamnya.

```html
<!-- src/routes/about/+page.svelte -->
<h1>About</h1>
```

<img class="img-fluid" alt="about" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/about.png" />

Begitu juga untuk halaman Home `localhost:5173/` artinya folder `routes adalah main page` dan file `+page.svelte` didalamnya. Misalnya Lo pingin membuat halaman Home `localhost:5173/about/profile` maka Lo perlu bikin folder `profile` lalu bikin file `+page.svelte` didalam folder `about`.

### Navigation

Ketika pertama kalo path atau URL diakses via browser, maka Sveltekit akan melakukan SSR (Server Side Render). Tapi pas Lo pindah halaman Sveltekit akan melakukan CSR (Client Side Render). Artinya ketika Lo pindah halaman browser ga akan reload.

Sveltekit ini ga pake component khusus buat navigasi antar halaman, Lo bisa tetep pake element ancor `<a>`. Kalo misalnya Lo pake React (NextJs) atau Vue (Nuxt) maka biar dapet efek navigasi CSR Lo perlu pake component `<Link />`.

```html
<!-- src/routes/+layout.svelte -->
<h1 class="text-3xl font-bold">Home</h1>
<a class="underline text-blue-500" href="/about">About</a>
```

```html
<!-- src/routes/about/+page.svelte -->
<h1 class="text-3xl font-bold">About</h1>
<a class="underline text-blue-500" href="/">Home</a>
```

Selain itu Sveltekit juga menyediakan function untuk redirect mirip kaya windows.location.href. Namanya adalah `goto` atau `redirect`.

- `goto('/about')` ini dipake di CSR
- `redirect('/about')` ini dipake di SSR

### Layout

#### Layout === Parent

Sebelumnya pas Lo bikin project Sveltekit Lo dikasih file `+layout.svelte` ini buat apa? Ini adalah layout suatu template yang akan membungkus halaman Lo. Misal nih Lo pingin ada halaman Home, About, Contact. Nah yang seelumnya Lo lakuin adalah membuat tag `<a>` di setiap halaman. Kalo Lo makin banyak halaman artinya Lo bakal maintain navigation. Disinilah Layout akan sangat membantu.

Coba sekarang tag `<a>` dihapus lalu pindahkan ke file `+layout.svelte` ini:

```html
<!-- src/routes/+layout.svelte -->
<h1 class="text-3xl font-bold">Home</h1>
```

```html
<!-- src/routes/about/+page.svelte -->
<h1 class="text-3xl font-bold">About</h1>
```

```html
<!-- src/routes/+layout.svelte -->
<script>
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<a class="underline text-blue-500" href="/">Home</a>
<a class="underline text-blue-500" href="/about">About</a>

{@render children()}
```

Saat membuat file `+layout.svelte` secara default Sveltekit akan membuat halaman nya dalam `snippet` ernama `children`. Jadi Lo waji anget menggunakan `@render children()`. agar Lo isa me-render semua halaman yang ada di bawahnya.

<img class="img-fluid" alt="layout" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/layout.png" />

#### Nested Layout

Lo bisa bikin layout dimana aja sesuai kebutuhan Lo. Misalnya di halaman About Lo pingin ada 2 halaman lagi yaitu `/about/profile` dan `/about/wallet`. Maka di folder `about` Lo bisa bikin file baru `+layout.svelte` artinya nanti Layout dari halaman About akan membungkus halaman profile dan wallet dan ada di dalam Layout root juga.

```html
<!-- src/routes/about/+layout.svelte -->
 <script>
    const { children } = $props();
</script>

{@render children()}
<a href="/about/profile" class="text-blue-500 undeline">Profile</a>
<a href="/about/wallet" class="text-blue-500 undeline">Wallet</a>
```

```html
<!-- src/routes/about/profile/+page.svelte -->
<h1 class="text-3xl font-bold">Profile</h1>
```

```html
<!-- src/routes/about/wallet/+page.svelte -->
<h1 class="text-3xl font-bold">Wallet</h1>
```

<div class="row">
    <div class="col-md-6">
		<img class="img-fluid" alt="nested-layout" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/nested-layout.png" />
	</div>
    <div class="col-md-6">
		<img class="img-fluid" alt="about-page" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/about-page.png" />
	</div>
</div>

### Load Data

#### Page Load Data

Sebelumnya di Svelte kalo Lo mau ambil data Lo perlu melakukan `onMount` nah tapi ini perilakunya akan dijalankan setelah component di mount. Kalo untuk halaman mungkin kebutuhan Lo beda. yang Lo butuhin harusnya data diload dulu sebelum halaman tampil. Di Sveltekit ada fitur `load` yang akan dijalankan sebelum halaman dirender.

Function `load` hanya bisa Lo buat di Page dan Layout aja tapi bukan `.svelte` tapi `.js`. Jadi misalnya Lo mau load data artinya Lo harus bikin file baru `+page.js` (Jika sifatnya hanya untuk page itu aja) dan file baru `+layout.js` (Maka data bisa dipake di semua halaman di layout itu).

```js
// src/routes/about/profile/+page.js
export const load = () => {
    return {
        username: 'Feri Irawansyah',
        email: 'feryirawansyah@gmail.com',
        phone: '082323443535',
        address: 'Jl. Raya Mataram No. 11, Mataram, Nusa Tenggara Barat, Indonesia'
    };
};
```

Untuk mengakses datanya Lo bisa ambil dengan parameter data pada `$props`

```html
<!-- src/routes/about/profile/+page.svelte -->
<script>
    const { data } = $props();
</script>

<h1 class="text-3xl font-bold">Profile</h1>

<ul>
    <li>Name: {data.username}</li>
    <li>Email: {data.email}</li>
    <li>Phone: {data.phone}</li>
    <li>Address: {data.address}</li>
</ul>
```

<img class="img-fluid" alt="load-page" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/load-page.png" />

#### Layout Load Data

Sekarang kita coba yang Load data di Layout. Tapi ada beberapa yang perlu Lo perhatiin bro.

- Jika Load data di Layout maka data akan dipake di semua halaman di dalamnya. Artinya jika data sangat besar tidak direkomendasikan karena data akan tetap hidup selama halaman - halaman dan layout tersebut aktif.
- Jika didalam layout terdapat `+page.js` dan memiliki atribut pada object yang sama maka data pada Layout bisa di timpa oleh data pada Page.

```js
// src/routes/about/+layout.js
export const load = () => {
    return {
        username: 'Snake System',
    };
};
```

```html
<!-- src/routes/about/+layout.svelte -->
 <script>
    const { children, data } = $props();
</script>

<h1 class="text-3xl font-bold">Halo {data.username}</h1>
{@render children()}
<a href="/about/profile" class="text-blue-500 undeline">Profile</a>
<a href="/about/wallet" class="text-blue-500 undeline">Wallet</a>
```

<img class="img-fluid" alt="load-layout" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/load-layout.png" />

Ketika berada di `/about/profile` username Snake System ditimpa oleh username Feri Irawansyah. Tapi Kalo Lo pindah ke halaman `/about/wallet` harusnya username tetap Snake System. Karena di halaman `/about/wallet` tidak ada `+page.js` maka akan menggunakan data dari Layout.

#### Invalidate

Svelte secara default akan melakukan cache data Load Function yang sudah dikunjungi. Kaya sebelumnya misal Lo dari home, pergi ke halaman `/about`, nah di about Lo pergi ke `/about/profile` terus pergi ke `about/wallet` dan Lo balik lagi ke `/about/profile`. Karena kedua page itu pake Layout yang sama load function tidak akan di panggil ulang karena Sveltekit mengangkap tidak ada perubahan di layout itu contoh gini. COba Lo tambahin waktu `routes/about/+layout.js`:

```js
// src/routes/about/+layout.js
export const load = (params) => {
    return {
        username: 'Snake System',
        date: new Date()
    };
};
```

Render datenya di `routes/about/+layout.svelte`:

```html
<p>Date: {data.date}</p>
```

Harusnya ketika Lo pindah - pindah antara `/about`, `/about/profile` dan `/about/wallet` `Date` nya ga akan berubah nilainya. Kalo Lo ga mau kaya gitu Lo bisa pake `invalidate` atau `invalidateAll` dari `$app/navigation`.

- `invalidate` Kalo Lo ingin menjalankan kembali semua Load Function yang tergantung ke parameter url di halaman yang sedang aktif  misal `invalidate('/about/profile')`. <a href="https://svelte.dev/docs/kit/$app-navigation#invalidate" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/$app-navigation#invalidate</a>
- `invalidateAll` akan dijalankan di semua halaman yang aktif dimana `invalidateAll` dijalankan. <a href="https://svelte.dev/docs/kit/$app-navigation#invalidateAll" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/$app-navigation#invalidateAll</a>

<div class="row">
    <div class="col-md-6">
		<img class="img-fluid" alt="invalidate-1" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/invalidate-1.png" />
	</div>
    <div class="col-md-6">
		<img class="img-fluid" alt="invalidate-2" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/invalidate-2.png" />
	</div>
</div>

```html
<!-- src/routes/about/wallet/+page.svelte -->
 <script>
  import { invalidateAll } from '$app/navigation';
  import { onMount } from 'svelte';

    const { data } = $props();

    onMount(() => {
        invalidateAll();
    })
</script>

<h1 class="text-3xl font-bold">Wallet {data.username}</h1>
```

### Page Information

Karena routing ini dibuat dengan filesystem Sveltekit menyediakan helper untuk mengambil informasi yang ada di halaman yang sedang aktif bernama `page` didalam `$app/state`. Lo bisa kunjungi dokumentasi nya di <a href="https://svelte.dev/docs/kit/$app-state#page" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/$app-state#page</a>. Lo bisa memanggilnya di file `+page.svelte` atau `+layout.svelte`. 

```html
<!-- src/routes/about/+layout.svelte -->
<script>
    import { page } from '$app/state'; // import page

    const { children, data } = $props();

    $inspect(page); // inspect page
</script>

<h1 class="text-3xl font-bold">Halo {data.username}</h1>
{@render children()}
<a href="/about/profile" class="text-blue-500 undeline">Profile</a>
<a href="/about/wallet" class="text-blue-500 undeline">Wallet</a>
```

<img class="img-fluid" alt="page-information" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/page-information.png" />

Page information ini sifatnya reactive. Jadi ketika Lo pindah halaman maka akan berubah. Nah Sekalian coba kita kasih style tailwind agar lebih enak dilihat dan gue mau navigasinya akan memiliki border bawah jika sedang active.

```html
<!-- src/routes/+layout.svelte -->
<script>
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { page } from '$app/state';

	let { children } = $props();

	const isAactive = (path) => page.url.pathname === path

</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>{page.url.pathname.replace('/', '') ? page.url.pathname.replace('/', '').toLocaleUpperCase() : 'HOME'}</title>
</svelte:head>

<div class="dark min-h-screen bg-gray-800 text-gray-100">
	<nav class="p-4 flex gap-4 border-b border-gray-500 justify-center">
		<a class="text-slate-100 {isAactive('/') ? 'border-b border-blue-400' : ''}" href="/">Home</a>
		<a class="text-slate-100 {isAactive('/about') ? 'border-b border-blue-400' : ''}" href="/about">About</a>
	</nav>

	<main class="p-4">
		{@render children()}
	</main>
</div>
```

<img class="img-fluid" alt="active-navigation" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/active-navigation.png" />

</details>

<details>

<summary><h2>Load Parameter 📚</h2></summary>

Function load itu function special bro di routing Sveltekit. Karena selain dijalankan sebelum halaman tampil, dia juga punya banyak fitur lain seperti URL parameter, fetch, cookies dll. Semuanya bisa Lo akses menambahkan parameter di function `load` seperti berikut.

```js
// src/routes/about/+layout.js
export const load = (params) => {
    console.log(params);
    return {
        username: 'Snake System',
    };
};
```

<img class="img-fluid" alt="load-parameter" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/load-parameter.png">

Isinya kayanya gini, atau Lo bisa kunjungi dokumentasinya disini <a href="https://svelte.dev/docs/kit/@sveltejs-kit#RequestEvent" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/@sveltejs-kit#RequestEvent</a>. Di load function Lo juga bisa ambil informasi dari halaman yang sedang di kunjungi pake `params.url` ini sama kaya `page.url` dari `$app/state`. 

Tapi hati - hati kalo misalnya di load function Lo ada naro request ke api dan Lo panggil `params.url` nanti ketika ada perubahan url load function akan melakukan request lagi ketika Lo pindah halaman. Jadi direkomendarikan kalo hanya untuk memberukan informasi dari halaman yang sedang di kunjungi pake `page.url` saja. `params.url` bisa Lo pake jika ada halaman yang perlu validasi ke backend seperti role permission dan sebagainya.

### Dynamic Parameter

Biasanya ketika Lo bikin website mesti Lo ada bikin url yang dinamis misal nya dihalaman admin dashboard website kampung Lo bikin url `/user/123` atau `/user/rahmat` dimana urlnya bisa di isi id atau username. Di Sveltekit kalo mau bikin fitur itu dengan cara bikil folder `[nama-folder]` misalnya nanti gue mau bikin `/book/[id]` maka Lo bikin folder `book` lalu folder `[id]` kemudian Lo bikin file `+page.svelte` kaya gini.

Untuk mengambilnya Lo bisa ambil dari `params.params` pada parameter di load funtcion. Nah biar ga bingung Lo bisa langsung ditructuring `{params}` di parameter load function. Sekarang coba Lo bikin folder baru namanya book dan folder `[id]`. Ceritanya gue mau bikin halaman untuk menampilkan informasi buku. Dantambahkan navigasinya juga di `routes/+layout.svelte`. Jadi coba Lo buat beberapa file ini.

- Buat folder `book`
- Buat folder `[id]` didalam folder `book`
- Buat file `+page.svelte` didalam folder `book`
- Buat file `+page.svelte` didalam folder `[id]`
- buat file `+page.js` didalam folder `[id]`
- Buat file `book.json` di folder `static/api` ceritanya ini api.

Sebelum itu ubah style tag `main` di `routes/+layout.svelte`

```html
<!-- src/routes/+layout.svelte -->
<main class="p-4 max-w-2xl mx-auto border border-gray-500 rounded-2xl my-3">
	{@render children()}
</main>
```

```json
// static/book.json
[
    {
        "id": 1,
        "title": "Pemrograman PHP",
        "author": "Feri Irawansyah",
        "description": "Belajar dasar - dasar bahasa pemrograman PHP",
        "year": 1954
    },
    {
        "id": 2,
        "title": "Linux dasar",
        "author": "Dede Sukron",
        "description": "Belajar dasar - dasar operating system Linux",
        "year": 1937
    },
    {
        "id": 3,
        "title": "Tutorial React JS",
        "author": "Dede Sukron",
        "description": "Belajar dasar - dasar library React JS",
        "year": 1977
    }
]
```

```html
<!-- src/routes/book/+page.svelte -->
 <script>
  import { onMount } from "svelte";

    let books = $state([]);

    onMount(async () => {
        const response = await fetch('/api/book.json');
        books = await response.json();
    })

</script>

<h1 class="text-3xl font-bold text-center">List of Books</h1>

<div class="grid grid-cols-3 gap-6">
    {#each books as book}
        <a href="/book/{book.id}" class="rounded-xl bg-gray-800 text-white p-4 shadow">
            <img src="https://picsum.photos/seed/{book.id}/200/300"
                class="rounded-lg mb-3 w-full object-cover" alt=""/>
            <h3 class="text-lg font-semibold">{book.title}</h3>
            <p class="text-xs text-gray-500 border-b border-gray-700 py-2">{book.author} ({book.year})</p>
            <p class="text-sm text-gray-300 line-clamp-3">
                {book.description}
            </p>
        </a>
    {/each}
</div>
```

Untuk halaman List Book kaya gini

<img class="img-fluid" alt="book-list" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/book-list.png" />

Sela jutnya 

```js
// src/routes/book/[id]/+page.js
export const load = ({ params }) => {

    const id = params.id

    return {
        id
    };
};
```

```html
<!-- src/routes/book/[id]/+page.svelte -->
<script>
	export let id;
</script>
<script>
    const { data } = $props();
</script>

<h1 class="text-3xl font-bold text-center">Book detail {data.id}</h1>
```

Coba Lo klik salah satu buku di List Book. Nanti di Book Detail bakal kaya gini

<img class="img-fluid" alt="book-1" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/book-1.png" />

Begitu juga kalo Lo klik buku - buku yang lainnya.

### Fetch Request

Di List Book yang udah Lo bikin fetch api di lakukan pake `onMount` artinya fetch dilakukan setelah component dirender. Nah kenapa ga dilakuin di load function aja? Bisa banget dan itu lebih recomended. Tapi karena load function ini dijalankan di mode SSR kemudian datanya baru di CSR, maka ini akan error.

Karena fetch itu adalah `Ajax Request` dan hanya jalan di browser bukan di server. Okeh kita coba dulu. **gue buat pake layout agar datanya bisa diakses sama dynamic page**.

```js
// src/routes/book/+layout.js
export const load = async () => {
    const res = await fetch('/api/book.json')
    const data = await res.json()
    return {
        data
    }
}
```

```html
<!-- src/routes/book/+page.svelte -->
<script>
    const { data } = $props();
</script>

<h1 class="text-3xl font-bold text-center">List of Books</h1>

<div class="grid grid-cols-3 gap-6">
    {#each data.books as book}
        <a href="/book/{book.id}" class="rounded-xl bg-gray-800 text-white p-4 shadow">
            <img src="https://picsum.photos/seed/{book.id}/200/300"
                class="rounded-lg mb-3 w-full object-cover" alt=""/>
            <h3 class="text-lg font-semibold">{book.title}</h3>
            <p class="text-xs text-gray-500 border-b border-gray-700 py-2">{book.author} ({book.year})</p>
            <p class="text-sm text-gray-300 line-clamp-3">
                {book.description}
            </p>
        </a>
    {/each}
</div>
```

<img class="img-fluid" alt="error-fetch" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/error-fetch.png" />

Langsung dapet surat cinta dari Sveltekit. Katanya <span class="text-danger">Cannot call `fetch` eagerly during server-side rendering with relative URL (/api/book.json) — put your `fetch` calls inside `onMount` or a `load` function instead</span> ga boleh call fetch di server-side, Lo cuma bolef fetch pake onMount. Terus gimna?

Sveltekit punya fetch http call sendiri sama kaya fetch biasa tapi Lo ambil dari parameter si load function.

```js
// src/routes/book/+layout.js
export const load = async ({ fetch }) => {
    const res = await fetch('/api/book.json')
    const books = await res.json()
    return {
        books
    }
}
```

### Parent Data

Coba Lo buka pake detail book bro, harusnya tampilannya baru heading dan ada id. Nah terus gimana cara mengambil single datanya?. Mesti dalam bayangan Lo Lo bikin load function terus fetch, tapi gimana cara dapet datanya? apakah perlu fetch ulang? Atau pake onMount aja kan datanya bisa di ambil dari props. Itu bisa - bisa aja. Tapi ada cara yang lebih recomended.

Lo bisa pake `parent` parameter. Jadi data dari layout itu bisa diambil dengan cara menggunakan parameter `parent`. 

```js
// src/routes/book/[id]/+page.js
export const load = async ({ params, parent }) => {
    const id = Number(params.id)
    const books = await parent();
    const book = books.books.find(book => book.id === id);

    return {
        id,
        book
    };
};
```

Kenapa parent perlu await? Karena parent ini typenya promise, tapi Lo bisa aja pake method `then` dan `catch`.

```html
<!-- src/routes/book/[id]/+page.svelte -->
 <script>
    const { data } = $props();
</script>

<div class="flex flex-col">
    <h1 class="text-3xl font-bold text-center">{data.book.title}</h1>
    <p class="text-xs text-gray-500 border-b border-gray-700 py-2 text-center">{data.book.author} ({data.book.year})</p>
    <p class="text-sm text-gray-300 line-clamp-3 text-center">{data.book.description}</p>

    <div class="flex items-center justify-evenly mt-5">
        <a class="bg-slate-500 w-1/6 rounded-md text-center p-1" href="/book">Kembali</a>
        <a href={null} class="cursor-pointer bg-blue-500 w-1/6 rounded-md text-center p-1" onclick={alert("Belum ada bukunya")}>Baca</a>
    </div>
</div>
```

### Error Page

Lo ngeh ga bro misalnya load function ada error atau halaman error Sveltekit langsung ngasih halaman error tergantung errornya. Misal halaman ga ada langsung dikasih 404, atau load function ada error langsung dikasih 500. 

Tapi Itu kan punya Sveltekit kadang di lapangan tim design bakal ngasih halaman khusus buat error. Nah di Sveltekit Lo juga bisa custom error pagenya biar errornya ga ambil lagi halaman error default punya Sveltekit.

Cara membuatnya dengan cara membuat file bernama `+error.svelte`. Kalo Lo naronya di folder yang sejajar dengan layout, maka url di bawahnya akan menggunakan halaman error itu. Tapi Lo juga bisa custom perpage agar setiap page punya halaman error sendiri tapi ini hanya berlaku untuk server side.

#### Unexpected Error

Secara default Sveltekit akan menampilkan halaman error 500. Tapi bisa pake file `+error.svelte` untuk custom error page. Ini yang disebut Unexpected error, yaitu error yang terjadi karena ada kesalahan logic atau error code. Sveltekit akan langsung melakukan `throw error(500, message)` ketika terjadi error.

Kalo Lo mau ngehandle error di client side seperti error load function `+page.js` atau `+layout.js` Lo hanya di perbolehkan bikin 1 file error saja di `routes/+error.svelte`

```html
<!-- routes/+error.svelte -->
 <script>
	import { page } from '$app/state';
</script>

<h1 class="text-center text-4xl text-red-500">{page.error.message}</h1>
```

Coba Lo typoin bagian `const res = await fetch('/api/book.json')` di file `routes/book/+layout.js` dan buka halaman `/book`

```js
export const load = async ({ fetch }) => {
    // const res = await fetch('/api/book.json')
    const res = await fetch('/api/boo.json')
    const books = await res.json()
    return {
        books
    }
}
```

<img class="img-fluid" alt="error-page" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/error-page.png" />

#### Expected Error

Tapi Lo juga bisa custom errornya misalnya ketika Lo melakukan render data tapi datanya tidak ditemukan Lo bisa `thow error(404, message)` di file `+page.js` atau `+page.server.js`. Caranya masih sama dengan sebelumnya dimana Lo tinggal pake component `+error.svelte` dan diisi dengan message errornya.

</details>

<details>

<summary><h2>Server Load Parameter 📚</h2></summary>

Server Load Parameter ini mirip - mirip dengan Load Parameter bedanya jika Load parameter meskipun dia dijalankan dimode SSR tapi dia tetep bisa diakses di Client karena akan diteruskan di mode CSR. Sedangkan Server Load Function bener - bener dijalankan di Server Side dan data hanya datanya aja yang di berikan ke client.

Oleh karena itu Server Load Function ini memiliki beberapa perbedaan parameter seperti cookies, setHeader dll. Lebih lengkapnya Lo bisa kunjungi dokumentasi <a href="https://svelte.dev/docs/kit/@sveltejs-kit#RequestEvent" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/@sveltejs-kit#RequestEvent</a>.

Perbedaan lainnya adalah pada nama filenya. Server Load Function menggunakan nama file `.server.js` sedangkan Load Function menggunakan nama file `.js` aja. Okeh ceritanya gue mau bikin halaman dashboard dimana halaman itu hanya bisa diakses oleh user yang sudah login.

1. Pertama buat halaman baru /login

```html
<!-- src/routes/login/+page.svelte -->
<form action="/login" method="GET" class="max-w-sm mx-auto p-6 bg-zinc-900 rounded-xl shadow-md space-y-4">
    <h1 class="text-2xl font-semibold text-center text-gray-800 dark:text-gray-100">
        Login
    </h1>

    <input
        type="text"
        name="username"
        placeholder="Username"
        class="w-full px-4 py-2 border rounded-lg
               border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none
               dark:bg-zinc-800 dark:border-zinc-700 dark:text-white"
    />

    <button
        type="submit"
        class="w-full py-2 rounded-lg bg-blue-600 text-white font-medium
               hover:bg-blue-700 transition"
    >
        Login
    </button>
</form>
```

2. Buat Load Server Function di halaman /login

```js
// src/routes/login/+page.server.js
import { redirect } from '@sveltejs/kit';

export async function load({ cookies, url }) {
    const username = url.searchParams.get('username');
    if (username) {
        cookies.set('username', username, { path: '/' });
        redirect(303, '/dashboard');
    }

    if(cookies.get('username')) redirect(303, '/dashboard');

    return {};
}
```

3. Buat halaman dashboard

```html
<!-- src/routes/dashboard/+page.svelte -->
 <script>
    const { data } = $props();
</script>

<h1 class="text-3xl font-bold">Hello {data.username}</h1>

<a href="/dashboard?logout=true">Logout</a>
```

4. Buat Load Server Function di halaman /dashboard

```js
// src/routes/dashboard/+page.server.js
import { redirect } from '@sveltejs/kit';

export async function load({ cookies, url }) {
    
    if (url.searchParams.get('logout')) {
        cookies.delete('username', username, { path: '/' });
        redirect(303, '/login');
    }

    if(!cookies.get('username')) redirect(303, '/login');

    return {
        username: cookies.get('username')
    };
}
```

Ini gue set pake method `GET` dulu gpp karena kita masih coba buat pake load server function, nanti kita perbaiki sambil jalan. Kalo udah coba Lo ketik `/dashboard` di address bar harusnya nanti langsung redirect ke halaman Login lagi. Karena di Devtools belum ada cookies dengan key username.

<img class="img-fluid" alt="login" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/login.png" />

Coba Lo isikan username lalu enter atau klik login. Nanti Lo akan langsung redirect ke halaman dashboard dan kalo Lo cek devtools => cookies nanti ada key username yang isinya adalah yang Lo masukin.

<img class="img-fluid" alt="dashboard" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/dashboard.png" />

Sekarang coba Lo pindah halaman ke `/login` harsunya Lo akan di redirect ke `/dashboard` lagi karena cookies sudah ada. Kalo Lo mau ke halaman login Lo harus logout dulu.

</details>

<details>

<summary><h2>REST API Route 📚</h2></summary>

Type default Sveltekit adalah SSR (Server Side Render), artinya halaman akan di render di Server. Jadi Lo juga bisa bikin REST Api di Svelte. Cara membuatnya masih tetep di folder `routes`

- Syarat membuat API di Sveltekit harus menggunakan file `+server.js`.
- Untuk endpointnya menggunakan nama folder dimana file `+server.js` itu berada.
- Nama function ditentukan dengan jenis Http Method yang digunakan, misal GET, POST, PUT, PATCH, DELETE, OPTIONS dan HEAD
- Parameter tetap menggunakan `RequestEvent` <a href="https://svelte.dev/docs/kit/@sveltejs-kit#RequestEvent" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/@sveltejs-kit#RequestEvent</a>
- Response sesuai dengan standarisasi REST Api <a href="https://developer.mozilla.org/en-US/docs/Web/API/Response" target="_blank" rel="noopener noreferrer">https://developer.mozilla.org/en-US/docs/Web/API/Response</a>
- Api Route berjalan di Server bukan di client. jadi kalo Lo buat SPA (Single Page Application) api ga bisa jalan.

Sebelumnya untuk login masih menggunakan query parameter dan mpake http GET buat melakukan request. Lo mesti tau lah ya ini bahaya karena Lo bisa input dari url. Nah sebelum itu gue mau setup DB di Sveltekit. Gue mau pake PostgreSQL buat studi kasus Perpustakaan Online. Kalo Lo mau pake DBMS lain juga bisa bro atau kalo Lo mau ngikutin dan belum ada PostgreSQL Lo bisa baca catatan gue yang ini [Postgres SQL Playground: Bermain bersama Query dan Kawan - kawan](https://feri-irawansyah.my.id/catatan/backend/postgres-sql-playground-bermain-bersama-query-dan-kawan-kawan).

### Server Only Modules

Sebelumnya di architecture Sveltekit ada folder khusus yang hanya jalan diserver yaitu folder `lib/server`. Lebih lengkapnya Lo bisa baca dokumentasi <a href="https://svelte.dev/docs/kit/server-only-modules" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/server-only-modules</a>. Sekarang coba Lo buat folder server di folder lib dan buat file `connection.js`.

#### Create Connection Pool

Untuk membuat koneksi ke postgres perlu menambahkan dependencies. Disini gue pake `pg` lebih lengkapnya Lo  bisa baca dokumentasinya <a href="https://www.npmjs.com/package/pg" target="_blank" rel="noopener noreferrer">https://www.npmjs.com/package/pg</a>.

```bash
npm install pg
```

Gue anggep Lo udah punya dan udah jago lah ya soal DBMS nah gue ada bikin database namanya books. Buat table users nya ini kaya gini schemanya.

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    fullname VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO users (username, fullname, password, email) VALUES
    ('admin', 'Admin', 'password', 'admin@example.com');
```

Ini file `connection.js` ini.

```js
// src/lib/server/connection.js
import { Pool } from 'pg';

const connection = new Pool({
    connectionString: "postgres://postgres:postgress@localhost:5432/books", 
    max: 20, 
    idleTimeoutMillis: 30000, 
});

export default connection;
```

#### Test Connection

Coba Lo tes koneksinya dulu, coba aja di file `routes/login/+page.server.js` masukin kode ini.

```js
// src/routes/login/+page.server.js
import connection from '$lib/server/connection.js';
import { redirect } from '@sveltejs/kit';

export async function load({ cookies, url }) {
    
    const username = url.searchParams.get('username');
    if (username) {
        cookies.set('username', username, { path: '/' });
        redirect(303, '/dashboard');
    }

    if(cookies.get('username')) redirect(303, '/dashboard');

    // Test koneksinya
    try {
        const result = await connection.query('SELECT * FROM users;');
        console.log(result.rows);
    } catch (error) {
        console.error('Database error:', error);
    }

    return {};
}
```

```bash
  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
  ➜  press h + enter to show help
[
  {
    id: 1,
    username: 'admin',
    fullname: 'Admin',
    password: 'password',
    email: 'admin@example.com',
    created_at: 2025-12-31T00:47:45.964Z
  }
]
```

Harusnya meskipun pake console.log tapi outputnya ga akan tampil do browser. Tapi ada di command line dimana Lo jalanin Sveltekit nya. Ini karena file `connection.js` dan `+page.server.js` itu jalan di server, bukan di client. Jadi ini aman.

#### Private environment variables

Connection String di tulis atau di hardcode di file `connection.js`, ini tidak direkomendasikan malah kalo menurut pribadi gue ini ga boleh. Karena kalo Lo misalnya nanti dishare ke github, maka koneksinya bakal bocor. Cara paling baik adalah dengan menyimpannya di file yang hanya bisa diakses dalam server dan environtment dimana aplikasi berjalan. Seperti file `.env`, `appsettings.json`, `config.toml`, dll.

Sveltekit punya env private dari `$env/dynamic/private` & env public dari `$env/dynamic/public` environtment variables. Private hanya bisa diakses oleh server side only sedangkan public bisa diakses oleh client side dan server side.

- Private env <a href="https://svelte.dev/docs/kit/$env-dynamic-private" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/$env-dynamic-private</a>
- Public env <a href="https://svelte.dev/docs/kit/$env-dynamic-public" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/$env-dynamic-public</a>

Jadi coba Lo perbaiki file `connection.js` ini. Buat file `.env` di root project. Isi datanya seperti ini.

```bash
DATABASE_URL=postgres://postgres:postgress@localhost:5432/books
```

Lalu Lo tinggal ganti file `connection.js` ini.

```js
// src/lib/server/connection.js
import { env } from '$env/dynamic/private';
import { Pool } from 'pg';

const connection = new Pool({
    connectionString: env.DATABASE_URL, 
    max: 20, 
    idleTimeoutMillis: 30000, 
});

export default connection;
```

### API Route

Sekarang tinggal bikin endpoint API nya. Coba Lo buat folder `api` di folder `routes` ini untuk nyimpen semua api route.

- Buat file `+server.js` didalam folder `api/auth/login`
- Buat file `+server.js` didalam folder `api/auth/session`
- Buat file `+server.js` didalam folder `api/auth/logout`

```js
// src/routes/api/auth/login/+server.js
import connection from '$lib/server/connection';

export const POST = async ({ request, cookies }) => {
    const body = await request.json();
    
    const rows = await connection.query(`SELECT * FROM users WHERE username = '${body.username}' AND password = '${body.password}'`);
    if (rows.rowCount > 0) {
        const users = rows.rows[0];
        cookies.set('users', JSON.stringify(users), { path: '/' });
        return new Response(JSON.stringify(users), { 
            status: 200,
            headers: {
                "content-type": "application/json"
            }
         });
    } else {
        return new Response(JSON.stringify({ error: 'Invalid username or password' }), {
            status: 401,
            headers: {
                "content-type": "application/json"
            }
         });
    }
}
```

```js
// src/routes/api/auth/session/+server.js
export const GET = async ({ cookies }) => {
    const users = cookies.get('users');
    if (users) {
        return new Response(users, { 
            status: 200,
            headers: {
                "content-type": "application/json"
            }
         });
    } else {
        return new Response(JSON.stringify({ error: 'Unauthorized' }), { 
            status: 401,
            headers: {
                "content-type": "application/json"
            }
         });
    }
}
```

```js
// src/routes/api/auth/logout/+server.js
export const DELETE = async ({ cookies }) => {
    cookies.delete('users', { path: '/' })
    return new Response(null, { status: 200 })
}
```

Sekarang Lo coba call lewat postman atau http client lain kaya Talent API kalo via browser.

- Api Login

<img class="img-fluid" alt="api-login" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/api-login.png" />

- Api Session

<img class="img-fluid" alt="api-session" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/api-session.png" />

- Api Logout

<img class="img-fluid" alt="api-logout" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/api-logout.png" />

Harusnya Lo juga aman ya bro, nah sekarang coba kita implementasikan di website kita. Pertama mungkin Lo bisa hapus file `routes/login/+page.server.js` dan `routes/dashboard/+page.server.js` karena akan pake api route.

```html
<!-- src/routes/login/+page.svelte -->
 <script>
    import { goto } from "$app/navigation";

    let user = $state({
        username: '',
        password: ''
    });

    async function login(e) {
        e.preventDefault();
        const response = await fetch('/api/auth/login', {
            method: 'POST',
            body: JSON.stringify(user)
        });

        if(response.ok) {
            await goto('/dashboard');
        }
    }

</script>

<form onsubmit={login} class="max-w-sm mx-auto p-6 bg-zinc-900 rounded-xl shadow-md space-y-4">
    <h1 class="text-2xl font-semibold text-center text-gray-800 dark:text-gray-100">
        Login
    </h1>

    <input
        type="text"
        name="username"
        bind:value={user.username}
        placeholder="Username"
        class="w-full px-4 py-2 border rounded-lg
               border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none
               dark:bg-zinc-800 dark:border-zinc-700 dark:text-white"
    />

    <input
        type="password"
        name="password"
        bind:value={user.password}
        placeholder="Password"
        class="w-full px-4 py-2 border rounded-lg
               border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none
               dark:bg-zinc-800 dark:border-zinc-700 dark:text-white"
    />

    <button
        type="submit"
        class="w-full py-2 rounded-lg bg-blue-600 text-white font-medium
               hover:bg-blue-700 transition"
    >
        Login
    </button>
</form>
```

Sekarang Lo hapus bagian action dan method di form nya, jadi sekarang Lo akan request ke api `/api/auth/login` untuk login.

<img class="img-fluid" alt="login-api" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/login-api.png" />

Untuk halaman dashboardnya sekarang gini.

```html
<script>
    import { goto } from "$app/navigation";

    async function session() {
        const response = await fetch('/api/auth/session');
        if(response.ok) {
            const data = await response.json();
            return data;
        } else {
            await goto('/login');
        }
    }

    async function logout() {
        const response = await fetch('/api/auth/logout', {
            method: 'DELETE'
        });
        if(response.ok) {
            await goto('/login');
        }
    }
</script>

{#await session()}
    <p class="text-center text-white bg-amber-400">Loading ...</p>
{:then data} 
    <h1 class="text-2xl font-bold">Hello {data.username}</h1>
    <a href={null} class="cursor-pointer" onclick={logout}>Logout</a>
{/await}
```

- Pertama coba Lo ganti urlnya jadi `/dashboard` harusnya akan redirect ke login lagi. Karena Lo belum login.
- Tapi sekarang jadi kaya ada screen sebentar halaman dashboard dulu sebelum redirect ke login. Nanti kita perbaiki.
- Kalo Lo udah login nanti harsusnya akan tampil username dan tombol logout.

kalo Lo mau repiin dikit stylenya boleh

```html
<div class="w-full flex flex-col items-center">
    {#await session()}
        <p class="text-center text-white bg-amber-400">Loading ...</p>
    {:then data} 
        <h1 class="text-2xl font-bold border-b">Hello {data.username}</h1>
        <a href={null} class="cursor-pointer" onclick={logout}>Logout</a>
    {/await}
</div>
```

<img class="img-fluid" alt="dashboard-api" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/dashboard-api.png" />

Kalo misalnya Lo belum login terus mencoba akses ke dashboard yang terjadi adalah halaman dashboard tampil dulu terus redirect ke halaman login. Nah kalo kaya gitu user bisa lohat dashboard meskipun hanya sebentar. Caranya benerinnya pake `+page.js`, coba Lo buat file nya di dashboard.

```js
// src/routes/dashboard/+page.js
import { redirect } from '@sveltejs/kit';

export async function load({ fetch }) {
    const response = await fetch('/api/auth/session');
    if(!response.ok) {
        redirect(303, '/login');
    }

    const user = await response.json();
    return {
        user
    };
}
```

```html
<!-- src/routes/dashboard/+page.svelte -->
 <script>
    import { goto } from "$app/navigation";
    const { data } = $props();

    async function logout() {
        const response = await fetch('/api/auth/logout', {
            method: 'DELETE'
        });
        if(response.ok) {
            await goto('/login');
        }
    }
</script>

<div class="w-full flex flex-col items-center">
    <h1 class="text-2xl font-bold border-b">Hello {data.user.fullname}</h1>
    <a href={null} class="cursor-pointer" onclick={logout}>Logout</a>
</div>
```

Harusnya sekarang udah ga bakal keliatan lagi halaman dashboardnya kalo session nya belum ada.

</details>

<details>

<summary><h2>Form Action 📚</h2></summary>

Form action adalah fitur bawaan dari web application secara fungsi sebenernya sama aja kaya Lo misalnya pake fetch api pake http POST. Toh intinya yang penting data ke kirim ke server terus dapat response entah error atau success. Tapi ada beberapa perbedaan bro kurang leih kaya gini.

<div class="table-responsive">
  <table class="table">
    <thead>
      <tr>
        <th scope="col">#</th>
        <th scope="col">Aspect</th>
        <th scope="col">Form Action</th>
        <th scope="col">fetch POST</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <th scope="row">1</th>
        <td>SSR</td>
        <td>✅ native</td>
        <td>❌ manual</td>
      </tr>
      <tr>
        <th scope="row">2</th>
        <td>Progressive</td>
        <td>✅ Yoi</td>
        <td>❌ Tidak</td>
      </tr>
      <tr>
        <th scope="row">3</th>
        <td>Redirect</td>
        <td>✅ auto</td>
        <td>❌ manual (page goto)</td>
      </tr>
      <tr>
        <th scope="row">4</th>
        <td>Cookie/session</td>
        <td>✅ otomatis</td>
        <td>⚠️ hati-hati (kadang perlu credentials)</td>
      </tr>
      <tr>
        <th scope="row">5</th>
        <td>JS mati</td>
        <td>✅ tetap jalan karena document</td>
        <td>❌ mati karena runtime</td>
      </tr>
      <tr>
        <th scope="row">6</th>
        <td>UX SPA</td>
        <td>⚠️ Statis</td>
        <td>✅ Bisa smooth</td>
      </tr>
      <tr>
        <th scope="row">7</th>
        <td>Sveltekit SSR</td>
        <td>⭐⭐⭐⭐⭐</td>
        <td>⭐⭐⭐</td>
      </tr>
      <tr>
        <th scope="row">9</th>
        <td>Security lebih raw</td>
        <td>✅ Aman</td>
        <td>⚠️ Harus di validasi</td>
      </tr>
    </tbody>
  </table>
</div>

Tapi form action di Sveltekit ini hanya bisa dipake di mode SSR, ga isa dipake di mode CSR. Jadi Lo harus hati - hati buat implementasinya. Buat studi kasusnya ceritanya di halaman dashboard kita akan melakukan CRUD untuk list buku.

- Untuk schema tablenya kaya gini

```sql
CREATE TABLE books (
    id INT NOT NULL AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    year INT NOT NULL,
    author VARCHAR(255) NOT NULL,
    PRIMARY KEY (id)
);

INSERT INTO books (title, description, year, author) VALUES
    ('Pemrograman PHP', 'Belajar pemrograman PHP', 2024, 'Feri Irawansyah');
```

Sebelumnya gue mau perbaiki beberapa di halaman dashboard.

- Get session akan gue pindah ke file `routes/dashboard/+page.server.js`.
- Gue buat pake .server karena ini akan dipake buat nampilin list buku.
- Untuk halaman dashboard ada ada update tampilan.

```html
<!-- src/routes/dashboard/+page.svelte -->
 <script>
    import { goto } from '$app/navigation';

    const { data } = $props();

    async function logout() {
        const response = await fetch('/api/auth/logout', {
            method: 'DELETE'
        });
        if(response.ok) {
            await goto('/login');
        }
    }
</script>

<div class="flex justify-between w-full">
    <h1 class="text-2xl font-bold">Hello {data.user.fullname}</h1>
    <a href={null} class="cursor-pointer border border-pink-500 px-3 rounded-2xl bg-pink-700" onclick={logout}>Logout</a>
</div>

<h1 class="text-3xl font-bold text-center my-5">List of Books</h1>
<button type="button" class="border border-sky-300 px-3 mb-3 rounded-2xl bg-sky-700 cursor-pointer">Tambah Buku +</button>

<div class="grid grid-cols-3 gap-6">
    {#each data.books as book}
        <div class="rounded-xl border bg-gray-800 text-white p-4 shadow">
            <img src="https://picsum.photos/seed/{book.id}/200/300"
                class="rounded-lg mb-3 w-full object-cover" alt=""/>
            <h3 class="text-lg font-semibold">{book.title}</h3>
            <p class="text-xs text-gray-500 border-b border-gray-700 py-2">{book.author} ({book.year})</p>
            <p class="text-sm text-gray-300 line-clamp-3">
                {book.description}
            </p>

            <div class="flex justify-center gap-3 mt-5">
                <button type="button" class="border border-amber-200 px-3 rounded-2xl bg-amber-500 cursor-pointer">Edit</button>
                <button type="button" class="border border-red-200 px-3 rounded-2xl bg-red-500 cursor-pointer">Hapus</button>
            </div>
        </div>
    {/each}
</div>
```

```js
// src/routes/dashboard/+page.server.js
import connection from '$lib/server/connection'
import { redirect } from '@sveltejs/kit';

export const load = async ({ fetch }) => {
    const rows = await connection.query(`SELECT * FROM books`);

    const response = await fetch('/api/auth/session');
    if(!response.ok) {
        redirect(303, '/login');
    }

    const user = await response.json();

    return {
        books: rows.rowCount > 0 ? rows.rows : [],
        user
    }
}
```

Tampilannya akan kaya gini.

<img class="img-fluid" alt="ssr-dashboard" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/ssr-dashboard.png" />

### Default Method

Form action harusnya punya action properti para tag form. Di Sveltekit kalo Lo ga menambahkan action properti, maka Sveltekit akan otomatis menggunakan method `default` pada Form Actions parameter. Jadi coba Lo buat halaman baru namanya add dan buat file `+page.svelte` dan `+page.server.js`.

```html
<form method="post">
    <div class="flex flex-col mb-3">
        <label for="title">Title</label>
            <input
                id="title"
                type="text"
                name="title"
                placeholder="Book Title"
                class="w-full px-4 py-2 border rounded-lg
                    border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none
                    dark:bg-zinc-800 dark:border-zinc-700 dark:text-white"
        />
    </div>
    <div class="flex flex-col mb-3">
        <label for="author">Author</label>
            <input
                id="author"
                type="text"
                name="author"
                placeholder="Author Name"
                class="w-full px-4 py-2 border rounded-lg
                    border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none
                    dark:bg-zinc-800 dark:border-zinc-700 dark:text-white"
        />
    </div>
    <div class="flex flex-col mb-3">
        <label for="description">Description</label>
            <input
            id="description"
            type="text"
            name="description"
            placeholder="Book Description"
            class="w-full px-4 py-2 border rounded-lg
                border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none
                dark:bg-zinc-800 dark:border-zinc-700 dark:text-white"
        />
    </div>
    <button type="submit" class="w-full border border-teal-300 bg-teal-700 rounded">Submit</button>
</form>
```

```js
// src/routes/dashboard/add/+page.server.js
import connection from '$lib/server/connection.js';
import { redirect } from '@sveltejs/kit';

export const actions = {
    default: async ({ request }) => { // default method
        const data = await request.formData(); // cara ambil form data
        
        const body = {
            title: data.get('title'), // ambil dari name
            author: data.get('author'),
            description: data.get('description')
        }

        const response = await connection.query(`
            INSERT INTO books (title, author, description, year) 
            VALUES ('${body.title}', '${body.author}', '${body.description}', ${new Date().getFullYear()})    
        `);

        if (response.rowCount > 0) {
            redirect(303, '/dashboard');
        } else {
            throw new Error('Failed to add book');
        }
    }
};
```

<img class="img-fluid" alt="ssr-add" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/ssr-add.png" />

Untuk SSR (Server Side Render) di Sveltekit + Server Action Lo udah ga perlu lagi pake binding karena `actions` akan mengambilnya sebagai name dari input. Sama kaya SSR form web application.

<img class="img-fluid" alt="ssr-list" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/ssr-list.png" />

### Name Action

Kadang-kadang, dalam satu URL path, kita ingin bisa menerima lebih dari satu jenis Form Action. Sveltekit mendukung ini dengan cara membuat method selain default() method. Jadi nanti Lo bisa menambahkan action ?/namaMethod di tag form. Nah kalo misalnya di satu URL path itu terdapat lebih dari satu. Lo malah udah ga bisa lagi pake default. Harus semuanya di rename dalam actions.

Misalnya gue mau aktifik tombol delete tapi gue bakal rename nama methodnya jadi `remove`. Tapi pertama Lo harus ubah dulu button hapus menjadi sebuah form. Karen namanya aja form action, jadi bisa jalan di form doang.

```html
<!-- <button type="button" class="border border-red-200 px-3 rounded-2xl bg-red-500 cursor-pointer">Hapus</button> -->

<form method="post" action="?/remove">
    <input type="hidden" name="id" value={book.id} />

    <button
        type="submit"
        name="action"
        value="delete"
        class="border border-red-200 px-3 rounded-2xl bg-red-500 text-white"
    >
        Hapus
    </button>
</form>
```

Difile `routes/dashboard/+page.server.js` Lo bisa tamahin ini di paling bawah

```js
// src/routes/dashboard/+page.server.js

// ......
export const actions = {
    remove: async ({ request }) => {
        const data = await request.formData();
        const id = data.get('id');
        console.log(id);
        const rows = await connection.query(`DELETE FROM books WHERE id = ${id}`);

        console.log(rows);
    }
};
```

Sekarang coba Lo klik tomol deletenya harusnya akan terhapus sesuai id yang Lo klik.

<img class="img-fluid" alt="ssr-delete" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/ssr-delete.png" />

### Enhance Function

Lo ngerasa ga bro pas Lo klik delete atau input halaman akan di reload? Itulah ciri - ciri SSR bro karena setiap ada action maka halaman akan reload. Selain itu ketika Lo delete 1 data nanti di url akan ada `/dashboard?remove=true` itu juga sama karena ada action di form dan itu bukan bug. Di Sveltekit ada cara biar kedua hal itu ga terjadi yaotu dengan menggunakan `enhance` function dari `$app/forms`.

Caranya Lo import `import { enhance } from '$app/forms';` lalu Lo pake di form dengan `use:enhance` dan Lo tinggal pake `action` di formnya. Harusnya pas Lo jalanin lagi ga akan reload dan urlnya akan tetep `/dashboard` saja.

### Validation

Membuat website yang akan menerima request dari user apalagi berkaitan dengan database validasi itu mahal. NBayangin Lo melakukan insert ke table nah Lo ga melakukan validasi pada data yang di input oleh user. Jika ternyata user yang melakukan adalah seorang hacker bisa aja dia menginputkan suatu script ke dalam website Lo ini yang namanya XSS.

Jadi implementasi validation itu penting banget baik itu di client side atau di server side. Pada kasus Sveltekit SSR artinya hlaman akan dirender di server dan form akan do handle oleh valiable `actions` di `page.server.js` jadi gmna cara Lo ambil messagenya?.

Sveltekit memberikan sebuat variable parameter bernama `form` pada props di page yang menggunakan form action. Cara mengirmkannya dengan menggunakan function `fail` dari `@sveltejs/kit`. `fail` punya 2 parameter yaitu `status` dan `data`. Jadi misal `fail(400, { message: 'Error message' })`, nanti akan ke kirim statusnya dan data yang Lo masukin. Lebih lengkapny Lo bisa kunjungi dokumentasinya disini <a href="https://svelte.dev/docs/kit/@sveltejs-kit#fail" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/@sveltejs-kit#fail</a>.

```js
// src/routes/dashboard/add/+page.server.js
import connection from '$lib/server/connection.js';
import { fail, redirect } from '@sveltejs/kit';

export const actions = {
    default: async ({ request }) => {
        const data = await request.formData();
        
        const body = {
            title: data.get('title'),
            author: data.get('author'),
            description: data.get('description')
        }

        if(body.title.length === 0) {
            return fail(400, { message: 'Title is required' });
        }

        if(body.author.length === 0) {
            return fail(400, { message: 'Author is required' });
        }

        if(body.description.length === 0) {
            return fail(400, { message: 'Description is required' });
        }

        const response = await connection.query(`
            INSERT INTO books (title, author, description, year) 
            VALUES ('${body.title}', '${body.author}', '${body.description}', ${new Date().getFullYear()})    
        `);

        if (response.rowCount > 0) {
            redirect(303, '/dashboard');
        } else {
            throw new Error('Failed to add book');
        }
    }
};
```

Jadi di `page.svelte` Lo bisa ambil error dengan mengunakan `form` dari `$props`

```html
<!-- src/routes/dashboard/add/+page.svelte -->
 <script>
    const { form } = $props();
</script>

<p class="text-red-500">{form?.message}</p>

<!-- ..... <form... -->
```

<img class="img-fluid" alt="ssr-validation" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/ssr-validation.png" />

</details>

<details>

<summary><h2>Advance Routing 📚</h2></summary>

### Route Grouping

Karena fitur filesystem route di Sveltekit terlalu sakti Lo mesti bingung buat memetakan route berdasarkan groupnya. Misalnya halaman Home, Login, About, Books adalah public dan sedangkan halaman Dashboard adalah private. Tapi layoutnya sama aja. Harusnya layout nya beda. Nah gmna cara membuatnya?.

- Buat ngatasin itu Lo bisa pake cara yang namanya groupng route / options route
- Caranya dengan menambahkan () pada folder routenya.
- Folder/route yang memiliki () tidak akan dianggap page oleh Sveltekit dia hanya akan menganggap layoutnya aja.
- Karena tidak dianggap page. Maka Lo ga boleh bikin halaman yang sejajar dengan groupnya.
- Misalkan Lo bikin `routes/(guest)/+page.svelte` dan Lo juga punya `routes/+page.svelte` maka ini akan bentrok. Karena Sveltekit menganggapnya adalah satu url.

Untuk lebih lengkapnya Lo bisa kunjungi dokumentasi <a href="https://svelte.dev/docs/kit/page-options" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/page-options</a>.

Okeh di kasus kita gue mau bikin kaya gini.

- Halaman Home, About, Books adalah guest page. Atau halaman public.

Ini adalah layout utama untuk semua page.

```html
<!-- src/routes/+layout.svelte -->
<script>
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { page } from '$app/state';

	let { children } = $props();

</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>{page.url.pathname.replace('/', '') ? page.url.pathname.replace('/', '').toLocaleUpperCase() : 'HOME'}</title>
</svelte:head>

<div class="dark min-h-screen bg-gray-800 text-gray-100">
	{@render children()}
</div>
```

```html
<!-- src/routes/(guest)/+layout.svelte -->
 <script>
    import { page } from '$app/state';

    const { children } = $props();
    const isAactive = (path) => page.url.pathname === path
</script>

<nav class="p-4 flex gap-4 border-b border-gray-500 justify-center">
    <a class="text-slate-100 {isAactive('/') ? 'border-b border-blue-400' : ''}" href="/">Home</a>
    <a class="text-slate-100 {isAactive('/about') ? 'border-b border-blue-400' : ''}" href="/about">About</a>
    <a class="text-slate-100 {isAactive('/book') ? 'border-b border-blue-400' : ''}" href="/book">Books</a>
</nav>

<main class="p-4 max-w-2xl mx-auto border border-gray-500 rounded-2xl my-3">
    {@render children()}
</main>
```

<img class="img-fluid" alt="guest-layout" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/guest-layout.png" />

- Halaman Login public juga tapi punya layout berbeda

```html
<!-- src/routes/(auth)/+layout.svelte -->
 <script>
    const { children } = $props();
</script>

<div class="min-h-screen flex flex-col items-center justify-center">
    {@render children()}
</div>
```

<img class="img-fluid" alt="auth-layout" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/auth-layout.png" />

- Halaman Dashboard adalah private.

```html
<!-- src/routes/(private)/+layout.svelte -->
<script>
    const { children } = $props();
</script>

<div class="w-full flex flex-col items-center p-4">
    {@render children()}
</div>
```

<img class="img-fluid" alt="private-layout" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/private-layout.png" />

jadi folder `routes` akan menjadi kaya gini.

```bash
routes/
├── (auth)/
|   └── login/
|       └── +page.svelte
│   └── +layout.svelte
├── (guest)/
│   └── about/
│   └── book/
│   └── +layout.svelte
│   └── +page.svelte
├── (private)/
│   └── dashboard/
│       └── add/
│           └── +page.svelte
│           └── +page.server.js
│       └── +page.svelte
│       └── +page.server.js
│   └── +layout.svelte
├── api/
└── +error.svelte
├── +layout.svelte
├── layout.css
```

Dengan begini layout tiap halaman akan berbeda sesuai dengan groupnya.

### Hierarchy Route

Kadang Lo mesti pernah ngalamin gini. Misalnya Lo punya template layout pake layout A, nah Lo pingin bikin halaman tapi dia masuknya ke group B, jadi mau ga mau Lo harus pake layoutnya B. Sah-sah aja misalnya Lo masukin page B ke layout A. Tapi kan kan jadi aneh aja kok halaman yang harusnya masuk group ini jadi masuk ke folder lain?

Sveltekit punya fitur yang namanya hierarhy route. Hierarchy route akan membuat halaman Lo bisa pake layout halaman lain. Misalnya Lo pngin bikin halaman contact. Nah masuknya ke guest group kan, tapi tim design ngasihnya layout utama. 

- Sekarang coba Lo buat folder `contact` di guest group untuk filenya namain `+page@.svelte`.
- `@` inilah yang akan mengambil layout tertentu. Defaultnya mengambil layout utama paling luar. `routes/+layout.svelte`.
- Lo juga bisa ambil layout lain tapi dengan syarat masih satu group / siblings.
- Misalnya Lo mau pake layout (auth) itu ga bisa, karena diluar dari group.
- Karena Lo pake layout lain, maka page itu ga akan pake layout nya sendiri. Misal di cpntact Lo bikin `+layout.svelte` maka itu ga akan dipake.

<img class="img-fluid" alt="hierarchy" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/hierarchy.png">

Selain untuk page, Lo juga bisa pake di layout. Jadi layout itu akan mengikuti layout lain.

Sebelumnya contact punya `+layout.svelte` tapi masih kosong. Nah gue mau gue isi kaya gini.

```html
<!-- src/routes/(guest)/contact/+layout.svelte -->
 <script>
    const { children } = $props();
</script>

<main class="p-4 max-w-2xl mx-auto border bg-red-500 border-gray-500 rounded-2xl my-3">
    {@render children()}
</main>
```

Nah di dalem contact gue mau bikin halaman payment dan layout nya juga. Tapi layoutnya gue mau pake layout punya contact.

```html
<!-- src/routes/(guest)/contact/payment/+layout@contact.svelte -->
<script>
    const { children } = $props();
</script>

<div class="w-full flex flex-col items-center">
    {@render children()}
</div>
```

<img class="img-fluid" alt="hierarchy-layout" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/hierarchy-layout.png">

### Optional Parameter

Sebelumnya membuat parameter di suatu halaman adalah dengan menggunakan `[nama folder]`. Tapi halaman itu harus memiliki parameter di urlnya misalnya `/book/1`. Kalo Lo ga ngasih parameter maka akan kembali ke parent nya. Atau kalo halaman tanpa parent maka akan error. Tapi di Sveltekit Lo juga bisa bikin halaman dengan parameter tapi optional. Artinya Lo bisa akses halaman tanpa parameter.

- Cara membuatnya dengan membungkus folder dengan `[[nama folder]]`.
- Dengan begini parameter boleh kosong.
- Tapi harus hati-hati. Misalnya gue bikin `+page.svelte` di profile. Maka nanti akan error karena akan conflic.

<img class="img-fluid" alt="conflic" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/conflic.png">

Coba Lo buat kaya di gambar gue terus isi `+page.svelte` nya kaya gini.

```html
<!-- src/routes/profile/[[id]]/+page.svelte -->
<script>
    import { page } from "$app/state";
</script>

<h1>Profile {page.params.id}</h1>
```

<div class="row">
    <div class="col-md-6">
		<img class="img-fluid" alt="ops-params-1" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/ops-params-1.png" />
	</div>
    <div class="col-md-6">
		<img class="img-fluid" alt="ops-params-2" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/ops-params-2.png" />
	</div>
</div>

### Rest Parameter

Lo merti pernah nemuin website yang punya halaman /parameter/parameter/parameter/parameter panjang intinya parameternya. Misalnya kaya google drive atau aplikasi file management lain kaya onedrive web. Kebayang ga bro misalnya Lo buat pake framework yang filesystem routing. Artinya Lo bahal bikin halaman `routes/[file]/[file]/[file]/[file]` akan kaya gini.

Ini akan merepotkan bro. Untunganya Sveltekit punya cara buat nanganin itu. Caranya sama kaya bikin route parameter cuma nama foldernya di spread jadi `[...file]. COba Lo buat route baru do private aja.

```html
<!-- src/routes/(private)/files/[...file]/+page.svelte -->
<script>
    const { data } = $props();
</script>

<p>File: {data.file}</p>
```

```js
// src/routes/(private)/files/[...file]/+page.js
export const load = async ({params}) => {
    return {
        file: params.file
    };
}
```

<img class="img-fluid" alt="spread-route" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/spread-route.png" />

### Match Parameter

Sekarang coba Lo ke halaman book dan disitu Lo isi parameternya bebas itu karena default parameter emng gitu, Lo bisa masukin apa aja misal kaya gini.

<img class="img-fluid" alt="nomatch-param" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/nomatch-param.png">

Akan error karena null tidak ditemukan. Dan harusnya isinya adalah angka atau number aja ga boleh karakter. Sveltekit punya folder special lain agar Lo bisa kontrol parameter yang di kirimkan di url. Namanya adalah folder `params`. Kalo Lo inget di awal catatan ini ada architecture Sveltekit dimana ada folder `src/params/`. Disinilah Lo bisa lakuin Match Parameter.

- Di folder ini Lo bisa bikin file.js dengan nama sesuai parameter yang Lo punya. Misalnya book/id artinya namanya `id.js`.
- Di dalem sini Lo dikasih 1 function namanya `match` dan punya parameter dimana isinya adalah url parameter Lo
- Lo harus melakukan initialisasi ke folder `[id]` Lo = `id.js` kaya gini `[id=id]`

```js
// src/params/id.js
export function match(param) {
  return /^\d+$/.test(param);
}
```

Ganti folder `[id]` jadi `[id=id]`.

<img class="img-fluid" alt="match-param" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/match-param.png">

Dengan gini maka messagenya akan Not Found atau 404 ini yang bener karena params nya ga menyediakan data itu.
 
</details>

<details>

<summary><h2>Hooks 📚</h2></summary>

Hooks adalah fitur di Sveltekit yang akan dieksekusi ketika ada suatu kejadian. Hooks adalah middleware di Sveltekit dimana dia bisa di set jalan paling awal dan tanpa harus ke url tertentu. Sebelumnya seperti page atau layout fungsi di dalamnya hanya akan jalan ketika Lo mengakses nya aja.

Hooks punya 3 hooks

- Server Hooks `hooks.server.js` hook ini hanya bisa dipake di SSR
- Client Hooks `hooks.client.js` hook ini hanya bisa dipake di CSR
- Universal Hooks `hooks.js` hook yang mau jalan di dua sisi

### Server Hooks

Server Hooks akan dijalankan setiap SvelteKit menerima request, baik itu ketika aplikasi berjalan, ataupun ketika prerender. Jadi server hooks ini bisa Lo pake misalnya ketika awal buka website Lo mau bikin Otorisasi atau Otektikasi biar user Lo ga bisa masuk sembarangan.

Cara bikin server hooks Lo tinggal bikin file `hooks.server.js` dan bikin function `handle(request)`, dimana parameter request merupakan object berisi attribute event dan resolve. Terus Lo bisa lakuin apapun atau ubah response dengan mengembalikan object `Response`, atau jika ingin meneruskan request ke aplikasi, kita bisa menggunakan attribute resolve pada request. jadi ada 4 function yang di miliki hooks.

- `handle(request)` dipanggil setiap request. Isi parameter adalah `({event, resolve})`
- `handleError(request, error)` dipanggil ketika ada error. Isi parameter adalah `({event, resolve})`
- `handleFetch(request)` dipanggil setiap request tapi untuk menjalankan http call. Isi parameter adalah `({event, resolve, fetch})`
- `init()` dipanggil sekali saat SvelteKit dimulai.

Tapi Lo ga bisa bikin fetch atau http call, soalnya server hooks ga punya parameter fetch. Tapi Lo bisa pake function `handleFetch(request)`. Kalo buat error Lo bisa pake function `handleError(request, error)`. Ini function perilakukan langsung http call jadi ati-ati kalo mau pake. Biasanya server hooks ini dipake buat kaya gini.

- Buat Otorisasi
- Buat Otektikasi
- Logging request
- Rate limiting
- Security headers

Okeh sekarang coba Lo buat file `hooks.server.js` di `src`, ceritanya gue mau bikin logging untuk setiap request masuk.

```js
// src/hooks.server.js
export const handle = async ({ event, resolve }) => {
    console.log('request ke:', event.url.href);

    return resolve(event);
};

export const handleError = ({ error, event }) => {
    console.error('Error di:', event.url.pathname);
    console.error(error);

    return {
        message: 'Terjadi kesalahan'
    };
};

export const init = async () => {
    console.log("Migration running...");
}
```

<img class="img-fluid" alt="hooks-server" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/hooks-server.png">

Ketika ada error maka akan dijalankan

<img class="img-fluid" alt="hooks-server-error" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/hooks-server-error.png">

Ketika Sveltekit jalan pertama kali maka akan dijalankan

<img class="img-fluid" alt="hooks-server-init" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/hooks-server-init.png">

### Client Hooks

Jika server hooks hanya bisa jalan di server maka client hooks hanya bisa jalan di client aja. Jadi client hooks hanya bisa akses window dan cuma cocok buat UI concern seperti global toast, atau handler error secara UI biar tidak membosankan. 

```js
// src/hooks.client.js
export const init = async () => {
    console.log("Client running...");
};

export const handleError = async ({ error }) => {
    console.error("Ups: ", error);
};
```

<img class="img-fluid" alt="hooks-client" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/hooks-client.png">

### Universal Hooks

Hooks ini bisa dipake di server dan client. Jadi bisa dipake buat akses cookie, akses db, akses window, localstorage dll. Tapi Universal ini kurang direkomendasikan, karena meskipun bisa dipake di kedua sisi biasanya ketika mengakses server Universal hooks akan gagal akses window. Jadi bisa error. 

Universal hook ini recomended kalo Lo misalnya mau buat mengubah url misalnya Lo punya halaman / Lo ganti jadi /home, tapi foldernya tetep mengakses ke `+page.svelte` paling luar. Untuk melakukan itu Lo bisa pake function yang namanya `reroute` ini hanya ada di Universal hooks.

```js
// src/hooks.js
export const reroute = async ({url}) => {
    if(url.pathname === "/") {
        return "/home";
    }
};
```

</details>

<details>

<summary><h2>Deployment 📚</h2></summary>

Ini adalah materi terakhir tentang Sveltekit di catatan gue ini yaitu tentang deployment. Setelah Lo ngoding, gelut sama error, terus serasa udah ga abar pen pamer ke orang - orang. Nah deployment ini adalah ritual yang paling sakral kalo Lo mau lakuin hal - hal yang Lo mau itu.

Buat melakukan deployment Lo cukup ketikkan `npm run build`. Nanti tunggu sampe kelar dan selamat web Lo udah jadi. Tapi realitanya ga semudah itu wkwkwk. 

- Perlu diperhatikan, saat proses build ini, semua kode di page dan layout, server maupun client akan di load oleh SvelteKit untuk melakukan analisis.
- Jika ada kode yang seharusnya tidak di load pada waktu build, itu harus Lo tandain contoh misal Lo nambahin perintah load ke database di Load Function nah mending jangan di jalankan pas build.
- Pastiin ga ada error dan code udah okeh.

Atau Lo bisa kunjungi dokumentasi <a href="https://svelte.dev/docs/kit/$app-environment#building" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/$app-environment#building</a>.

### Pre-rendering

Pre-rendering adalah teknik melakukan render dulu sebelum ada request. Jadi Lo bakal memaksa Sveltekit bikinin file HTML dulu di server, jadi pas ada user yang request ke halaman itu Sveltekit ga akan bikin halaman tapi Sveltekit langsung ngasih file HTML.

Untuk membuat pre-rendering Lo bisa tambahin ini di `routes/*.js`:

```js
export const prerender = true;
```

Coba Lu tambahin prerender di `routes/(guest)/book/+layout.js`:

```js
// src/routes/(guest)/book/+layout.js
export const prerender = true;

export const load = async ({ fetch }) => {
    const res = await fetch('/api/book.json')
    const books = await res.json()
    return {
        books
    }
}
```

Jadi pas nanti Lo jalanin `npm run build` maka halaman akan di buat dulu di server, jadi pas ada user yang request ke halaman itu Sveltekit langsung ngasih file HTML.

</img class="img-fluid" alt="prerender" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/prerender.png">

Sekarang coba Lo jalanin `npm run preview` ini untuk menjalankan aplikasi Sveltekit Lo dalam mode production.

### Adapter

Karena aplikasi Lo udah di build tinggal di deploy. Nah tapi kalo di server Lo jangan pake `npm run preview`. Karena itu bakal di jalanin oleh Vite, sedangkan vite itu bukan runtime tapi build tool. jadi performance aplikasi Lo ga akan maksimal dan bisa jadi lambat.

Disinilah Lo perlu yang namanya adapter. Sveltekit menyediakan beberapa adapter tergantung Lo mau deploy dimana dan sebagai apa. Lo bisa kunjungi documentasi <a href="https://svelte.dev/docs/kit/adapters" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/adapters</a> buat liat ada macam adapter apa saja.

### Node Server

Karena aplikasi di catatan ini di buat pake nodejs dan typenya SSR, maka runtime yang paling tepat adalah pake NodeJS itu sendiri. Sveltekit punya adapter yang namanya [Adapter Node](https://svelte.dev/docs/kit/adapter-node). Pertama Lo perlu install dulu.

```bash
npm i -D @sveltejs/adapter-node
```

Terus Lo buka file `svelte.config.js`:

```js
// import adapter from '@sveltejs/adapter-auto'; // ganti ini
import adapter from '@sveltejs/adapter-node'; // jadi ini

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		// adapter-auto only supports some environments, see https://svelte.dev/docs/kit/adapter-auto for a list.
		// If your environment is not supported, or you settled on a specific environment, switch out the adapter.
		// See https://svelte.dev/docs/kit/adapters for more information about adapters.
		adapter: adapter()
	}
};

export default config;
```

<img class="img-fluid" alt="adapter-node" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/adapter-node.png">

Sekarang warning yang sebelumnya udah ga ada. Dan aplikasi Lo tetep di tempat yang sama yaitu `.svelte-kit/output` dengan `server/index.js` sebagai entry point nya. Tapi Lo belum bisa jalanin pake node. Karena Lo harus konfigurasi sendiri dan perlu seluruh source ini. Agar menjadi 1 folder bundle Lo perlu tamahin parameter `out` di adapter node:

```js
adapter({
    out: 'build'
})
```

<img class="img-fluid" alt="build" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/build.png">

jadi sekarang Lo bisa jalanin gini:

```bash
node build/index.js

Migration running...
Listening on http://0.0.0.0:3000
```

<img class="img-fluid" alt="image" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/sveltekit-framework/public/image.png">

### Single Page Application (SPA/CSR)

Selain Lo bikin jadi mode Server Side Render, Lo bisa juga bikin jadi Single Page Application. Tapi syaratnya ga boleh ada code server seperti file `page.server.js`, folder `lib/server`, file `hooks.server.js` dan configurasi server side lain. Lebih lengkapnya Lo bisa kunjungi dokumentasi <a href="https://svelte.dev/docs/kit/single-page-apps" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/single-page-apps</a>.

</details>

Lumayan capek juga nulisnya. Udah dulu ya gaes kalo ada pertanyaan atau saran Lo bisa contact gue atau buat form diskusi di [Coffee Room](https://feri-irawansyah.my.id/coffee-room). Terima kasih.

---

<div class="d-flex flex-row justify-content-center align-items-center">Regards <a href="https://feri-irawansyah.my.id"><img style="width: 1rem; height: 1rem;" src="https://feri-irawansyah.my.id/favicon.ico" alt="Feri Irawansyah"> Feri Irawansyah</a></div>

---