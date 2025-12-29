Di jaman modern sekarang banyak framework untuk membuat aplikasi khusuanya website. Kalo Lo pasukan king PHP ada Laravel + Livewire, kalo Lo pasukan Java ada Springboot atau kalo Lo pasukan kepiting yang demen di omelin compiler ada Leptos dan masih banyak lagi. 

Di catatan gue kali ini gue mau bahas tentang salah satu framework didunia Java Script yaitu pasukan anak yang hype, egie dan frameworker yang tiap hari kek ada aja gitu. Framework yang akan gue bahas yaitu `Sveltekit`. Catatan ini adalah lanjutan dari [Catatan Ini Tentang Svelte Frontend Library Yang Minimalis](https://feri-irawansyah.my.id/catatan/frontend/catatan-ringan-ini-tentang-svelte-frontend-framework-yang-minimalis). Dimana sekarang Lo bakal baca tulisan gue yang suka typo ini yang membahas tentang Framework dari Svelte. Karena di catatan sebelumnya pernah gue bahas **Framework itu perlu banget kalo Lo kerja secara tim, biar ga ada yang asal nulisin kode - kode nuklir yang bisa bikin aplikasi Lo meledak dan bug jadi numpuk kaya utang Lo**. Lo Bisa kunjungi documentasinya Sveltekit disini

- Sveltekit <a href="https://svelte.dev/docs/kit" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit</a>.

Tapi sebelum Lo baca kebawah gue merekomendasikan buat Lo baca dulu catatan tentang Svelte biar ga kaya bocah TK di ajak main biliar. Dan juga mingkin Lo perlu baca catatan gue tentang [Pilih CSR (Client Side Render) Atau SSR (Server Side Render) Untuk Website?](https://feri-irawansyah.my.id/catatan/frontend/pilih-csr-client-side-render-atau-server-side-render-untuk-frontend). Karena pada catatan tentang Sveltekit ini bakal membahas tentang CSR dan SSR. Tapi kalo Lo udah paham dan gue rasa Lo juga udah jago skip aja atau dari pada Lo baca tulisan gue mending chattingan sama gebetan.

<details open>
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

### Project Type

Selain architecture Lo juga bisa buat banyak macem aplikasi di Svelte fullstack, Single Page Application, Offline App, Mobile App (Pake Tauri/Capacitor), Desktop App (Pake Tauri, Wails, Electron) dll Lo bisa baca lebih lengkapnya disini. 

- Project Type Sveltekit <a href="https://svelte.dev/docs/kit/project-types" target="_blank" rel="noopener noreferrer">https://svelte.dev/docs/kit/project-types</a>.

Di catatan ini gue mau fokus ke project type Single Page Application (SPA/CSR) dan project type Server Side Render (SSR). Tapi kalo Lo pingin buat project type lainnya gue Lo bisa baca salah satu catatan gue yang ini [Bikin Mobile & Desktop App Yang Bau Kepiting (Rust 🦀) Dengan Tauri + Svelte](https://feri-irawansyah.my.id/catatan/frontend/bikin-mobile-desktop-app-yang-bau-kepiting-rust-dengan-tauri-svelte).

### Web Standard

Karena Sveltekit ini bisa bikin fullstack web, jadi Sveltekit sudah lengkap untuk standar - standar website kaya fetch, request, response, cookies, headers, dll. Lo bisa baca lebih lengkapnya disini.

- Web Standard Sveltekit <a href="https://svelte.dev/docs/kit/web-standards" target="_blank" rel="noopener noreferrer">Web Standard Sveltekit</a>.

</details>