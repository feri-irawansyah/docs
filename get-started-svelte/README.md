Woi bro..., Gue tau yang ada di benak lo, pas lo mau bikin tampilan web yang bagus, dinamis, dan banyak populer mesti lo bakal milih **React**. Dan kalo lo pingin yang mudah dipahami oleh programmer yang baru nyemplung tapi pingin cepet - cepet bikin **Web Application Using Frontend Framework** lo mesti bakal nyletuk, gue mau pake **Vue** aja ah....

Perlu gue akui 2 benda itu bagus 👍, modern technology 🤖 dan bisa buat fullstack application bro. Okeh gue mau minggir dulu sebentar mau bahas Frontend Framework yang ASING, JARANG DIPAKE, MINIMALIS, CURANG, SERASA KAYA NGOPLOS HTML + JS. Iyaaa, kita bakal bahas Svelte.

<img class="img-fluid" alt="image" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/svelte-wiki-1.png" />

Dikutip dari <a href="https://en.wikipedia.org/wiki/Svelte" target="_blank">Wikipedia</a> Svelte ini dibuat oleh Bapak - Bapak yang namanya <a href="https://x.com/rich_harris" target="_blank">Rich Harris</a> dan Kroco - Krocony tentunya Svelte Team. Dan Svelte ini langsung di compile ke **JS DOM**, tanpa Runtime, Hasil Kompilasi **Mini Size** dan ga kaya **React** atau **Vue** yang pake Virtual DOM katanya. Serasa bikin murni javascript? Tapi Declarative? Dan tanpa cari-cari class atau id bahkan element?. Wow minimalis sekali tapi apakah sepowerfull itu? Okeh kita coba sekarang.

<details open>
<summary><h2>📌 Svelte Frontend Framework</h2></summary>

### Kenapa Butuh Framework

Di dunia frontend banyak sekali framework populer kaya React, Vue, Angular, Svelte tapi buat apa si framework? Bikin aplikasi web pake HTML, CSS, JS juga udah bisa dan bagus. Kalo Lo sendiri bukan tim bebas itu terserah Lo bro tapi kalo Lo kerja tim gimana? Mesti bakal ada beberapa kekurangan misal:
	
- Code Javascript hanya Lo yang tau. Tim Lo belum tentu tau
- Tiap orang beda - beda nulis code bahkan arsitektur.
- Tidak ada aturan dalam gaya penulisan code
- Kalopun beneran pake vanila js, harus ada 1 atau 2 orang yang bikin arsitektur nya dan anggota lain mau ga mau harus mengikuti aturan yang di buat.

Nah dengan adanya Framework tim Lo bakal terorganisir bro ada aturan tertentu dalam membuat code dan aturannya sudah dibuatkan oleh si pembuat Frameworknya dan udah menjadi standarisasi di dunia.

### Dokumentasi Svelte

Pertama Lo coba ketikan di mesin pencarian lo `Svelte` atau pergi aja ke alamat <a href="https://svelte.dev" target="_blank">https://svelte.dev</a> lalu ke pergi ke <a href="https://svelte.dev/docs/svelte/overview" target="_blank">docs Svelte</a>. Atau silahkan coba - coba main disitu, asal jangan tersesat dijalan Pidana, Kejahatan dan Korupsi. 

<img class="img-fluid" alt="image" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/docs-svelte.png" />

Nah setalah masuk ke webnya dan Lo pencet yang Svelte bukan Sveltekit ya, nanti gue bakal buatin terpisah untuk Sveltekit. Pas lu masuk langsung di sugihkan dengan code

```html
<script>
	function greet() {
		alert('Welcome to Svelte!');
	}
</script>

<button onclick={greet}>click me</button>

<style>
	button {
		font-size: 2em;
	}
</style>
```
Nah apa itu, baru masuk langsung dapet bahasa Alien👽. Tenang bro, itu cuma overview doang kurang lebih codenya seperti itu, script, style, dan html di oplos jadi satu kaya Vue? Iyes bro betul svelte ini arsitekturnya mirip Vue JS ga perlu class component, functional component, atau ya semacam itu..

<h3>Component</h3>

Hampir semua frontend framework modern mengunakan Component sebagai base nya. Component adalah kumpulan code yang bisa digunakan secara independe dan biasanya berisikan satu atau lebih Element HTML, kode Javascript dan CSS. Tidak ada aturan seberapa besar atau kecil ukuran component seperti saat Lo bikin function Lo bisa bikin panjang atau kecil dan di pisah - pisah.

Kalo Lo kurang baham dengan konsep Component, component itu anggaplah kaya Lego yang Lo bisa susun dari kepingan kepingan agar jadi sutu bentuk yang Lo mau. Bedanya dengan javascript biasa Lo harus bikin dan jahit sendiri kek Lo bikin patung pake tanah liat.

Seperti code yang ada di halaman dokumentasi Svelte code itu adalah contoh component di Svelte. Semua file dengan extention atau format `.svelte` itu adalah component.

```html
<!-- Component.svelte -->

<script>
	function greet() {
		alert('Welcome to Svelte!');
	}
</script>

<button onclick={greet}>click me</button>

<style>
	button {
		font-size: 2em;
	}
</style>
```

<h3>Ekosistem Svelte</h3>

Svelte mengembangkan ekosistemnya sendiri, berbeda dengan React dimana banyak pihak ketiga yang mengembangkan ekosistemnya. Sveltekit adalah ekosistem Svelte itu sendiri dimana didalam Sveltekit ada banyak fitur yang bisa Lo pake untuk membuat website. Di catatan ini belum menggunakan Sveltekit karena untuk pembelajaran aja bukan untuk membuat website.

Keuntungan Svelte yang memiliki ekosistem nya sendiri Lo gampang buat integrasi dengan dependensi yang Lo butuhin karena dibangun oleh tim yang sama. ga ada lagi drama-drama dependensi yang entrok karena versinya ga compatible atau deprecated.

Kekurangannya ekosistemnya kecil. Karena hanya internal tim svelte yang mengambangkan tidak seesar React yang kalo Lo cari apa aja mesti ada.

</details>

<details open>
<summary><h2>📌 Get Started Svelte</h2></summary>

Sebelum Lo mulai membuat Svete Project, Lo perlu beberapa hal yang harus dipenuhi dulu
### Pre Requisites
- Pemahaman tentang Fundamental Javascript
- <a href="https://nodejs.org/" target="_blank" rel="">NodeJS </a> (npm, yarn, deno, bun, pnpm)
- VS Code (bisa cod eeditor lain)
- Pemahaman fundamental Vite

Di catatan ini gue pake nodejs version 22.20.0

```bash
C:\Users\feryi>node --version
v22.20.0
```

### Vite & Svelte

Untuk membuat Svelte project kita akan menggunakan <a href="https://vite.dev/" target="_blank" rel="">Vite</a> frontend build tools. Sebenarnya ada banyak build tools seperti gulp, webpack, rollup dll. Tapi framework modern hampir mayoritas menggunakan Vite sebagai default nya. Ouh iya membuat project Svelte ini kurang direkomendasikan untuk website besar karena Lo akan melakukan setup sendiri dengan Vite js dan itu memerlukan efort dan waktu lebih lama. Tapi kalo Lo mau full control dan mau bikin arsitektur Lo sendir valid - valid aja.

### Membuat project Svelte 

Untuk membuat project Svelte Lo cukup buka terminal dan arahkan ke folder dimana Lo pingin nyimpen source aplikasi nya lalu ketikkan

```bash
npm create vite@latest get-started-svelte
```

Nanti ada beberapa pilihan kaya gini. Ouh iya Lo bisa pake arah panah di keyboard buat geser - geser, tekan enter kalo buat milih:

```bash
# Select a framework
*  Select a framework:
|    Vanilla
|    Vue
|    React
|    Preact
|    Lit
|  > Svelte # selected
|    Solid
|    Qwik
|    Angular
|    Marko
|    Others
—

# Next
*  Select a variant:
|    TypeScript
|  > JavaScript # javascript karena hanya svelte aja
|    SvelteKit ↗
—

# Next
o  Use rolldown-vite (Experimental)?:
|    Yes
|  > No # default karena masih experimental

# Next
*  Install with npm and start now?
|  > Yes /   No # Pilih yes kalo mau langsung install dependencies dan start project
```

Karena gue pilih langsung install dan start maka jadinya seperti ini:

```bash
F:\project>npm create vite@latest get-started-svelte
Need to install the following packages:
create-vite@8.2.0
Ok to proceed? (y)


> npx
> create-vite get-started-svelte

|
o  Select a framework:
|  Svelte
|
o  Select a variant:
|  JavaScript
|
o  Use rolldown-vite (Experimental)?:
|  No
|
o  Install with npm and start now?
|  Yes
|
o  Scaffolding project in F:\project\get-started-svelte...
|
o  Installing dependencies with npm...

added 38 packages, and audited 39 packages in 8s

5 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
|
o  Starting dev server...

> get-started-svelte@0.0.0 dev
> vite

11:32:31 AM [vite] (client) Forced re-optimization of dependencies

  VITE v7.2.4  ready in 23150 ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose # Bisa Lo akses dari hp atau komputer lain asal 1 koneksi internet
  ➜  press h + enter to show help

```

<img class="img-fluid" alt="svelte-vite" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/svelte-vite.png" />

Ketik ctrl + c untuk keluar dari terminal lalu buka projectnya di VS Code. Tetap di posisi terminal sebelumnya lalu ketik perintah `code .` nanti akan terbuka projectnya di VS Code.

<img class="img-fluid" alt="svelte-vscode" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/svelte-vscode.png" />

Code utama svelte untuk development ada di src untuk yang lainnya adalah file - file configurasi bawaan dari Vite. Nah defaultnya Vite ngasih kita contoh component dan state (nanti kita bahas) 

```html
<!-- src/App.svelte -->

 <script>
  import Counter from "./lib/Counter.svelte";

</script>
<Counter /> <!-- Component Counter -->
```

```html
<!-- src/lib/Counter.svelte -->

<script>
	let count = $state(0)
	const increment = () => {
		count += 1
	}
</script>

<button onclick={increment}>
  	count is {count}
</button>

```

```js
// src/main.js

import { mount } from 'svelte'
import './app.css'
import App from './App.svelte'

const app = mount(App, {
  target: document.getElementById('app'),
})

export default app
```

#### Element `<Counter />`

Ketika Lo mau panggil Component di svelte component itu akan jadi Element HTML dengan Format `PascalCase` mengikuti nama file yang Lo bikin `Counter.svelte`.

- Jika Lo bikin component dengan nama `counter.svelte` maka teteap akan jadi `<Counter />`
- Kalo Lo bikin dengan `snake_case` seperti `hello_svelte.svelte` maka akan jadi `<HelloSvelte />`
- Kalo Lo bikin dengan `kebab-case` seperti `hello-svelte.svelte` maka akan jadi `<HelloSvelte />` meskipun di Svelte 4 - 1 bisa `<hello-svelte />` dan sekarang di Svelte 5 masih bisa, tapi tidak direkomendasikan lagi.
- Kalo Lo bikin dengan `camelCase` seperti `helloSvelte.svelte` maka akan jadi `<HelloSvelte />`

Jadi pada intinya agar lebih konsistem dan rapi untuk membuat component di svelte disarankan menggunakan `PascalCase` untuk membuat component.

#### Bahas file `main.js`



</details>

<details open>
<summary><h2>📌 Component</h2></summary>



</details>

