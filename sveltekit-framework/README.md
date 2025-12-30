Di jaman modern sekarang banyak framework untuk membuat aplikasi khusuanya website. Kalo Lo pasukan king PHP ada Laravel + Livewire, kalo Lo pasukan Java ada Springboot atau kalo Lo pasukan kepiting yang demen di omelin compiler ada Leptos dan masih banyak lagi. 

Di catatan gue kali ini gue mau bahas tentang salah satu framework didunia Java Script yaitu pasukan anak yang hype, egie dan frameworker yang tiap hari kek ada aja gitu. Framework yang akan gue bahas yaitu `Sveltekit`. Catatan ini adalah lanjutan dari [Catatan Ini Tentang Svelte Frontend Library Yang Minimalis](https://feri-irawansyah.my.id/catatan/frontend/catatan-ringan-ini-tentang-svelte-frontend-framework-yang-minimalis). Dimana sekarang Lo bakal baca tulisan gue yang suka typo ini yang membahas tentang Framework dari Svelte. Karena di catatan sebelumnya pernah gue bahas **Framework itu perlu banget kalo Lo kerja secara tim, biar ga ada yang asal nulisin kode - kode nuklir yang bisa bikin aplikasi Lo meledak dan bug jadi numpuk kaya utang Lo**. Lo Bisa kunjungi documentasinya Sveltekit disini

- Sveltekit <a href="https://svelte.dev/docs/kit" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit</a>.

Tapi sebelum Lo baca kebawah gue merekomendasikan buat Lo baca dulu catatan tentang Svelte biar ga kaya bocah TK di ajak main biliar. Dan juga mingkin Lo perlu baca catatan gue tentang [Pilih CSR (Client Side Render) Atau SSR (Server Side Render) Untuk Website?](https://feri-irawansyah.my.id/catatan/frontend/pilih-csr-client-side-render-atau-server-side-render-untuk-frontend). Karena pada catatan tentang Sveltekit ini bakal membahas tentang CSR dan SSR. Tapi kalo Lo udah paham dan gue rasa Lo juga udah jago skip aja atau dari pada Lo baca tulisan gue mending chattingan sama gebetan.

<details>
<summary><h2>Kenalan Dengan Sveltekit 📚</h2></summary>

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
│ │ │ └ [code server side Lo (bisa backend)]
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

Di List Book yang udah Lo bikin fetch api di lakukan pake `onMount` artinya fetch dilakukan setelah component dirender. Nah kenapa ga dilakuin di load function aja? Bisa banget dan itu lebih recomended. Tapi karenaload function ini dijalankan di mode SSR kemudian datanya baru di CSR, maka ini akan error.

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
    const id = params.id
    const books = await parent();
    const book = books.books.find(book => book.id == id);

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

</details>