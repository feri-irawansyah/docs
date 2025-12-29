<style>
  @media screen and (min-width: 768px) {
    img[alt="props-drilling"], img[alt="image"], img[alt="svelte-vite"], img[alt="hello-world"] {
      width: 50% !important;
    }
  }
</style>

Woi bro..., Gue tau yang ada di benak lo, pas lo mau bikin tampilan web yang bagus, dinamis, dan banyak populer mesti lo bakal milih **React**. Dan kalo lo pingin yang mudah dipahami oleh programmer yang baru nyemplung tapi pingin cepet - cepet bikin **Web Application Using Frontend Framework** lo mesti bakal nyletuk, gue mau pake **Vue** aja ah....

Perlu gue akui 2 benda itu bagus 👍, modern technology 🤖 dan bisa buat fullstack application bro. Okeh gue mau minggir dulu sebentar mau bahas Frontend Framework yang ASING, JARANG DIPAKE, MINIMALIS, CURANG, SERASA KAYA NGOPLOS HTML + JS. Iyaaa, kita bakal bahas Svelte.

<img class="img-fluid" alt="image" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/svelte-wiki-1.png" />

Dikutip dari <a href="https://en.wikipedia.org/wiki/Svelte" target="_blank">Wikipedia</a> Svelte ini dibuat oleh Bapak - Bapak yang namanya <a href="https://x.com/rich_harris" target="_blank">Rich Harris</a> dan Kroco - Kroconya tentunya Svelte Team. Dan Svelte ini langsung di compile ke **JS DOM**, tanpa Runtime, Hasil Kompilasi **Mini Size** dan ga kaya **React** atau **Vue** yang pake Virtual DOM katanya. Serasa bikin murni javascript? Tapi Declarative? Dan tanpa cari-cari class atau id bahkan elemen?. Wow minimalis sekali tapi apakah sepowerfull itu? Okeh kita coba sekarang.

<details>
<summary><h2>Svelte Frontend Framework 📚</h2></summary>

### Kenapa Butuh Framework

Di dunia frontend banyak sekali framework populer kaya React, Vue, Angular, Svelte tapi buat apa si framework? Bikin aplikasi web pake HTML, CSS, JS juga udah bisa dan bagus. Kalo Lo sendiri bukan tim bebas itu terserah Lo bro tapi kalo Lo kerja tim gimana? Mesti bakal ada beberapa kekurangan misal:
	
- Code Javascript hanya Lo yang tau. Tim Lo belum tentu tau
- Tiap orang beda - beda nulis code bahkan arsitektur.
- Tidak ada aturan dalam gaya penulisan code
- Kalopun beneran pake vanila js, harus ada 1 atau 2 orang yang bikin arsitektur nya dan anggota lain mau ga mau harus mengikuti aturan yang di buat.

Nah dengan adanya Framework tim Lo bakal terorganisir bro ada aturan tertentu dalam membuat code dan aturannya sudah dibuatkan oleh si pembuat Frameworknya dan udah menjadi standarisasi di dunia. Framework itu perlu banget kalo Lo kerja secara tim, biar ga ada yang asal nulisin kode - kode nuklir yang bisa bikin aplikasi Lo meledak dan bug jadi numpuk kaya utang Lo

Tapi sebenernya Svelte ini bukan framework melainkan library. Sama seperti React, Vue, Angular, dll. mereka adalah library. Untuk frameworknya biasanya punya masing - masing seperti NextJs atau Astro untuk React. Nuxt untuk Vue.

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

Hampir semua frontend framework modern mengunakan Component sebagai base nya. Component adalah kumpulan code yang bisa digunakan secara independe dan biasanya berisikan satu atau lebih elemen HTML, kode Javascript dan CSS. Tidak ada aturan seberapa besar atau kecil ukuran component seperti saat Lo bikin function Lo bisa bikin panjang atau kecil dan di pisah - pisah.

Kalo Lo kurang baham dengan konsep Component, component itu anggaplah kaya Lego yang Lo bisa susun dari kepingan kepingan agar jadi suatu bentuk yang Lo mau. Bedanya dengan javascript biasa Lo harus bikin dan jahit sendiri kek Lo bikin patung pake tanah liat.

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

<details>
<summary><h2>Get Started Svelte 📚</h2></summary>

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
  target: document.getelemenById('app'),
})

export default app
```

#### elemen `<Counter />`

Ketika Lo mau panggil Component di svelte component itu akan jadi elemen HTML dengan Format `PascalCase` mengikuti nama file yang Lo bikin `Counter.svelte`.

- Jika Lo bikin component dengan nama `counter.svelte` maka teteap akan jadi `<Counter />`
- Kalo Lo bikin dengan `snake_case` seperti `hello_svelte.svelte` maka akan jadi `<HelloSvelte />`
- Kalo Lo bikin dengan `kebab-case` seperti `hello-svelte.svelte` maka akan jadi `<HelloSvelte />` meskipun di Svelte 4 - 1 bisa `<hello-svelte />` dan sekarang di Svelte 5 masih bisa, tapi tidak direkomendasikan lagi.
- Kalo Lo bikin dengan `camelCase` seperti `helloSvelte.svelte` maka akan jadi `<HelloSvelte />`

Jadi pada intinya agar lebih konsistem dan rapi untuk membuat component di svelte disarankan menggunakan `PascalCase` untuk membuat component.

#### Bahas file `main.js`

Di baris pertama ada `import { mount } from 'svelte'` function `mount` ini digunakan untuk merender suatu Component yaitu `App` ke dalam elemen HTML dengan id `app`. Nah kalo Lo buka file `index.html` di root project Lo ada file HTML dengan elemen `<div id="app"></div>`.

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>get-started-svelte</title>
  </head>
  <body>
    <div id="app"></div> <!-- elemen di panggil di file main.js -->
    <script type="module" src="/src/main.js"></script> <!-- File main.js di panggil di file index.html -->
  </body>
</html>
```

Jadi App Lo di taro di elemen HTML div ini. Nah pada catatan ini kita bakal pake `mount` untuk merender App ke dalam elemen HTML.

### Studi Case Hello World

```html
<!-- src/lib/HelloWorld.svelte -->
 <script>
    alert("Hello World");
</script>

<h1>Hellow World</h1>

<style>
    h1 {
        color: salmon;
    }
</style>
```

```html
<!-- hello.html -->
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Hello World</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/hello.js"></script>
  </body>
</html>
 ```

```js
// src/hello.js
import { mount } from 'svelte'
import './app.css'
import HelloWorld from './lib/HelloWorld.svelte'

const app = mount(HelloWorld, {
  target: document.getelemenById('app'),
})

export default app
```

```js
// vite.config.js
import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  build: {
    rollupOptions: {
      input: {
        index: 'index.html',
        hello: 'hello.html'
      }
    }
  }
})
```

<img class="img-fluid" alt="hello-world" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/hello-world.png" />

</details>

<details>
<summary><h2>Template HTML 📚</h2></summary>

File Svelte atai `.svelte` sebenarnya sama seperti HTML biasa Lo bisa mmenuliskan tag html terserah Lo ga ada aturan harus di bungkus pake `<></>` atau ada component khusus seperti `<Fragment></Fragment>` engga bro, file Svelte sama kaya file HTML. Bedanya ada fitur - fitur tambahan buat mempermudah hidup Lo.

File `.svelte` juga ga mewajibkan Lo buat nutup tag HTML, Nah di Framework lain seperti React atau Leptos Lo wajib nutup tag HTML `Self Closing Tag` seperti `<input />`, `<img />`, `<hr/>`. Kecuali ketika Lo manggil suatu Component seperti `<Counter />` Lo wajib pake tutup tag HTML.

Jadi biar konsisten direkomendasikan buat tetep pake `Self Closing Tag` aja.

### Text Expression

Fitur pertama adalah `Text Expression` ini diguakan untuk mengakses langsung suatu data dari Javascript. Ouh iya sebenarnya fitur pertama di file .svelte ini Lo bisa langsung akses suatu data dari tag `<script></script>` dan tag `<style></style>` juga bisa langsung akses suatu data. Nah dengan begini `Text Expression` lebih mudah digunakan.

Fitur lainnya di Text Expression ini Lo bisa juga lakuin function, method, dan juga object dari Javascript. Seperti `toUpperCase`, `toLowerCase`, `concat`, `split`, `slice`, dan masih banyak lagi.

Untuk cara pakenya itu Lo bisa pake kurung kurawal `{disini valuenya}`. Coba Lo buka file `HelloWorld.svelte` di folder `lib` dan ubah code nya menjadi ini:

```html
<!-- src/lib/HelloWorld.svelte -->
<script>
	const name = "Feri";
</script>

<h1>Hellow {name.toUpperCase()}</h1>

<style>
	h1 {
		color: salmon;
	}
</style>
```

<img class="img-fluid" alt="text-expression" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/text-expression.png" />


### Dynamic Attribute

Karena file `.svelte` sama kaya HTML, artinya Lo juga bisa pake atribut HTML persis kaya HTML biasa. Bedanya kalo misalnya variable yang Lo pake di atribut svelte ini memiliki nama yang sama Lo bisa memanggilnya sama persis kaya Text Expression.

```html
<!-- src/lib/HelloWorld.svelte -->
<script>
    const name = "Feri";
    const src = "https://feri-irawansyah.my.id/favicon.ico";
</script>

<h1>Hellow {name.toUpperCase()}</h1>

<img src={src} alt="Logo Snakesystem"/>

<img {src} alt="Logo Snakesystem"/> <!-- Fitur Svelte -->

<style>
    h1 {
        color: salmon;
    }
</style>
```

<img class="img-fluid" alt="dynamic-attribute" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/dynamic-attribute.png" />

### Nested Component

Nah kaya yang gue bilang di awal Component itu kaya Kepingan - Kepingan Lego yang bisa Lo susun jadi suatu bentuk yang Lo mau. Jadi nested component inilah implementasi dari Lego itu bro. 

Sekarang Coba Lo bikin file baru dengan nama `Logo.svelte` di folder `lib` dan Lo pindahin tag `<img/>` tadi ke dalam file `Logo.svelte` dan Lo ubah code nya menjadi ini:

```html
<!-- src/lib/Logo.svelte -->
<script>
    const src = "https://feri-irawansyah.my.id/favicon.ico";
</script>

<img {src} alt="Logo Snakesystem"/>
```

```html
<!-- src/lib/HelloWorld.svelte -->
 <script>
    import Logo from "./Logo.svelte";

    const name = "Feri";
</script>

<h1>Hellow {name.toUpperCase()}</h1>

<Logo/>

<style>
    h1 {
        color: salmon;
    }
</style>
```

<img class="img-fluid" alt="nested-component" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/nested-component.png" />

Di Svelte untuk Text Expression atau menampilkan suatu data di HTML ini sudah aman dari XSS (Cross Site Scripting) karena Svelte akan melakukan escape sebelum menampilkan data tersebut ke HTML.

</details>

<details>
<summary><h2>Rune 📚</h2></summary>

`Rune` adalah suatu simbol atau keyword di Svelte yang akan mengontrol Svelte Compiler seperti di awal catatan bahwa Svelte akan melakukan kompilasi code nya ke Javascript DOM murni tanpa Runtime seperti Virtual DOM. Oleh karena itu Rune itu sangat penting untuk mengontrol Svelte Compiler saat menggunakan suatu data, state atau elemen.

`Rune` memiliki awalan/prefix `$` dan kalo Lo pake VS Code akan terlihat seperti sebbuah function seperti `$state("hello");` artinya Lo pake `Rune state` yang memiliki parameter `hello`. 

Di svelte tersedia banyak Rune dan punya fiturnya masing-masing seperti `$state` contohnya. Lo bisa langsung kunjungi ke website nya di <a href="https://svelte.dev/docs/svelte/$state" target="_blank">Rune State</a> untuk melihatnya.

### State Rune `$state()`

Rune `$state()` adalah Rune yang digunakan untuk membuat reactive state di Svelte. Apa bedanya reactive sate dengan state biasa? 

Pada state biasa yang kita buat dalam variable, ketika terjadi perubahan pada variabel tersebut, maka UI tidak akan bereaksi terhadap perubahannya, hal ini menjadikan UI tidak akan berubah mengikuti perubahan state.

Nah sedangkan reactive state itu beda perilakunya bro, ketika terjadi perubahan pada reactive state, maka UI akan bereaksi terhadap perubahan tersebut tanpa kita lakuin apapun. Udah tinggal bengong aja nanti UI akan berubah sendiri.

- `$state("nilai awal")` parameter bisa di isi tipe data apa aja, bisa `number`, `string`, `boolean`, `null`, dan `undefined`.
- Ketika membuat variable untuk menampung reactive state haus mengunakan `mutable variable` seperti `let` atau `var` jangan gunakan `const`.

Aturannya ga terlalu ketat kok bro, tapi Lo harus bijak buat pakenya. Contoh kita balik ke halaman root atau index.html. disitu ada contoh pake `$state` di component `Counter.svelte`.

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

Biar ga terlalu bingung karena di `App.svelte` ada banyak elemen, kita buat halaman baru aja. Coba Lo buat `counter.html` dan `counter.js` seperti ini:

```html
<!-- counter.html -->
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Counter State</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/counter.js"></script>
  </body>
</html>
```

```js
// vite.config.js
import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  build: {
    rollupOptions: {
      input: {
        index: 'index.html',
        hello: 'hello.html',
        counter: 'counter.html'
      }
    }
  }
})
```

```js
// src/counter.js
import { mount } from 'svelte'
import './app.css'
import Counter from './lib/Counter.svelte'

const app = mount(Counter, {
  target: document.getelemenById('app'),
})

export default app
```

<img class="img-fluid" alt="counter-state" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/counter-state.png" />

Sekarang coba Lo bikin pake state biasa javascript seperti ini:

```html
<!-- src/lib/Counter.svelte -->
<script>
  let count = 0;

  const increment = () => {
    count += 1;
    document.getelemenById("count").innerHTML = "" + count;
  };
</script>

<button on:click={increment}>
  count is <span id="count">0</span>
</button>
```

Seperti ini juga sama aja dia Reactive juga tapi Lo harus pake DOM bro. Nah sebenarnya yang terjadi dari Svelte melakukan kompilasi itu akan mengubah code menjadi JS DOM yang kaya Lo buat dan + nya Svelte melakukan optimasi terhadap codenya.

### Deep State `($state.raw())`

Kalau Lo pake tipe data seperti `Array` atau `Object` di `$state`, maka data tersebut akan dibungkus dalam <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy" target="_blank">JavaScript Proxy</a>. Dengan begitu, perubahan yang terjadi pada data tersebut, Svelte bisa mendeteksinya, dan secara otomatis UI akan bereaksi terhadap perubahan data nya.

Karena `$state` itu Proxy maka kalo Lo mau kirim data ke API ga direkomendasikan langsung mentah-mentah kasih value dari `$state`. Kenapa?

- `$state` itu Proxy punya metadata, jadi misal Lo kirim serialize ke backend datanya bisa beda.
- Ukuran proxy lebih besar daripada data aslinya.
- Rawan infinite loop kalo Lo kirim proxy langsung ke backend.
- Bisa Error. Library Validasi kaya `Zod`, `Yup`, `Joi` dll biasanya suka bentrok atau ga cocok dengan Proxy.

Jadi misalnya Lo mau pake data buat di kirim ke backend, maka Lo bisa pake `$state.raw()` karena data udah clean. Tapi sayangnya tidak reactive karena tidak berkaitan dengan UI.

Analoginya gini
- `$state` = Armor Iron Man (Berat, Reactive bisa terbang, ada jarvis, bisa buat gelut) tapi ga bisa Lo pake buat jalan kaki.
- `$state.raw` = Tony Stark (Wujud asli, ga ribet, ga bisa terbang, tapi bisa buat jalan kaki).

Jadi `$state` tidak mendukung tipe data Collection seperti `Set` atau `Map`, namun Svelte menyediakan fitur `$state.raw()` untuk mengakses data tersebut. Contoh coba Lo buat halaman baru `salam.html`, `salam.js` dan `Salam.svelte` kaya sebelumnya. Nah terus pake code ini di `Salam.svelte`:

```html
<!-- src/lib/Salam.svelte -->
<script>
    let person = $state({
        firstName: 'Feri',
        lastName: 'Irawansyah'
    })

    function gantiNama() {
        person.firstName = 'Snake'
        person.lastName = 'System'
        console.log(person)
    }

</script>

<h1>Hello {person.firstName} {person.lastName}</h1>
<button onclick={gantiNama}>Ganti Nama</button>
```

<div class="row">
<div class="col-md-6">
<img class="img-fluid" alt="proxy-1" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/proxy-1.png" />
</div>
<div class="col-md-6">
<img class="img-fluid" alt="proxy-2" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/proxy-2.png" />
</div>
</div>

Ketika Lo click UI akan langsung berubah dan data di console menunjukan datanya adalah Proxy yang punya Header, Target dan metadata lainnya. Coba Lo bandingkan sama yang ini:

```html
<!-- src/lib/Salam.svelte -->
<script>
    let person = $state.raw({ // Ini pake `$state.raw()`
        firstName: 'Feri',
        lastName: 'Irawansyah'
    })

    function gantiNama() {
        person.firstName = 'Snake'
        person.lastName = 'System'
        console.log(person)
    }

</script>

<h1>Hello {person.firstName} {person.lastName}</h1>
<button onclick={gantiNama}>Ganti Nama</button>
```

<div class="row">
<div class="col-md-6">
<img class="img-fluid" alt="proxy-1" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/proxy-1.png" />
</div>
<div class="col-md-6">
<img class="img-fluid" alt="proxy-3" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/proxy-3.png" />
</div>
</div>

Saat Lo click datanya UI nya ga berubah, tapi datanya cuma Object biasa bukan proxy, Nah jadi ketika Lo serialize misal pake `JSON.stringify` maka datanya akan menjadi Object String biasa.

Intinya sesuaikan dengan kebutuhan.

### Derived Rune `$derived()`

Kadang nih ada kasus misal Lo mau bikin state baru tapi dia relative ke state lain. Misal Lo ada state `let count = $state(0)` nah terus Lo ada increment atau decrement gitu ya, terus Lo pingin bikin state baru yang reactive ke count dan nilainya misal pingin di kali 2 dari count (count * 2). Nah Lo bisa pake `$derived` buat ngelakuin itu.

Untuk lebih jelas bisa lihat [$derived Rune docs](https://svelte.dev/docs/svelte/$derived)

```html
<!-- src/lib/Counter.svelte -->
<script>
  let count = $state(0)
  const increment = () => {
    count += 1
  }

  let total = $derived(count * 2)
  // let total = count * 2 // ❌ Tidak reactive
</script>

Total: {total}

<button onclick={increment}>
  count is {count}
</button>
```

Okeh tapi misalnya Lo ada case lagi pingin pake expression control flow semacam bikin sebuah condisi atau function yang compleks di `$derived` sayangnya itu ga bisa bro. 

```html
<!-- src/lib/Counter.svelte -->
<script>
  let count = $state(0)
  const increment = () => {
    count += 1
  }

  let total = $derived(count * 2)
  let status = $derived(() => {
    if (count < 5) return 'low' 
    else if (count < 10) return 'medium'
    else return 'high'
  })
</script>

<div class="div">
  <p>total: {total}</p>
  <p>status: {status}</p>
</div>

<button onclick={increment}>
  count is {count}
</button>
```

### Derived By Rune `$derived.by(expression)`

Untuk mengatasi expression control flow di `$derived` Lo bisa pake `$derived.by(expression)`. Ini masih sama dengan `$derived` tapi bisa pake expression control flow bahkan function yang kompleks. 

```html
<!-- src/lib/Counter.svelte -->
<script>
  let count = $state(0)
  const increment = () => {
    count += 1
  }

  let total = $derived.by(count * 2)
  let status = $derived.by(() => {
    if (count < 5) return 'low' 
    else if (count < 10) return 'medium'
    else return 'high'
  })
</script>

<div class="div">
  <p>total: {total}</p>
  <p>status: {status}</p>
</div>

<button onclick={increment}>
  count is {count}
</button>
```

Nah kalo kaya gini bisa bro, coba buka halaman `http://localhost:5173/counter.html` dan coba click sampe nilai melebihin 5 dan 10.

### Debugging Rune `$inspect()`

Kadang saat membuat aplikasi, Lo biasanya sering melakukan log perubahan yang terjadi di State kan bro pake `console.log(state)` Tapi, karena reactive state itu sebenarnya adalah Proxy Object, maka akan terjadi warning ketika kita lakukan itu dan perubahan state juga ga terdeteksi oleh `console.log`.
Disarankan untuk menggunakan Rune $inspect(state) jika kita ingin pendeteksian perubahan yang terjadi pada state. 
Kalo Lo pingin lebih compleks atau custom Lo bisa pake `$inpect` dengan cara begini `$inspect(state).with((type, data) => …)`. Dokumentasi lengkapnya ada di [$inspect Rune docs](https://svelte.dev/docs/svelte/$inspect).

```html
<!-- src/lib/Counter.svelte -->
<script>
  let count = $state(0)
  const increment = () => {
    count += 1
  }

  let total = $derived.by(count * 2)
  let status = $derived.by(() => {
    if (count < 5) return 'low' 
    else if (count < 10) return 'medium'
    else return 'high'
  })

  // Pake $inspect
  $inspect(total).with((type, data) => {
    console.log(type, data)
  })
</script>

<div class="div">
  <p>total: {total}</p>
  <p>status: {status}</p>
</div>

<button onclick={increment}>
  count is {count}
</button>
 ```

<img class="img-fluid" alt="inspect-rune" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/inspect-rune.png" />

Tapi bro`$inspect` hanya jalan di mode development, ketika kode Svelte udah Lo build pake  Vite `$inspect` ga akan jalan dan kalo Lo pake minify malah akan dibuang codenya.
Jadi aman Lo menambahkan `$inspect` sebanyak apapun, karena hanya jalan di development, tidak akan jalan di production.

### Effect Rune `$effect()`

Lo pernah kepikiran ga bro? Misalnya suatu state berubah nah Lo pingin lakuin sesuatu gitu gimana caranya. *Ah gampang tinggal pake if di tag `script` nanti pas nilainya sesuai baru lo jalanin function*. Okeh kita coba.

```html
<!-- src/lib/Counter.svelte -->
<script>
  let count = $state(0)
  const increment = () => {
    count += 1
  }

  let status = $state('')

  if(count > 5) {
    status = 'high'
  } else {
    status = 'low'
  }

</script>

<div class="div">
  <p>status: {status}</p>
</div>

<button onclick={increment}>
  count is {count}
</button>
 ```

Kaya gini kan maksud nya? Wkwkwkwk sayangnya ga jalan bro nih gue udah sampe seratus blm belubah statusnya.

<img class="img-fluid" alt="no-effect" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/no-effect.png" />

 Kenapa bisa gitu? Karena perubahan statenya tidak di deteksi if atau apapun yang di tulis di tag `script` hanya akan di jalankan sekali ketika load aja, sedangkan Rune state adalah Proxy. Untuk melakukannya Lo harus pake `$effect`.

 ```html
<!-- src/lib/Counter.svelte -->
<script>
  let count = $state(0)
  const increment = () => {
    count += 1
  }

  let status = $state('')

  $effect(() => {
    if(count > 5) {
      status = 'high'
    } else {
      status = 'low'
    }
  })

</script>

<div class="div">
  <p>status: {status}</p>
</div>

<button onclick={increment}>
  count is {count}
</button>
 ```

<img class="img-fluid" alt="effect" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/effect.png" />

Nah ini baru jalan, Tapi kenapa ga pake Rune `$derived` aja? Kayanya sama aja kan. Kelihatannya sama tapi beda fungsi bro.

<div class="table-responsive">
  <table class="table">
    <thead>
      <tr>
        <th scope="col">#</th>
        <th scope="col">Context</th>
        <th scope="col">$effect</th>
        <th scope="col">$derived</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <th scope="row">1</th>
        <td>Tujuan</td>
        <td>Menjalankan efek samping (side-effects)</td>
        <td>Menghitung nilai baru berdasarkan state lain</td>
      </tr>
      <tr>
        <th scope="row">2</th>
        <td>Jenis</td>
        <td>Imperatif (jalanin aksi)</td>
        <td>Deklaratif (computed value)</td>
      </tr>
      <tr>
        <th scope="row">3</th>
        <td>Dipanggil ulang saat state berubah</td>
        <td>Iyes</td>
        <td>Iyes, tapi hanya hasil computation</td>
      </tr>
      <tr>
        <th scope="row">4</th>
        <td>Bisa mutasi state?</td>
        <td>✅ Boleh (misalnya fetch data, update store)</td>
        <td>❌ Tidak boleh – harus pure function</td>
      </tr>
      <tr>
        <th scope="row">5</th>
        <td>Return Value</td>
        <td>Abaikan</td>
        <td>Hasil data baru</td>
      </tr>
      <tr>
        <th scope="row">6</th>
        <td>Cocok untuk</td>
        <td>Fetch API, update DOM/manual event, logging</td>
        <td>Format data, kalkulasi, filter, mapping</td>
      </tr>
    </tbody>
  </table>
</div>

#### Jangan Asal Pake `$effect`

Karena effect ini akan memantau setiap perubahan state artinya apapun yang terjadi pada state dia akan menjalankan apapun yang Lo perintahkan. Gue ada beberapa contoh penggunaan `$effect` yang bisa jadi bom buat Lo.

##### Memantau sebuat state, tapi mengubah state itu juga

```html
<script>
	let count = $state(0);

	$effect(() => {
		count++; // Mutasi state yang jadi dependency-nya sendiri
	});
</script>
```

Yang terjadi bakal ngitung terus tanpa henti, kok bisa? Karena `$effect` akan memantau perubahan `count` dan dia sendiri juga yang melakukan perubahan jadi siklusnya ga akan berhenti.

##### Tidak melakukan cleanup

```html
<script>
	let count = $state(0);
	let interval = $state(1000);

	$effect(() => {
		setInterval(() => {
			count++;
		}, interval);
	});
</script>

<h1>{count}</h1>
<button onclick={() => interval += 1000}>Change Interval to {interval} ms</button>
```

Lo kira ini aman? Kaga bro ketika Lo klik tombolnya, yang terjadi count terlihat tidak konsisten pergerakannya. Ini terjadi karena Interval yang sebelumnya belum berhenti dan ketika Lo klik maka akan menambahkan interval baru. Ini bahaya bro bisa jadi memory leak. Terus gimana cara pake `$effect` ini?

- Jangan gunakan effect untuk memantau dan mengubah state yang sama.
- Selalu gunakan Cleanup untuk side-effect jangka panjang
- Batasi dependency sebanyak mungkin
- Effect merupakan cara yang biasanya dilakukan terakhir jika memang tidak ada cara lain, misal seperti memanipulasi DOM secara manual
- Jangan terlalu banyak menggunakan effect, terutama misal ketika ingin memanipulasi data secara synchronized, kita bisa manfaatkan contohnya $derived, dibanding menggunakan $effect

Untuk kasus sebelumnya Lo bisa pake cara ini

```html
<script>
	let count = $state(0);
	let interval = $state(1000);

	$effect(() => {
		const id = setInterval(() => {
			count++;
		}, interval);

		return () => clearInterval(id);
	});
</script>

<h1>{count}</h1>
<button onclick={() => interval += 1000}>Change Interval to {interval} ms</button>
```

Lo bisa lakuin cleanup dengan menambahkan callback di akhir `$effect` kemudian isinya adalah action yang Lo mau.

### Props Rune `$props()`

Lo pernah kepikiran ga bro misal ketika Lo panggil suatu component nah Lo pingin sharing data dari parent ke child. Misal Lo ada halaman user, nah dari pada Lo tampilin datanya langsung di component user, mending Lo buat component terpisah aja. Nanti data yang diparent tinggal pindah ke component lain.

Okeh ini bisa aja. Tapi bagaimana kalo misa parent component juga butuh data yang Lo pindah? Bikin 2 data gitu?. Nah dari pada Lo buat 2 data, mending Lo pake rune `$props()`.

Coba Lo buat halaman baru `user.html`, `user.js`, `User.svelte` dan `UserRow.svelte`. untuk component `User.svelte` dan `UserRow.svelte` kaya gini:

```html
<!-- src/lib/User.svelte -->
<script>
  import UserRow from "./UserRow.svelte";

</script>
<table>
    <thead>
        <tr>
            <th>Id</th>
            <th>Name</th>
            <th>Address</th>
        </tr>
    </thead>
    <tbody>
        <UserRow id="1" name="Snake System" address="Jakarta" />
    </tbody>
</table>
```

```html
<!-- src/lib/UserRow.svelte -->
<script>
    const { id, name, address } = $props();
</script>

<tr>
    <td>{id}</td>
    <td>{name}</td>
    <td>{address}</td>
</tr>
```

Seperti ini bisa bro. Maksudnya gimana?

- Jadi saat Lo panggil component Lo bisa ngasih atribut apa aja, misal `id`, `name`, `address` selagi tidak ada sepace seperti `add ress` ini ga boleh. Lo bisa pake `camelCase`, `PascalCase`, `snake_case` atau `kebab-case` juga.
- Atribut yang pasang itu yang nantinya di ambil oleh Rune `$props()`
- Ketika diambil formatnya akn berubah menjadi object dengan begitu Lo bisa pake distructuring kaya gini `{ id, name, address }`.

Lo juga bisa pake variable bro ga harus langsung menuliskan valuenya ke atribut.

```html
<!-- src/lib/User.svelte -->
<script>
  import UserRow from "./UserRow.svelte";

  const user = {
    id: "1",
    name: "Snake System",
    address: "Jakarta",
  }

</script>
<table>
    <thead>
        <tr>
            <th>Id</th>
            <th>Name</th>
            <th>Address</th>
        </tr>
    </thead>
    <tbody>
        <UserRow id={user.id} name={user.name} address={user.address} />
    </tbody>
</table>
```

### Spread Props

Tapi dengan cara membuat banyak atribut di component, akan membuat component semakin panjang. Di svelte, Lo bisa pake spread props. Kaya gini:

```html
<!-- src/lib/User.svelte -->
<script>
  import UserRow from "./UserRow.svelte";

  const user = {
    id: "1",
    name: "Snake System",
    address: "Jakarta",
  }

</script>
<table>
    <thead>
        <tr>
            <th>Id</th>
            <th>Name</th>
            <th>Address</th>
        </tr>
    </thead>
    <tbody>
        <UserRow {...user} />
    </tbody>
</table>
```

Dengan seperti ini code Lo jauh lebih simpel dan ga banyak boilerplate.

Sebenarnya masih ada 2 Rune yang lagi yang belum gue bahas yaitu `$host` dan `$bindable` tapi 2 Rune ini lumayan compleks nanti gue bahas di bagian **binding** dan **event**.

### Reactive Class

Sebelumnya semua reactive state biasanya dibuat dalam variable seperti `let count = $state(0)` atau `const count = $state(0)`. Tapi bisa juga pake reactive class dan tetep reactive.

```html
<!-- src/lib/Counter.svelte -->
<script>
  class Counter {
    count = $state(0);
    interval = $state(1000);
  }

  const counter = new Counter();

  $effect(() => {
    const id = setInterval(() => {
      counter.count++;
    }, counter.interval);

    return () => clearInterval(id);
  });
</script>

<h1>{counter.count}</h1>
<button onclick={() => counter.interval += 1000}>Change Interval to {counter.interval} ms</button>
```

</details>

<details>

<summary><h2>Syntax Templating 📚</h2></summary>

### Control Flow Templating `{#if}...{/if}`

Ketika ingin menampilkan suatu data di HTML, kadang Lo juga ga pingin menampilkan datanya mentah-mentah kan bro? Misalnya Lo mau bikin suatu control pada semua data umur misalnya kalo umurnya 10 kebawah itu anak-anak, 20 kebawah itu remaja, 30 keatas itu dewasa dan sebagainya.

Di Svelte ada fitur namanya `Control Flow Expression` seperti `if`, `else if`, `else`, `switch` dan `case` tapi langsung di HTML.

- `#if expression}...{/if}`
- `{#if expression}...{:else if expression}...{/if}`
- `{#if expression}...{:else}...{/if}`

Kita coba ambil contoh buat data user sebelumnya, tambahkan object key age.

```html
<!-- src/lib/User.svelte -->
<script>
  import UserRow from "./UserRow.svelte";

</script>
<table>
    <thead>
        <tr>
            <th>Id</th>
            <th>Name</th>
            <th>Address</th>
            <th>Age</th>
        </tr>
    </thead>
    <tbody>
        <UserRow id="1" name="Snake System" address="Jakarta" age={9} />
        <UserRow id="2" name="Feri Irawansyah" address="Semarang" age={25} />
        <UserRow id="3" name="Satria Baja Ringan" address="bandung" age={34} />
    </tbody>
</table>
```

Kemudian di `UserRow.svelte` tambahkan control flow expression.

```html
<!-- src/lib/UserRow.svelte -->
<script>
    const { id, name, address, age } = $props();
</script>

<tr>
  <td>{id}</td>
  <td>{name}</td>
  <td>{address}</td>
  <td>
    {#if age < 20}
      Anak-anak
    {:else if age < 30}
      Remaja
    {:else}
      Dewasa
    {/if}
  </td>
</tr>
```

<img class="img-fluid" alt="user" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/user.png" />

### Iteration (Looping) Expression `{#each ...}`

Sebelumnya Lo bikin nya kaya gini kan bro

```html
<UserRow id="1" name="Snake System" address="Jakarta" age={9} />
<UserRow id="2" name="Feri Irawansyah" address="Semarang" age={25} />
<UserRow id="3" name="Satria Baja Ringan" address="bandung" age={34} />
```

Sekarang coba Lo bayangin misalnya usernya bertambah jadi 100 orang, jadi Lo nanti copy paster component nya gitu kan? Wkwkwk. Ada cara yang lebih baik, yaitu menggunakan `Iteration Expression` seperti ini:

- `{#each expression as name}...{/each}`
- `{#each expression as name, index}...{/each}`
- `{#each expression as name (key)}...{/each}`

Kapan menggunakan Iteration? Tentunya ketika Lo bertemu dengan data `Array` karena cara mengakses data didalam sebuah `Array` itu kita perlu melakukan looping. Sekarang coba Lo refactor code di component `User` datanya di tampung  ke variable `users`:

```html
<!-- src/lib/User.svelte -->
 <script>
  import UserRow from "./UserRow.svelte";

  const users = [
    { id: 1, name: "Snake System", address: "Jakarta", age: 9 },
    { id: 2, name: "Feri Irawansyah", address: "Semarang", age: 25 },
    { id: 3, name: "Satria Baja Ringan", address: "Bandung", age: 34 },
  ]

</script>
<table>
    <thead>
        <tr>
            <th>Id</th>
            <th>Name</th>
            <th>Address</th>
            <th>Age</th>
        </tr>
    </thead>
    <tbody>
        {#each users as user}
            <UserRow {...user}/>
        {/each}
    </tbody>
</table>
```

Sekarang Lo juga bisa implement `Spread Props` lagi karena data yang Lo kirim ke child sudah berbentuk object sesuai yang di harapkan sama `$props()`.

<img class="img-fluid" alt="user" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/user.png" />

Tampilannya sama aja kok, bedanya sekarang data nya diambil dari `Array` bukan `Object` seperti sebelumnya dan Lo ga perlu nulisin satu-satu componentnya. Sekarang coba Lo hapus datanya.

```html
<!-- src/lib/UserRow.svelte -->
 <script>
    const { id, name, address, age } = $props();

    const emojies = {
        "1": "🐍",
        "2": "👨‍⚕️",
        "3": "🤖",
    }

    const emoji = emojies[id];

</script>

<tr>
  <td>{emoji}</td>
  <td>{id}</td>
  <td>{name}</td>
  <td>{address}</td>
  <td>
    {#if age < 20}
      Anak-anak
    {:else if age < 30}
      Remaja
    {:else}
      Dewasa
    {/if}
  </td>
</tr>
```

Gue ada nambah `emoji` di depan id, biar lebih cakepan dikit.

```html
<!-- src/lib/User.svelte -->
 <script>
  import UserRow from "./UserRow.svelte";

  const users = $state([
    { id: 1, name: "Snake System", address: "Jakarta", age: 9 },
    { id: 2, name: "Feri Irawansyah", address: "Semarang", age: 25 },
    { id: 3, name: "Satria Baja Ringan", address: "Bandung", age: 34 },
  ]);

  const remove = () => {
      users.shift();
  }

</script>

<button onclick={remove}>Remove</button>

<table>
    <thead>
        <tr>
            <th>Id</th>
            <th>Name</th>
            <th>Address</th>
            <th>Age</th>
        </tr>
    </thead>
    <tbody>
        {#each users as user}
            <UserRow {...user}/>
        {/each}
    </tbody>
</table>
```

<img class="img-fluid" alt="remove-each" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/remove-each.png" />

Lah kok aneh? wkwkwk, usernya kehapus tapi emojinya kok kaga ya malah ngegeser pindah ke id 2 padahal id emoji uler kan 1 ya?

Sebenarnya inilah kelebihan Svelte yaitu, Svelte tidak akan merender ulang component ketika ada perubahan data. Jadi component `UserRow.svelte` itu ya masih tetep ga terjadi apa-apa karena yang berubah datanya aja, jadi elemen yang nempel sama datanya aja yang berubah yaitu `td`. Sedangkan emoji adalah data baru bukan termasuk dalam data user.

Jadi saat memanipulasi data Array yang digunakan di Each Block, misal menghapus data, maka Svelte akan coba menghapus elemen terakhir. Termasuk ketika menambah data, Svelte akan menambah elemen di bagian akhir. Kalo Lo sebelumnya pernah pake React atau Vue ini ga akan kejadian, karena ketika ada perubahan data component akan di render ulang. Makanya Svelte jauh lebih cepat karena langsung spesifik ke element yang emang terjadi perubahan bukan component nya yang di render ulang.

Loh tapi bukannya keliatan kaya Bug malah ya? wkwkwk. Yap Lo ga salah bro, emang itu bug. Sebenarnya ada beberapa cara untuk mengatasinya.

#### Gunakan `key` di `Each Block`

```html
<!-- src/lib/User.svelte -->
<tbody>
  {#each users as user(user.id)}
      <UserRow {...user}/>
  {/each}
</tbody>
```

Dengan cara ini, `Each` akan memantau perubahannya menggunakan key `user.id` sebagai identity untuk emoji.

#### Pake Rune `$derived`

```html
<!-- src/lib/UserRow.svelte -->
<script>
  const { id, name, address, age } = $props();

  const emojies = {
      "1": "🐍",
      "2": "👨‍⚕️",
      "3": "🤖",
  }
  
  const emoji = $derived(emojies[id]);

</script>

<tr>
  <td>{emoji}</td>
  <td>{id}</td>
  <td>{name}</td>
  <td>{address}</td>
  <td>
    {#if age < 20}
      Anak-anak
    {:else if age < 30}
      Remaja
    {:else}
      Dewasa
    {/if}
  </td>
</tr>
```

Cara kedua pake Rune `$derived` bisa juga karena Rune ini akan memantau perubahan data dan kemudian membuat data emoji berdasarkan perubahan dari data user.

#### Langsung render emoji di `Each Block`

```html
<!-- src/lib/UserRow.svelte -->
 <script>
    const { id, name, address, age } = $props();

    const emojies = {
        "1": "🐍",
        "2": "👨‍⚕️",
        "3": "🤖",
    }

</script>

<tr>
  <td>{emojies[id]}</td>
  <td>{id}</td>
  <td>{name}</td>
  <td>{address}</td>
  <td>
    {#if age < 20}
      Anak-anak
    {:else if age < 30}
      Remaja
    {:else}
      Dewasa
    {/if}
  </td>
</tr>
```

Kenapa cara ini bisa? Karena data emoji langsung di tempelkan di elemen. Kalo sebelumnya data emoji dibentuk dari javascript tidak reactive karena tidak mengikuti perubahan dam DOM. Sedangkan untuk cara ini data emoji langsung mengikuti perubahan dari elemen nya.

Dokumentasi dari `#each` bisa dilihat di [https://svelte.dev/docs/svelte/each](https://svelte.dev/docs/svelte/each). dan dokumentasi dari `key #each` bisa dilihat di [https://svelte.dev/docs/svelte/each#Keyed-each-blocks](https://svelte.dev/docs/svelte/each#Keyed-each-blocks).

### Asyncronous Templating `{#await ...}`

Di Svelte Lo bisa langsung pake `Promise` didalam tag HTML, kaya gini cara pakenya:

- `{#await expression}...{:then name}...{:catch name}...{/await}`
- `{#await expression}...{:then name}...{/await}`
- `{#await expression then name}...{/await}`
- `{#await expression catch name}...{/await}`

Untuk lebih jelasnya bisa lihat di [https://svelte.dev/docs/svelte/await](https://svelte.dev/docs/svelte/await).

Wiiih keren dong gue ga perlu bikin promise di Javascript lagi misal mau bikin lazy loading. Yoi bro, Nah buat contoh coba Lo bikin halaman baru `article.html`, `article.js`, `Article.svelte`, dan `article.json` untuk nyimpen data. Ouh iya data article gue taro di `public/article.json`

```html
<!-- src/lib/Article.svelte -->
 <script>
    async function getArticle() {
        const response = await fetch("/article.json");
        return await response.json();
    }
</script>

{#await getArticle()}
    <p>Loading...</p>
{:then article}
    <h1>{article.title}</h1>
    {@html article.content}
{:catch error}
    <p style="color: red">{error.message}</p>
{/await}
```

<img class="img-fluid" alt="article" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/article.png" />

### HTML Templating

Saat menggunakan `Text Expression` di svelte itu udah aman dari `XSS (Cross Site Scripting)`. Jadi misal Lo mau nampilin text yang didalemnya ada tag HTML Svelte bakal lakuin escape text dulu baru di render. Kita coba di halaman `hello.html`.

```html
<!-- src/lib/HelloWorld.svelte -->
<script>
    import Logo from "./Logo.svelte";

    const name = "Feri";
    const text = "<h1>Hello Snakesystem</h1>";
</script>

<h1>Hellow {name.toUpperCase()}</h1>
{text}

<Logo/>

<style>
    h1 {
        color: salmon;
    }
</style>
```

<img class="img-fluid" alt="html-tags" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/html-tags-1.png" />

Nah tapi gimana misalnya Lo bener - bener utuh buat nampilin HTML dari suatu text atau string? Nah di svelte ada fitur namanya HTML Tags `@html`.

```html
<!-- src/lib/HelloWorld.svelte -->
<script>
    import Logo from "./Logo.svelte";

    const name = "Feri";
    const text = "<h1>Hello Snakesystem</h1>";
</script>

<h1>Hellow {name.toUpperCase()}</h1>
{@html text}

<Logo/>

<style>
    h1 {
        color: salmon;
    }
</style>
```

<img class="img-fluid" alt="html-tags" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/html-tags-2.png" />


### Render Templating `{@render ...}`

Lo pernah mikir ga bro atau pernah dapet kasus misalnya Lo pingin nampilin elemen tpi dinamis, pokoknya di satu tempat Lo pingin elemen nya bisa berubah-ubah. Gampang bro tinggal pake aja `{@html}` terus jadiin elemen nya html string. Nah misalnya component? Gimana tuh. Sayangnya component di jadiin string ga bakal jalan. Contoh coba Lo buat halaman baru `render.html`, `render.js`, `Render.svelte`:

```html
<!-- src/lib/Render.svelte -->
<script>
    import Logo from "./Logo.svelte";

    const logo = "<Logo />";
</script>


{@html logo}
```
<img class="img-fluid" alt="render" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/render.png" />


Ga bisa bro, kosong karena `<Logo/>` itu component, bukan murni elemen HTML. Nah cara yang benar pake `@render`. Jadi `@render` digunakan untuk melakukan rendering component yang dinamis bahkan beserta props yang menempel dengan componentnya.

```html
<!-- src/lib/Render.svelte -->
<script>
    import Logo from "./Logo.svelte";
</script>

{@render Logo()}
```

Kaya gini baru bisa, Ouh iya kalo Lo pake `@render` component yang Lo panggil caranya jadi kaya manggil function `NamaComponent()` bukan `{@render <NamaComponent />}`. 

Dengan `@render`, Lo juga bisa bikin props yang dinamis berupa component, props akan diterima sebagai `children`, sekarang Lo buat component baru dengan nama `RenderLayout.svelte` dan ubah `render.js` jadi ky gini:

```js
// src/render.js
import { mount } from 'svelte'
import './app.css'
import RenderLayout from './lib/RenderLayout.svelte'

const app = mount(RenderLayout, {
  target: document.getElementById('app'),
})

export default app
```

```html
<!-- src/lib/RenderLayout.svelte -->
<script>
  import Logo from "./Logo.svelte";
  import Render from "./Render.svelte";

</script>
<Render>
    <Logo />
</Render>
```

```html
<!-- src/lib/Render.svelte -->
 <script>
    const { children } = $props();
</script>

{@render children()}
```

Sekarng harusnya seperti ini sam ketika Lo pake `{@render Logo()}`. 

<img class="img-fluid" alt="render-2" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/render-2.png" />

Tapi bedanya sekarang lebih dinamis, coba Lo panggil component yang Lo buat sebelumnya kaya gini:

```html
<script>
  import Article from "./Article.svelte";
  import Counter from "./Counter.svelte";
  import Logo from "./Logo.svelte";
  import Render from "./Render.svelte";
  import User from "./User.svelte";

</script>
<Render>
    <Logo />
    <Counter/>
    <Article/>
    <User/>
</Render>
```

<img class="img-fluid" alt="render-3" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/render-3.png" />

Sekarng Lo jadi punya kaya halaman web yang di susun dari bnyak component yang dinamis.

### Snippet Templating `{#snippet ...}`

Fitur snippet ini sebenarnya fitur baru di Svelte tapi lumayan berguna kalo Lo pingin bikin semacam code yang berulang tapi pingin lebih simple di satu component.

- `{#snippet name()}...{/snippet}`
- `{#snippet name(param1, param2, paramN)}...{/snippet}`

```js
// src/lib/Snippet.svelte
 {#snippet Row(name)}
  <tr>
    <td>{name}</td>
  </tr>
{/snippet}

<table>
  {@render Row("Feri")}
  {@render Row("Snake System")}
  {@render Row("Satria")}
</table>
```

Misal kaya gini, jadi dari pada Lo menuliskan ulang suatu elemen jadi Lo pake snippet aja. Tapi ada beberapa hal yang perlu perhatiin.

1. `#snippet` hanya bisa dipake di satu component aja.
2. `#snippet` butuh templating `{@render ...}` untuk implementasinya.
3. Tidak punya state dan lifecycle. Artinya ketika ada perubahan data maka `snippet` akan dirender ulang.

### Debugger Template `{#debug ...}`

Kalo sebelumnya ada rune untuk debuging di level state. Di svelte Lo juga bisa deguging di level elemen. Misal kaya gini:

```js
// src/lib/Snippet.svelte
{#snippet Row(name)}
  <tr>
    <td>{name}</td>
    {@debug name}
  </tr>
{/snippet}

<table>
  {@render Row("Feri")}
  {@render Row("Snake System")}
  {@render Row("Satria")}
</table>
```

<img class="img-fluid" alt="debug" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/debug.png" />
<img class="img-fluid" alt="debug-2" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/debug-2.png" />

Ada tombol untuk next, jadi ketika jalan maka akan masuk ke debug mode, jadi browser akan melakukan loading. Lo bisa pencet tombol next yang gue garisin merah itu, nah nanti browser akan memberikan cursor ke code yang Lo jalanin. Ini cocok misal Lo mau ngecek code Lo baris demi baris.

Masih ada beberapa lagi templating syntax di Svelte seperti `{@attach ...}` (nanti gue bahas setelah 2 way binding), `{@const ...}`, dan `{#key ...}` ini jarang dipake, tapi Lo bisa baca baca di dokumentasinya Svelte.
</details>

<details>
<summary><h2>Event Handler 📚</h2></summary>

### Normal Event

Bicara soal Javascript mesinya ga jauh - jauh dari yang namanya `Event Handler`. Yes bro event handler adalah suatu action ketika Lo mau ngelakuin sesuai misal ketika Lo click suatu element, sedang mengetikkan kalimat, bahkan ketika Lo scrill halaman. Semua itu adalah event.

Sebelumnya Lo udah pernah pake salah satu Event Handler yaitu untuk hapus data. Di svelte, Lo bisa bikin event handler dengan cara `on<event>={function}`. Misal kaya gini:

```html
 <button onclick={() => console.log("pencet")}>Pencet</button>

<input oninput={(e) => console.log(e.target.value)} />
```

 Gue contohin di halaman `user.html` kita akan mencoba untuk menambahkan data.

```html
<!-- src/lib/User.svelte -->
 <script>
  import UserRow from "./UserRow.svelte";

  let users = $state([
    { id: 1, name: "Snake System", address: "Jakarta", age: 9 },
    { id: 2, name: "Feri Irawansyah", address: "Semarang", age: 25 },
    { id: 3, name: "Satria Baja Ringan", address: "Bandung", age: 34 },
  ]);

  let input = $state('');

  const remove = () => {
    users.shift();
  }

  const add = (e) => {
    e.preventDefault();

    users = [...users, {
      id: users.length + 1,
      name: input,
      address: "Jakarta",
      age: 9
    }]
  }

  const inputChange = (e) => {
    input = e.target.value;
  }

</script>

<button onclick={remove}>Remove</button>
<button onclick={add}>Tambah</button>

<input type="text" onchange={inputChange}>

<table>
    <thead>
        <tr>
            <th>Id</th>
            <th>Name</th>
            <th>Address</th>
            <th>Age</th>
        </tr>
    </thead>
    <tbody>
        {#each users as user}
            <UserRow {...user}/>
        {/each}
    </tbody>
</table>
```

<img class="img-fluid" alt="add-user" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/add-user.png" />

Event handler juga ga berlaku hanya di elemen itu aja, memang pada dasarnya Event akan menempel di suatu elemen HTML, tapi Lo juga bisa mengirimkan Event lewat props. Misal kaya gini:

```html
<!-- Child component -->
<script>
    const { onclick } = $props();
</script>

<button onclick={onclick}>Pencet</button>

<!-- Parent component -->
<Child onclick={() => console.log("pencet")} />
```

Nah coba sekarang Lo pindah tombol delete ke UserRow, dan juga gue mau hapus bagian emoji biar lebih rapi aja.

```html
<!-- src/lib/User.svelte -->
<script>
  import UserRow from "./UserRow.svelte";

  let users = $state([
    { id: 1, name: "Snake System", address: "Jakarta", age: 9 },
    { id: 2, name: "Feri Irawansyah", address: "Semarang", age: 25 },
    { id: 3, name: "Satria Baja Ringan", address: "Bandung", age: 34 },
  ]);

  let input = $state('');

  const remove = (id) => {
    users = users.filter(user => user.id !== id);
  }

  const add = (e) => {
      e.preventDefault();

      users = [...users, {
          id: users.length + 1,
          name: input,
          address: "Jakarta",
          age: 9
      }]
  }

  const inputChange = (e) => {
      input = e.target.value;
  }

</script>
<button onclick={add}>Tambah</button>

<input type="text" onchange={inputChange}>

<table>
    <thead>
        <tr>
            <th>Id</th>
            <th>Name</th>
            <th>Address</th>
            <th>Age</th>
        </tr>
    </thead>
    <tbody>
        {#each users as user}
            <UserRow {...user} onclick={() => remove(user.id)}/>
        {/each}
    </tbody>
</table>
```

```html
<!-- src/lib/UserRow.svelte -->
<script>
  const { id, name, address, age, onclick } = $props();
</script>

<tr>
  <td>{id}</td>
  <td>{name}</td>
  <td>{address}</td>
  <td>
    {#if age < 20}
      Anak-anak
    {:else if age < 30}
      Remaja
    {:else}
      Dewasa
    {/if}
  </td>
  <td><button type="button" {onclick}>Hapus</button></td>
</tr>
```

<img class="img-fluid" alt="remove-user" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/remove-user.png" />

### Dispatch Event

Di Svelte Lo bisa bikin event handler custom punya Lo sendiri dengan `createEventDispatcher`. Misal event onclick di remove user Lo ganti jadi `onhapus`. Nah tapi formatnya sekarang jadi `on:<event>`. jadi custom event Lo jadi `on:hapus` Misal kaya gini:

```html
<!-- src/lib/UserRow.svelte -->
<script>
  import { createEventDispatcher } from "svelte";

  const { id, name, address, age } = $props();

  const dispatch = createEventDispatcher();

</script>

<tr>
  <td>{id}</td>
  <td>{name}</td>
  <td>{address}</td>
  <td>
    {#if age < 20}
      Anak-anak
    {:else if age < 30}
      Remaja
    {:else}
      Dewasa
    {/if}
  </td>
  <td><button type="button" onclick={() => dispatch("hapus")}>Hapus</button></td>
</tr>
```

```html
<!-- src/lib/User.svelte -->
<tbody>
  {#each users as user}
      <UserRow {...user} on:hapus={() => remove(user.id)}/>
  {/each}
</tbody>
```

Keren ya bro, Lo bisa bikin custom handler sendiri. Tapi sayangnya `createEventDispatcher` ini udah deprecated di Svelte 5, makanya ketika jadi event handler `on:hapus` karena dulu di Svelte 4 semua event handler pake `on:`. Tapi masih compatible kok dipake di Svelte 5.

### Custom Element & Rune Host `$host()`

Di Svelte Lo bisa bikin custom elemen HTML misal `<my-component></my-component>` dengan Rune `$host()`. Kenapa Rune ini gue bahas terpisah ? Karena Rune ini berkaitan dengan Event handler dimana Lo bisa bikin Custom Event Handler kaya pake createEventDispatcher. Coba buat halaman baru bro `element.html`:

```html
<!-- src/lib/CustomElement.svelte -->
<svelte:options customElement={{ tag: 'custom-element', shadow: "none" }} />

<script>
	function dispatch(type) {
		$host().dispatchEvent(new CustomEvent(type));
	}
</script>

<button class="btn" onclick={() => dispatch('hello')}>Hello</button>
```

```html
<!-- src/lib/Element.svelte -->
 <script>
    import './CustomElement.svelte';

    const onHello = () => {
        alert('Hello')
    }

</script>

<custom-element onhello={onHello}></custom-element>
```

Disini ada `<svelte:options customElement={{ tag: 'custom-element', shadow: "none" }} />` ini adalah salah satu dari `Special Elements` di Svelte, nanti gue bahas.

Nah dengan custom element ini, Lo juga bisa bikin custom handler bro. Ini cara baru dari Svelte 5 untuk membuat Custom Event dengan cara membuat Custom Elemen juga dengan `svelte:option/>` dan Rune `$host()` untuk bikin custom event handler.

### Mount Action `use:action`

Selain action yang akan dijalankan ketika ada event handler, svelte juga menyediakan semacam auto action. Jadi ketika element dimount ke layar, svelte akan menjalankan function tersebut dan berlaku hanya untuk element tersebut.

```html
<script>
    let value = $state(3000);

    function formatCurrency(node) {
        // Simple currency formatting
        function formatValue(val) {
            let num = parseFloat(val.replace(/[^0-9.-]/g, ''));
            if (isNaN(num)) num = 0;
            return num.toLocaleString('en-US', { style: 'currency', currency: 'USD' });
        }
        node.value = formatValue(node.value);
    }

</script>

<!-- input type currency -->
<input type="text" bind:value={value} use:formatCurrency/>
```

<img class="img-fluid" alt="mount-action" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/mount-action.png" />

</details>

<details>
<summary><h2>Binding 📚</h2></summary>

### Two Way Binding

Lo tau ga bro, Svelte merupakan salah satu Library/Framework yang menggunakan perubahan data dengan 2 arah atau `Two Way Binding`. Ada banyak Library yang menggunakan konsep ini seperti `AngularJs` (Ini pendahulunya), `Angular`, `Vue`, `Knockout`, `Alpine`, `Ember` dan `Svelte`.

Two Way Binding contohnya misal Lu input suatu text dan text itu akan langsung ditangkap dan dikembalikan langsung ke Lo bro, Lo ga perlu get atau bikin event - event yang endingnya malah bikin Lo bingung kenapa datanya ga ada.

#### Input Value `bind:value={value}`

Bind value dipake buat ambil input dengan type string, number dan date. Misalnya Lo pingin ambil inputan nama, umur dan tanggal lahir contoh.

```html
<script>
   let name = $state('');
   let age = $state(0);
   let birth = $state(null);
   let sex = $state('');

   $inspect(name, age, birth, sex);
</script>

<input type="text" bind:value={name} placeholder="Full name"/>

<input type="number" bind:value={name} placeholder="Age" />

<input type="date" bind:value={birth} placeholder="Birth date" />

<select bind:value={sex}>
  <option value="male">Male</option>
  <option value="female">Female</option>
</select>
```

Cukup kaya gini aja Lo udah bisa ambil value dari input tanpa menambah event handler seperti `oninput` atau `onchange`. Tapi Lo juga tetep bisa pake event handler kalo emang di butuhkan. 

<img class="img-fluid" alt="bind-value" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/bind-value.png" />

Secara otomatis ketika menggunakan reactive state maka perilakunya akan sama seperti ketik Lo pke event handler `oninput` yaitu data akan otomtis terupdate ketika inputan tersebut diubah.

#### Input Checkbox `bind:checked={value}`

Bind checked dipake buat ambil inputan checkbox yang artinya dia hanya menerima `true` atau `false`. Misalnya Lo pingin ambil inputan `Agree to Terms and Conditions` contoh. 

```html
<script>
   let agree = $state(false);
</script>

<label for="agree">
  <input type="checkbox" bind:checked={agree} id="agree" />
  I agree
</label>
```

Karena input checkbox ini tidak punya placeholder dan bentuknya cuma input kecil dan hanya bisa diklik jadi Lo bisa bungkus pake label agar lebih menarik dan punya deskripsi.

#### Input Checkbox `bind:indeterminate={value}`

Bagian ini sebenernya masih berkaitan dengan input checkbox cuma jarang digunakan karena secara functional seperti perlu ga perlu. `bind:indeterminate` ini akan menghasilnya cektang separo pada input checkbox Lo jadi seolah olah kaya belum di apa - apain. Contohnya gini:

```html
<script>
	let checked = $state(false);
	let indeterminate = $state(true);
</script>

<form>
	<input type="checkbox" bind:checked bind:indeterminate>

	{#if indeterminate}
		waiting...
	{:else if checked}
		checked
	{:else}
		unchecked
	{/if}
</form>
```

<img class="img-fluid" alt="bind-indeterminate" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/bind-indeterminate.png" />

Bentuknya akan kaya gini, ketika Lo klik dan valuenya jadi `true` maka akan centang, kalo `false` maka tidak centang.

#### Input Group `bind:group={value}`

Input Group ini juga masih berkaitan dengan input checkbox dan binding ini menurut gue cukup sakti karena Lo bisa buat suatu list checkox tanpa melakukan iterator atau looping. Selain itu magicnya lagi Lo bisa ambil semua value dalam entuk `array` atau list.

```html
<script>
	let fillings = $state([]);

  $inspect(fillings)

</script>

<label><input type="checkbox" bind:group={fillings} value="Rice" /> Rice</label>
<label><input type="checkbox" bind:group={fillings} value="Beans" /> Beans</label>
<label><input type="checkbox" bind:group={fillings} value="Cheese" /> Cheese</label>
<label><input type="checkbox" bind:group={fillings} value="Guac (extra)" /> Guac (extra)</label>
```

<img class="img-fluid" alt="bind-group-check" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/bind-group-check.png" />

Dengan begini Lo ga perlu buat looping dan suatu event yang bisa push ke dalam array jika ada value baru ditambahkan. Selain itu `bind:group` juga bisa Lo pake untuk input type `radio` karena seperti yang Lo tau input radio hanya mengharapkan 1 nilai saja, sedangkan ketika nili itu udah dipilih maka nilai yang lain akan di hapus. Contohnya gini:

```html
<script>
	let tortilla = $state('Plain');

  $inspect(tortilla)

</script>

<label><input type="radio" bind:group={tortilla} value="Plain" /> Plain</label>
<label><input type="radio" bind:group={tortilla} value="Whole wheat" /> Whole wheat</label>
<label><input type="radio" bind:group={tortilla} value="Spinach" /> Spinach</label>
```

<img class="img-fluid" alt="bind-group-radio" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/bind-group-radio.png" />

Ketika Lo pilih salah satu maka semua input radio lain akan di uncheck. Dan ketika Lo pilih input radio yang sama maka input radio lain akan di uncheck.

#### Input File `bind:files={value}`

Binding ini digunakan untuk input file kaya gambar, excel, pdf atau file lainnya. Contohnya gini:

```html
<script>
	let files = $state();

    $inspect(files);
</script>

<label for="avatar">Upload a picture:</label>
<input accept="image/png, image/jpeg" bind:files id="avatar" name="avatar" type="file" />
```

<img class="img-fluid" alt="bind-file" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/bind-files.png" />

#### Reference Element `bind:this={value}`

Binding ini digunakan untuk mengambil referensi dari suatu element tersebut. Misalnya Lo bikin input, nah kemudian Lo mau dia auto focus ketika Lo kil suatu button. Lo bisa pake binding ini. Contohnya gini:

```html
<script>
  let inputEl = $state(null);

  function focusInput() {
    inputEl.focus();
  }
  
</script>

<input bind:this={inputEl} />
<button onclick={focusInput}>Focus</button>
```

<img class="img-fluid" alt="bind-this" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/bind-this.png" />

Ketika di klik input akan focus dan ketika Lo inspect ada element input dengan semua informasi nya. Sekilas ini mirip dengan DOM kalo Lo ambil element dengan `querySelector` atau `getElementById`. Jadi kalo kode tadi di convert ke avascript maka akan kaya gini:

```js
const inputEl = document.querySelector('input');

function focusInput() {
  inputEl.focus();
}
```

Bedanya, ketika Lo punya beberapa tag input di suatu halaman, semua input tersebut akan diambil semua oleh DOM, cara agar itu tidak terjadi biasanya Lo perlu memberikan id atau class untuk menandai element masing - masing. Nah dengan `bind:this` Lo bisa menyesuaikan statenya mau dipakein ke element mana aja yang Lo mau.

#### Rune `$bindable` sebagai props

Sebelumnya pada materi Rune ada satu rune yang belum gue bahas yaitu `$bindable`. Ini mau gue bahas. Kenapa dibahas terpisah? Karena `$bindable` ini merupakan rune khusus untuk Two Way Binding dan juga hanya bisa digunakan sebagai props. Artinya untuk pake Lo harus punya parent dan child component.

Okeh misalnya Lo ada study kasus buat bikin input auto complete dimana ketika Lo input akan muncul daftar List yang di pilih. Nah dari pada misalnya nantinya implement di tim codenya yang Lo bikin dan anggota tim lain beda. Nah Lo bisa bikin suau component input dengan props yang udah Lo sesuaikan agar perilakunya Two Way Binding. Contohnya gini:

```html
<!-- src/lib/AutoComplete.svelte -->
<script>
    let { value = $bindable(''), placeholder="", options = [] } = $props();
</script>

<input type="text" bind:value={value} placeholder={placeholder} list="autocomplete-options" />
<datalist id="autocomplete-options">
    {#each options as option}
        <option value={option} />
    {/each}
</datalist>
```

```html
<!-- src/lib/Bind.svelte -->
<script>
  import AutoComplete from "./AutoComplete.svelte";

  let fruit = $state('');

  $inspect("Selected Fruit: ", fruit);

</script>

<AutoComplete bind:value={fruit} options={[
    "Apple",
    "Banana",
    "Cherry",
    "Date",
    "Elderberry",
    "Fig",
    "Grape",
    "Honeydew"
  ]} placeholder="Fruits"/>

```

<img class="img-fluid" alt="bindable" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/bindable.png" />

Nah dengan begitu ketika ada anggota tim Lo yang ingin membuat input yang sama dengan data dan state berbeda tinggal pake component `AutoComplete` aja. Tanpa perlu membuat element baru.

</details>

<details>

<summary><h2>Style & Animation 📚</h2></summary>

Di svelte ada beberapa cara buat styling meskipun hasilnya sama aja tapi untuk penggunaannya aja yang berbeda tergantung kebutuhan Lo dan mempermudah hidup Lo juga.

### Styling

Membuat website Lo ga bakal lepas dari yang namanya styling atau CSS bro. Baik menggunakan teknologi apa aja website harus terlihat indah dan nyaman ketika dikunjungi. Svelte punya beberapa cara untuk memberikan styling pada suatu halaman.

#### Local Style

Local style ini default dari Svelte kenapa local style ? Karena style ini hanya akan berhalan di komponen dimana stylenya berada aja. Komponent lain tidak akan terpengaruh oleh style yang dibuat di component tersebut.

```html
<!-- src/lib/Hello.svelte -->
<script>
  console.log("Hello World");
</script>

<h1>Ini H1</h1>

<!-- Local Style -->
<style>
  h1 {
    color: red;
  }
</style>
```

Style ini hanya akan berjalan di component `Hello` saja, misal Lo buat component lagi maka component Lo ga akan terpengaruh oleh style ini.

Coba Lo buat component baru di folder `lib` dengan nama `Hello2.svelte` dan ubah code nya menjadi ini:

```html
<!-- src/lib/Hello2.svelte -->
<h1>Ini H1</h1>
```

```html
<!-- src/lib/Hello.svelte -->
<script>
  import Hello2 from "./Hello2.svelte";

  console.log("Hello World");
</script>

<h1>Ini H1</h1>
<Hello2/>

<!-- Local Style -->
<style>
  h1 {
    color: red;
  }
</style>
```

<img class="img-fluid" alt="local-style" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/local-style.png" />

Atau kalo misalnya pngin beda warna juga bisa gini:

```html
<!-- src/lib/Hello2.svelte -->
<h1>Ini H1</h1>

<style>
  h1 {
    color: green;
  }
</style>
```

`h1` di component `Hello2` akan berwarna hijau dan di parent tidak akan terpengaruh.

#### Global Style

Di Svelte kalo Lo buat style di suatu component maka sifatnya akan terisolasi dari component lain. Nah tapi gimana kalo misalnya Lo pingin buat style di seluruh halaman? Di svelte ada fitur namanya Global Style `:global(...)`. Contohnya gini:

```html
<!-- src/lib/Hello2.svelte -->
<h1>Ini H1</h1>
<!-- Global Style -->
<style>
  :global(h1) {
    color: green;
  }
</style>
```

Terus Lo hapus style di component `Hello`

```html
<!-- src/lib/Hello.svelte -->
<script>
  import Hello2 from "./Hello2.svelte";

  console.log("Hello World");
</script>

<h1>Ini H1</h1>
<Hello2/>
```

Ini akan membuat `h1` di component `Hello` dan `Hello2` akan berwarna hijau. Tapi harus hati-hati meskipun Lo bikin nya di child component semua component akan terpengaruh kecuali component tersebut punya style sendiri. 

Coba Lo balikin style di component `Hello` dan ubah code nya menjadi ini:

```html
<!-- src/lib/Hello.svelte -->
<style>
  h1 {
    color: red;
  }
</style>
```

Component `Hello` akan berwarna merah dan `Hello2` akan berwarna hijau. Karena component `Hello` punya style sendiri dan `specificity` nya lebih besar dari `Hello2` kecuali Lo paksain pake `!important` di style nya pada component `Hello2`. maka `Hello` akan berwarna hijau.

<img class="img-fluid" alt="global-style" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/global-style.png" />

Global style juga bisa dibuat dengan file terpisah khusus untuk css. Harusnya kalo Lo buat projectnya pake Vite maka akan ada file `app.css` di folder `src`. Itu juga termasuk global style, coba Lo cari bagian ini:

```css
/* src/app.css */
h1 {
  font-size: 3.2em;
  line-height: 1.1;
}
```

Ini membuat semua `h1` diseluruh halaman akan memiliki ukuran font 3.2em dan line height 1.1. Makanya semua `h1` Lo ukurannya sama, kecuali Lo cutsom di component via local style atau global style.

#### Inline Style

Inline style ini sama kaya html yaitu Lo nyisipin property style di tag HTML. Contohnya gini:

```html
<!-- src/lib/Hello2.svelte -->
<h1 style="color: green;">Ini H1</h1>
```

Atau kaya gini jua bisa jadi binding.

```html
<!-- src/lib/Hello2.svelte -->
<h1 style:color="green">Ini H1</h1>
```

Dan Lo tau bro? Di Svelte kalo Lo pake inline style Lo bisa buat stylenya dinamis misal ada suatu kondisi tertentu stylenya akan berubah rasanya mirip kaya Lo pake tailwindcss atau css modules.

Misalnya ada study case Lo pingin bikin `h1` ini warnanya bisa Lo ganti - ganti pake color picker. Coba Lo buat di component `Hello2` seperti ini:

```html
<!-- src/lib/Hello2.svelte -->
<script>
    let color = $state("green");
</script>

<input type="color" bind:value={color} />
<h1 style:color>Ini H1</h1>
```

<img class="img-fluid" alt="inline-style" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/inline-style.png" />

#### Class Binding

Class binding ini sama kaya style binding bedanya ini menggunakan class. Jadi Lo bisa meletakkan class yang sudah memiliki style css tertentu di tag HTML. Cara ini paling umum digunakan karena di era modern ini banyak CSS Framework yang populer dan biasanya menggunakan class sebagai selector nya seperti TailwindCSS, Bootstrap, Bulma, Materialize, dan masih banyak lagi. Contohnya gini:

```html
<!-- src/lib/Hello2.svelte with TailwindCSS -->
<h1 class="text-red-500">Ini H1</h1>

<!-- src/lib/Hello2.svelte with Bootstrap -->
<h1 class="text-danger">Ini H1</h1>

<!-- src/lib/Hello2.svelte with Materialize -->
<h1 class="red-text">Ini H1</h1>
```

Atau Lo bisa juga kaya gini:

```html
<!-- src/lib/Hello2.svelte with TailwindCSS -->
<h1 class:text-red-500>Ini H1</h1>

<!-- src/lib/Hello2.svelte with Bootstrap -->
<h1 class:text-danger>Ini H1</h1>

<!-- src/lib/Hello2.svelte with Materialize -->
<h1 class:red-text>Ini H1</h1>
```

#### CSS Pre Processors

Selain menggunakan CSS biasa, Lo juga bisa pake CSS Pre Processors seperti SASS, LESS, dan PostCSS. Cara menggunakannya adalah dengan cara menambahkan property `lang` di tag `<style></style>`. Contohnya gini:

```html
<!-- src/lib/Hello2.svelte with SCSS -->
<style lang="scss">
  h1 {
    color: red;
  }
</style>

<!-- src/lib/Hello2.svelte with LESS -->
<style lang="less">
  h1 {
    color: red;
  }
</style>

<!-- src/lib/Hello2.svelte with PostCSS -->
<style lang="postcss">
  h1 {
    color: red;
  }
</style>
```

### Transition

Biasanya agar website terlihat lebih hidup dan nyaman ketika dikunjungi, Lo mesti akan menambahkan transisi pada suatu halaman atau elemen HTML agar ketika elemen muncul atau hilang ada semacam efek. Menggunakan CSS saja biasanya cukup untuk membuat transisi. Tapi Svelte juga punya binding untuk membuat transisi dengan mudah serta Lo juga bisa melakukan custom transition.

#### Transition All `transition:name`

Transition ini akan berlaku pada semua elemen HTML barik ketika elemen tersebut muncul atau hilang. Contohnya gini:

```html
<!-- src/lib/Hello2.svelte -->
<script>
  import { fade, slide } from "svelte/transition";

  // Custom color picker state
  let color = $state("green");
</script>

<input type="color" bind:value={color} />
<h1 style:color transition:fade>Ini H1</h1>
<h2 style:color transition:slide>Ini H2</h2>
```

Selain itu Lo juga bisa memberikan parameter untuk menyesuaikan transisi seperti delay, duration, dan easing. Contohnya gini:

```html
<h2 style:color transition:slide={{ duration: 1000, axis: "x" }}>Ini H2</h2>
```

Untuk slide defaultnya adalah axis `y` (vertical) jadi Kalo Lo pingin horizontal maka gunakan `x`. Tapi untuk transisi lainnya seperti fade tidak selalu memiliki parameter yang sama seperti slide. Untuk jenis transisinya ada beberapa Lo bisa liat di docs nya Svelte disini [https://svelte.dev/docs/svelte/svelte-transition](https://svelte.dev/docs/svelte/svelte-transition). 

#### Transition In `in:name`

Kalo sebelumnya Lo pake binding `transition:name` dan akan berlaku ketika element muncul dan hilang. Kalo Lo ingin hanya ketika element muncul saja maka Lo bisa pake binding `in:name`. Contohnya gini:

```html
<!-- src/lib/Hello2.svelte -->
<script>
    import { fade } from "svelte/transition";

    let show = $state(false);
</script>

{#if show}
    <p in:fade>Hello from Hello2.svelte!</p>
{/if}

<button onclick={() => show = !show}>
    Click to show/hide message
</button>
```

Ketika Lo klik button maka message akan memiliki efek transisi. Tapi ketika hide message maka tidak akan ada efek transisi.

#### Transition Out `out:name`

Untuk memberikan efek transisi ketika element hilang maka Lo bisa pake binding `out:name`. Kalo biar 2x efek kenapa ga pake `transition:name` aja? Kan bisa ada efek transisi ketika element muncul dan hilang. Nah masalahnya kalo Lo pake `transition:name` efek transisinya hanya satu aja. Sedangkan kalo pake `in` dan `out` Lo bisa memberikan efek transisi yang berbeda ketika element muncul dan hilang. Contohnya gini:

```html
<!-- src/lib/Hello2.svelte -->
<script>
    import { blur, fade } from "svelte/transition";

    let show = $state(false);
</script>

{#if show}
    <p in:fade out:blur>Hello from Hello2.svelte!</p>
{/if}

<button onclick={() => show = !show}>
    Click to show/hide message
</button>
```

### Animation

Selain transisi Lo juga bisa pake animation. Tapi sayangnya di Svelte 5 ini baru ada satu animation yaitu `flip` yaitu seperti menikar posisi element. Contohnya gini:

```html
<!-- src/lib/Hello2.svelte -->
<script>
  import { flip } from "svelte/animate";

  let items = [
    { id: 1, text: "A" },
    { id: 2, text: "B" },
    { id: 3, text: "C" }
  ];

  function shuffle() {
    items = [...items].reverse();
  }
</script>

<button on:click={shuffle}>Reverse</button>

{#each items as item (item.id)}
  <p animate:flip={{ duration: 1000 }}>
    {item.text}
  </p>
{/each}
```

</details>

<details>

<summary><h2>Reactivity 📚</h2></summary>

Reactivity adalah kemampuan Svelte untuk memantau perubahan data dan mengubah UI secara otomatis ketika data tersebut berubah. Seperti contohnya memantau perubahan layar, perubahan data Date, Map, Set, dan lainnya. [https://svelte.dev/docs/svelte/svelte-reactivity](https://svelte.dev/docs/svelte/svelte-reactivity)

### MediaQuery

Untuk memantau perubahan layar bisa pake `window.matchMedia`. Contohnya gini:

```html
<script>
	import { MediaQuery } from 'svelte/reactivity';

	const large = new MediaQuery('min-width: 800px');
</script>

<h1>{large.current ? 'large screen' : 'small screen'}</h1>
```

Coba Lo kecilin browser nya terus lebarin sampe diatas 800px dan di bawah 800px nanti `h1` akan berubah content nya kalo misal sudah mencapai titik batas yang di tentuin. Dan ini reactive.

### SvelteDate

Untuk memantau perubahan data `Date` pada suatu device bisa pake `SvelteDate`. Contohnya gini:

```html
<script>
  import { SvelteDate } from 'svelte/reactivity';

  const now = new SvelteDate();

  $effect(() => {
    const i = setInterval(() => {
      now.setTime(Date.now());
    }, 1000);

    return () => clearInterval(i);
  });
</script>

<p>{now.toLocaleTimeString()}</p>
```

Ini waktunya akan terus bertambah karena reactive buat ui nya. Beda cerita kalo Lo pake `Date` biasa bawaan dari Javascript.

```html
<script>
  let now = new Date();

  $effect(() => {
    const i = setInterval(() => {
      now.setTime(Date.now()); // ❌ UI TIDAK UPDATE
    }, 1000);

    return () => clearInterval(i);
  });
</script>

<p>{now.toLocaleTimeString()}</p>
```

### SvelteMap

Untuk memantau perubahan data `Map` bisa pake `SvelteMap`. Yaitu untuk membuat data collection di Javascript. Bedanya kalo Lo pake `SvelteMap` ini akan reactive. Kalo Lo pake `Map` biasa bawaan dari Javascript ini tidak akan reactive. Contohnya gini:

```html
<script>
  let users = new Map();

  function addUser() {
    users.set(1, { name: 'Feri' });
  }
</script>

<button onclick={addUser}>Add</button>

<p>{users.size}</p>
```

Ini kalo Lo klik, tampilannya ga akan berubah, size akan tetap 0. Kalo Lo pake `SvelteMap` akan seperti ini:

```html
<script>
  import { SvelteMap } from 'svelte/reactivity';

  const users = new SvelteMap();

  function addUser() {
    users.set(1, { name: 'Feri' });
  }
</script>

<button onclick={addUser}>Add</button>

<p>Total users: {users.size}</p>
```

Ini akan berubah menjadi angka 1 kalo Lo klik.

</details>

<details>

<summary><h2>State Management 📚</h2></summary>

Sebelumnya ketika Lo pake state dan component lain ingin menggunakan dan memanipulasi datanya maka Lo perlu mengirimkan state tersebut sebagai props. Tapi props itu satu arah, artinya hanya dikirim dari parent ke child. Jadi kalo Lo mau manipulasi data di parent tapi action nya ada di child Lo perlu membuat suatu function untuk menangkap data tersebut dan yang kita lakukan sebelumnya ada gini:

```html
<!-- src/lib/UserRow.svelte -->
<script>
  import { createEventDispatcher } from "svelte";

  const { id, name, address, age } = $props();

  const dispatch = createEventDispatcher();

</script>

<tr>
  <td>{id}</td>
  <td>{name}</td>
  <td>{address}</td>
  <td>
    {#if age < 20}
      Anak-anak
    {:else if age < 30}
      Remaja
    {:else}
      Dewasa
    {/if}
  </td>
  <td><button type="button" onclick={() => dispatch("hapus")}>Hapus</button></td>
</tr>
```

```html
<!-- src/lib/User.svelte -->
<tbody>
  {#each users as user}
    <UserRow {...user} on:hapus={() => remove(user.id)}/>
  {/each}
</tbody>
```

Selain itu jika Lo misalnya punya 5 component yang bersarang, component parent punya suatu data dan component ke 2 dan ke 4 membutuhkan data dari parent. Maka yang terjadi data harus di kirimkan ke semua component. Padahal component child ke 1,3 dan 4 tidak membutuhkan datanya. Kondisi ini di sebut `props drilling`

<img class="img-fluid" alt="props-drilling" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/props-drilling.png" />

Untuk menangani beberapa kasus tersebut Svelte menyediakan state management bawaan. Ouh iya tidak seperti React yang menggunakan Redux atau Zustand dan Vue yang menggunakan Pania. Tim Svelte membuat state management langsung di dalam Svelte itu sendiri tanpa memerlukan dependency tambahan atau dari external.

### Context `getContext` & `setContext`

State management pertama di Svelte adalah `Context`. Context ini punya fitur `getContext` untuk getter dan `setContext` untuk setter. Tapi ada beberapa hal yang perlu perhatiin.

1. State pada context punya lifecycle satu arah.

Berjalan satu arah artinya state hanya bisa digunakan di scope component dimana Lo pake `setContext`. Misalnya di component `ParentA` Lo `setContext` maka state hanya bisa digunakan di Child nya misal `ChildA1` dan `ChildA2` saja. Kalo ada component yang setara dengan `ParentA` misal `ParentB` maka state nya ga bisa dipake di `ParentB` dan semua child dari `ParentB`.

2. Context state tidak bisa dipake di grand parent.

Meskipun berjalan satu arah tapi context state tidak bisa dipake di grand parent. Maksudnya misalnya `ParentA` punya parent lagi misalnya `GrandParentA` melakukan `getContext` maka context state juga ga bisa dipake.

3. Letakkan `setContext` di pada component paling atas.

Karena state hanya berlaku di parent dan child, maka untuk `setContext` harus dilakukan di component paling atas agar semua child nya bisa menggunakan `getContext` nya.

State management `Context` ini merupakan state manager paling aman. Kenapa? Karena data hanya akan di pake di scope tertentu aja. Jadi tidak dipake di seluruh website Lo. Tapi minusnya Lo harus `setContext` di component paling atas dan mungkin kalo ada banyak parent component teratas bisa memiliki banyak logic di dalamnya.

Kita coba buat studi kasus dari `User.svelte` ceritanya kita akan membuat contact book app. 

```html
<!-- src/App.svelte -->
 <script>
  import User from "./lib/User.svelte";

</script>

<h1>Contact Book</h1>
<User />
```

```html
<!-- src/lib/User.svelte -->
<script>
  import { setContext } from "svelte";
  import UserRow from "./UserRow.svelte";

  let users = $state([
    { id: 1, name: "Snake System", address: "Jakarta", age: 19, phone: "08123456789" },
    { id: 2, name: "Feri Irawansyah", address: "Semarang", age: 25, phone: "08987654321" },
    { id: 3, name: "Satria Baja Ringan", address: "Bandung", age: 34, phone: "08123456789" },
  ]);

  function deleteUser(id) {
    users = users.filter(u => u.id !== id);
  }

  setContext("users", {
    get list() {
      return users;
    },
    deleteUser
  });
</script>

<UserRow />
```

```html
<!-- src/lib/UserRow.svelte -->
<script>
  import { getContext } from "svelte";

  const users = getContext("users");

</script>

<table>
  <thead>
    <tr>
      <th>ID</th>
      <th>Name</th>
      <th>Address</th>
      <th>Age</th>
      <th>Phone</th>
      <th>Action</th>
    </tr>
  </thead>
  <tbody>
    {#each users.list as user}
      <tr>
        <td>{user.id}</td>
        <td>{user.name}</td>
        <td>{user.address}</td>
        <td>{user.age}</td>
        <td>{user.phone}</td>
        <td>
          <button on:click={() => users.deleteUser(user.id)}>
            Delete
          </button>
        </td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  table {
    width: 100%;
    border-collapse: collapse;
  }

  th, td {
    padding: 8px;
  }

  th {
    text-align: left;
  }

  button {
    padding: 5px 10px;
    background-color: red;
    border: none;
    cursor: pointer;
  }

  button:hover {
    background-color: darkred;
  }
</style>
```

<img class="img-fluid" alt="contact-book" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/contact-book.png" />

Sekarang kita coba tambahkan component baru untuk add user.

```html
<!-- src/lib/UserRow.svelte -->
 <script>
    const styleForm = `
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        margin-right: 10px;
    `;
</script>

<form action="" style="display: flex;">
    <div style={styleForm}>
        <label for="name">Name</label>
        <input type="text" placeholder="Name" id="name" />
    </div>
    <div style={styleForm}>
        <label for="address">Address</label>
        <input type="text" placeholder="Address" id="address" />
    </div>
    <div style={styleForm}>
        <label for="age">Age</label>
        <input type="number" placeholder="Age" id="age" />
    </div>
    <div style={styleForm}>
        <label for="phone">Phone</label>
        <input type="text" placeholder="Phone" id="phone" />
    </div>
    <button type="submit">Add User</button>
</form>
```

Lalu tambahkan di `App.svelte`:

```html
<!-- src/App.svelte -->
<h1>Contact Book</h1>
<AddUser /> <!-- tambah disini, jgn lupa import -->
<User />
```

Okeh sekarang gimana caranya kita bisa menambahkan user? Karena `setContext` di taro di `User.svelte`, jadi Lo ga bisa akses statenya coba console.log aja:

```html
<!-- src/lib/AddUser.svelte -->
 <script>
  import { getContext } from "svelte";

    const styleForm = `
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        margin-right: 10px;
    `;

    const users = getContext("users");

    console.log(users);
</script>
```

<img class="img-fluid" alt="add-contact" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/add-contact.png" />

### File `.svelte.js`

Agar Lo bisa akses state nya sekarang Lo ga bisa pake `Context` lagi bro, sekarang Lo perlu pake Global State dengan cara bikin file dengan extension `.svelte.js`. Coba Lo bikin folder baru di `src` dengan nama `stores` lalu buat file `users.svelte.js`:

```js
// src/stores/users.svelte.js
export const users = $state({
    list: [
        { id: 1, name: "Snake System", address: "Jakarta", age: 19, phone: "08123456789" },
        { id: 2, name: "Feri Irawansyah", address: "Semarang", age: 25, phone: "08987654321" },
        { id: 3, name: "Satria Baja Ringan", address: "Bandung", age: 34, phone: "08123456789" },
    ],
    remove: (id) => {
        users.list = users.list.filter(user => user.id !== id);
    }
});
```

Terus Lo ubah code di `User.svelte` dan `UserRow.svelte`:

```html
<!-- src/lib/User.svelte -->
 <script>
  import UserRow from "./UserRow.svelte";
</script>

<UserRow />
```

```html
<!-- src/lib/UserRow.svelte -->
<script>
  import { users } from "../stores/user.svelte";

</script>

<table>
  <thead>
    <tr>
      <th>ID</th>
      <th>Name</th>
      <th>Address</th>
      <th>Age</th>
      <th>Phone</th>
      <th>Action</th>
    </tr>
  </thead>
  <tbody>
    {#each users.list as user}
      <tr>
        <td>{user.id}</td>
        <td>{user.name}</td>
        <td>{user.address}</td>
        <td>{user.age}</td>
        <td>{user.phone}</td>
        <td>
          <button on:click={() => users.remove(user.id)}>
            Delete
          </button>
        </td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  table {
    width: 100%;
    border-collapse: collapse;
  }

  th, td {
    padding: 8px;
  }

  th {
    text-align: left;
  }

  button {
    padding: 5px 10px;
    background-color: red;
    border: none;
    cursor: pointer;
  }

  button:hover {
    background-color: darkred;
  }
</style>
```

Harusnya tampilannya masih sama, nah sekarang Lo bisa akses statenya di `AddUser.svelte`

```html
<!-- src/lib/AddUser.svelte -->
 <script>
    import { users } from "../stores/user.svelte";

    const styleForm = `
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        margin-right: 10px;
    `;

    console.log(users);
</script>
```

<img class="img-fluid" alt="svelte-js" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/svelte-js.png" />

Sekarang Lo seperti punya controller untuk mengatur state dari user di file `user.svelte.js`. Coba sekarang tambahkan method untuk menambahkan user kaya gini:

```js
// src/stores/user.svelte.js
export const users = $state({
    // ... list dan remove
    add: (user) => {
        const nextId =
            users.list.length === 0
            ? 1
            : Math.max(...users.list.map(u => u.id)) + 1;

        users.list = [
            ...users.list,
            { ...user, id: nextId }
        ];
    }
});
```

Sekarang Lo bisa pake method `add` di `AddUser.svelte`:

```html
<!-- src/lib/AddUser.svelte -->
<script>
    import { users } from "../stores/user.svelte";

    let formData = $state({
        name: "",
        address: "",
        age: "",
        phone: ""
    });

    function handleSubmit(event) {
        event.preventDefault();
        const newUser = {
            name: formData.name,
            address: formData.address,
            age: parseInt(formData.age),
            phone: formData.phone
        };
        users.add(newUser);
        formData = {
            name: "",
            address: "",
            age: "",
            phone: ""
        };
    }

    const styleForm = `
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        margin-right: 10px;
    `;
</script>

<form style="display: flex;" onsubmit={handleSubmit}>
    <div style={styleForm}>
        <label for="name">Name</label>
        <input type="text" placeholder="Name" id="name" bind:value={formData.name} />
    </div>
    <div style={styleForm}>
        <label for="address">Address</label>
        <input type="text" placeholder="Address" id="address" bind:value={formData.address} />
    </div>
    <div style={styleForm}>
        <label for="age">Age</label>
        <input type="number" placeholder="Age" id="age" bind:value={formData.age} />
    </div>
    <div style={styleForm}>
        <label for="phone">Phone</label>
        <input type="text" placeholder="Phone" id="phone" bind:value={formData.phone} />
    </div>
    <button type="submit">Add User</button>
</form>
```

<img class="img-fluid" alt="new-contact" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/new-contact.png" />

### Svelte Stores `writable`

Global state dengan menggunakan file `.svelte.js` + rune `$state()` ini memang powerfull dan simple tapi menggunakannya sebagai state management di App besar itu kurang direkomendasikan.

1. State benar-benar global

State ini benar - benar bisa diakses semua component di seluruh aplikasi. Kalo Lo sering bikin global state data akan terus aktif meskipun component atau DOM nya sudah hilang karena `file.svelte.js` adalah file module yang di-load sekali (singleton).

2. Kurang recommended untuk SSR (Server-Side Rendering)

Pada aplikasi dengan SSR, UI dirender di server untuk setiap request user. Masalahnya, module state (.svelte.js) akan hidup di server dan dibagi antar request ini bahaya bro state bisa leak, terjadi race condition dan bug juga susah di lacak.

3. Tidak Ada Lifecycle Otomatis untuk Reset State

Karena state berada di level module tidak ada mekanisme otomatis untuk reset, tidak tahu kapan halaman sudah tidak digunakan. Jadi Lo perlu membuat logic reset sendiri atau mengosongkan state secara explicit.

Alternatif dari `.svelte.js` + rune `$state()`, Svelte juga menyediakan Store (`writable`) sebagai solusi state management yang lebih terstruktur dan aman untuk aplikasi berskala besar. Store ini sudah ada sejak svelte 3 dan masih compatible dan recomended untuk aplikasi besar kenapa?

1. State Masih Global, tapi Lebih Terkontrol

Store pada dasarnya juga bersifat global, namun berbeda dengan module state, store memiliki mekanisme subscription. State hanya aktif ketika ada component yang subscribe, component bisa unsubscribe ketika di-unmount dan Lo punya kontrol lebih terhadap lifecycle statenya.

2. Lebih Aman untuk SSR

Store lebih direkomendasikan untuk SSR dibanding .svelte.js + $state() karena store bisa dibuat sebagai factory, state bisa diinisialisasi per request dan tidak harus berbagi state antar user

3. Lifecycle Lebih Jelas dan Mudah di Reset

Store menyediakan API eksplisit seperti `set`, `update`, `subscribe` dan `unsubscribe`. Ini yang bikin Lo punya control penuh dan jelas jadi ga bisa asal mengubah statenya. Kalo state dengan `.svelte.js` + rune `$state()` Lo bisa melakukan apa aja, data user bisa Lo tiban dengan data lain.

4. Cocok untuk Logic Kompleks dan Async

Store dirancang untuk menangani data async, streaming data, event eksternal dan side-effect ini membuat store lebih cocok digunakan pada aplikasi besar, banyak user, banyak interaksi dan state yang sering berubah.

Sekarang Lo coba ubah dari pake `.svelte.js` + rune `$state()` ke store, pertama Lo coba bikin file `user.js`:

```js
// src/stores/user.js
import { writable } from "svelte/store";

const createUserStore = () => {
    const { subscribe, set, update } = writable([
        { id: 1, name: "Snake System", address: "Jakarta", age: 19, phone: "08123456789" },
        { id: 2, name: "Feri Irawansyah", address: "Semarang", age: 25, phone: "08987654321" },
        { id: 3, name: "Satria Baja Ringan", address: "Bandung", age: 34, phone: "08123456789" },
    ]);

    return {
        subscribe,
        add: (user) => update((users) => [...users, user]),
        remove: (id) => update((users) => users.filter((user) => user.id !== id)),
    };
};

export const users = createUserStore();
```

Lalu di component `UserRow.svelte`:

```html
<!-- src/lib/UserRow.svelte -->
<script>
  import { users } from "../stores/user";

</script>

<table>
  <thead>
    <!-- ... -->
  </thead>
  <tbody>
    {#each $users as user} <!-- $users cara subscribe -->
      <tr>
        <td>{user.id}</td>
        <td>{user.name}</td>
        <td>{user.address}</td>
        <td>{user.age}</td>
        <td>{user.phone}</td>
        <td>
          <button on:click={() => users.remove(user.id)}>
            Delete
          </button>
        </td>
      </tr>
    {/each}
  </tbody>
</table>
```

```html
<!-- src/lib/AddUser.svelte -->
<script>
  import { users } from "../stores/user";
  // import { users } from "../stores/user.svelte"; ❌ hapus bagian ini
</script>
```

Biar lebih kerasa sekalian coba Lo tambahin buat edit data, scenarionya ada tombol action baru ketika di click formnya akan berubah jadi component `EditUser.svelte` dan udah di isi datanya.

```js
// src/lib/EditUser.svelte
const createUserStore = () => {
    const { subscribe, set, update } = writable([
        // ...
    ]);

    return {
        // subscribe, add, remove
        update: (id, updatedUser) => update((users) => users.map((user) => (user.id === id ? updatedUser : user))),
    };
};
```

Ada perubahan di `App.svelte` tambahin `setContext` untuk control flow `EditUser.svelte` dan `AddUser.svelte`:

```html
<!-- src/App.svelte -->
 <script>
    import { setContext } from "svelte";
    import AddUser from "./lib/AddUser.svelte";
    import User from "./lib/User.svelte";
    import EditUser from "./lib/EditUser.svelte";

    let onEdit = $state({
      edit: false,
      id: null
    });

    setContext("on-edit", onEdit);

</script>

<h1>Contact Book</h1>
{#if onEdit.edit}
    <EditUser id={onEdit.id} />
{:else}
  <AddUser />
{/if}
<User />
```

Ambil state context nya di `UserRow.svelte` buat dipake di tombol update:

```html
<!-- src/lib/UserRow.svelte -->
<script>
  // ....
  const onEdit = getContext("on-edit");

</script>

<table>
  <thead>
    <!-- ... -->
  </thead>
  <tbody>
    {#each $users as user (user.id)}
      <tr>
        <!-- ... -->
        <td>
          <button onclick={() => users.remove(user.id)}>
            Delete
          </button>
          <button style="background-color: green;" onclick={() => {
            onEdit.edit = true;
            onEdit.id = user.id;
          }}>
            Update
          </button>
        </td>
      </tr>
    {/each}
  </tbody>
</table>
```

Terus di `EditUser.svelte`:

```html
<!-- src/lib/EditUser.svelte -->
 <script>
    import { getContext } from "svelte";
    const { id } = $props();
    import { users } from "../stores/user";

    const onEdit = getContext("on-edit");
    
    // const styleForm = `...`;

    let formData = $state({
        name: $users.find(u => u.id === id)?.name || "",
        address: $users.find(u => u.id === id)?.address || "",
        age: $users.find(u => u.id === id)?.age || 0,
        phone: $users.find(u => u.id === id)?.phone || ""
    });

    function handleSubmit(event) {
        event.preventDefault();
        const updatedUser = {
            id: id,
            // ... sama seperti AddUser
        };
        users.update(id, updatedUser);
        onEdit.edit = false;
        onEdit.id = null;
    }
</script>

<form style="display: flex;" onsubmit={handleSubmit}>
    <!-- ... sama seperti AddUser -->
</form>
```

<div class="row">
    <div class="col-md-6">
        <img class="img-fluid" alt="edit-contact-1" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/edit-contact-1.png" />
    </div>
    <div class="col-md-6">
        <img class="img-fluid" alt="edit-contact-2" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/edit-contact-2.png" />
    </div>
</div>

Sebenarnya masih ada banyak lagi untuk store di Svelte tapi yang paling sering dipake adalah `writable` untuk membuat state management. Untuk lebih jelasnya bisa lihat di [https://svelte.dev/docs/svelte/svelte-store](https://svelte.dev/docs/svelte/svelte-store).

</details>

<details>
<summary><h2>Lifecycle Hooks (Alur Hidup Component) 📚</h2></summary>

Ini adalah topik pembahasan terakhir di catatan Svelte ini yaitu tentang alur hidup component. Ketika Lo panggil component misalnya `<User/>` artinya apapun code yang di dalamnya itu akan di init dan di render. Misalnya Lo panggil tag `<script></script>` maka code di dalamnya akan di init dan di render.

Misalnya Lo ambil data dari api mungkin dalam pikiran Lo bakal buat async function di dalam `<script></script>` kemudian langsung panggil juga functionnya kaya gini.

```html
<!-- Example -->
<script>
  async function getData() {
    const response = await fetch('https://api.example.com/data');
    const data = await response.json();
    return data;
  }

  const data = await getData();
</script>
```

Ini jalan, tapi perilakunya beda karena fetch ini dijalankan saat component sedang di init. Kalo terjadi error saat call api component tidak akan muncul di layar dan akan error di console. Cara yang valid adalah fetch dijalankan ketika component selsai di init => call api => render. Cara ini disebut **lifecycle hooks**.

Untuk lebih jelasnya bisa lihat di [https://svelte.dev/docs/svelte/lifecycle-hooks](https://svelte.dev/docs/svelte/lifecycle-hooks).

### Mounted Component `onMount`

Lifecycle hooks pertama yang bakal gue bahas yaitu `onMount`. `onMount` ini dipanggil ketika component selesai di init dan siap di render di browser. Nah inilah yang cocok ketika Lo pingin ambil data dari api. Kalo error bisa Lo kasih message dan success bisa Lo tampilin datanya dilayar.

```html
<!-- Example -->
<script>
  onMount(async () => {
    try {
      const response = await fetch('https://api.example.com/data');
      const data = await response.json();
      // Do something with the data
    } catch (error) {
      console.error(error);
    }
  });
</script>
```

Sebelumnya data pada kasus Contact Book di tulis manual di code. Nah sekarang ceritanya Lo pura - pura ambil dari api dengan cara bikin file `user.json` di folder `public`.

```json
// public/user.json
{
    "users": [
        { "id": 1, "name": "Snake System", "address": "Jakarta", "age": 9, "phone": "08123456789" },
        { "id": 2, "name": "Feri Irawansyah", "address": "Semarang", "age": 25, "phone": "08123456789" },
        { "id": 3, "name": "Satria Baja Ringan", "address": "Bandung", "age": 34, "phone": "08123456789" }
    ]
}
```

Aktifkan set di `src/stores/users.js`

```js
// src/stores/users.js
const createUserStore = () => {
    const { subscribe, set, update } = writable([]);

    return {
        subscribe,
        set,
        // ...
    };
};
```

Buat lifecycle hooks `onMount` di `src/lib/UserRow.svelte` seperti berikut:

```html
<!-- src/lib/UserRow.svelte -->
 <script>
  import { getContext, onMount } from "svelte";
  import { users } from "../stores/user";

  onMount(async () => {
    const response = await fetch("/user.json");
    const result = await response.json();
    users.set(result.users);
  })

  const onEdit = getContext("on-edit");

</script>
```

### Unmounted Component `onDestroy`

Tadi ketika mount sekarang adalah ketika unmount. Ketika component di unmount maka lifecycle hooks `onDestroy` akan dijalankan. Kalo Lo pingin kasih message ketika component di unmount bisa Lo kasih di sini. Sebelumnya Lo punya component `AddUser.svelte` dan `EditUser.svelte` yang mana ketika Lo klik tombol Update maka component `AddUser` akan di unmount dan component `EditUser` akan di mount.

```html
<!-- src/lib/AddUser.svelte -->
 <script>
    import { getContext, onDestroy } from "svelte";
    
    // ... seperti code sebelumnya

    const onEdit = getContext("on-edit");

    onDestroy(() => {
        onEdit.message = "Edit sedang berlangsung, tambahkan user diblokir.";
    });
</script>
```

Lalu taro messagenya di edit user seperti berikut:

```html
<!-- src/lib/EditUser.svelte -->
</script>

<p>{onEdit.message}</p> <!-- Letakkan di sini -->

<form style="display: flex;" onsubmit={handleSubmit}>
```

<img class="img-fluid" alt="destroy" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/destroy.png" />

### DOM Updated `tick`

Sebelumnya di Svelte 3/4 ada yang namanya `beforeUpdate` dan `afterUpdate`. Ini digunakan ketika suatu component mengalami perubahan. `beforeUpdate` akan dijalankan ketika component sedang di render dan `afterUpdate` akan dijalankan ketika component selesai di render. Tapi sekarang udah deprecated wkwkwk. Di Svelte 5 ada yang namanya `tick`. `tick` ini tidak menggantikan 100% tapi juga punya fitur dari keduanya + ada fitur sakti dimana bisa Lo pakein asyncronous operation di `tick` untuk memantau DOM bener - bener sudah update.

```html
<script>
	import { tick } from 'svelte';

	$effect.pre(() => {
		console.log('the component is about to update');
		tick().then(() => {
				console.log('the component just updated');
		});
	});
</script>
```

Untuk contoh update DOM tanpa tick kaya gini

```html
<script>
  let open = false;

  function show() {
    open = true;
    console.log(document.querySelector('.panel'));
  }
</script>

{#if open}
  <div class="panel">Hello</div>
{/if}

<button onclick={show}>Show</button>
```

<img class="img-fluid" alt="no-tick" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/no-tick.png" />

Kalo Lo klik maka di console akan `null`. Kenapa bisa terjadi? Karena DOM nya belum update. Untuk memantau DOM sudah update bisa Lo pakein `tick` seperti berikut:

```html
<script>
  import { tick } from "svelte";

  let open = false;

  async function show() {
    open = true;
    await tick(); // ⏳ tunggu DOM update
    console.log(document.querySelector(".panel"));
  }
</script>

{#if open}
  <div class="panel">Hello</div>
{/if}
<button onclick={show}>Show</button>
```

<img class="img-fluid" alt="tick" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/tick.png" />

</details>

<details>
<summary><h2>Referensi 📚</h2></summary>

Catatannya udah nyampe sini, sebenarnya masih ada beberapa yang belum gue bahas tentang Svelte. Tapi casenya lebih advance dan jarang digunakan. Di catatan ini hanya fokus ke fitur yang ada di Svelte 5 yang sering digunakan. Kalo Lo pingin lebih detail bisa lihat di referensi ini.

- [Svelte](https://svelte.dev/)
- [Wiki Svelte](https://en.wikipedia.org/wiki/Svelte)

</details>

---

<div class="d-flex flex-row justify-content-center align-items-center">Regards <a href="https://feri-irawansyah.my.id"><img style="width: 1rem; height: 1rem;" src="https://feri-irawansyah.my.id/favicon.ico" alt="Feri Irawansyah"> Feri Irawansyah</a></div>

---