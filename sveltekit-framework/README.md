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

<details open>

<summary><h2>Routing 📚</h2></summary>

Kalo Lo pilih yang Sveltekit minimal nanti Lo akan dikasih folder `src/routes` yang didalamnya ada file `+layout.svelte` dan file `+page.svelte`. Sedangkan `layout.css` ini buat naro directive Tailwind CSS.

```css
@import 'tailwindcss';
@plugin '@tailwindcss/forms';
@plugin '@tailwindcss/typography';
```	

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

</details>