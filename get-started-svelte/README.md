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

Di baris pertama ada `import { mount } from 'svelte'` function `mount` ini digunakan untuk merender suatu Component yaitu `App` ke dalam Element HTML dengan id `app`. Nah kalo Lo buka file `index.html` di root project Lo ada file HTML dengan element `<div id="app"></div>`.

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
    <div id="app"></div> <!-- Element di panggil di file main.js -->
    <script type="module" src="/src/main.js"></script> <!-- File main.js di panggil di file index.html -->
  </body>
</html>
```

Jadi App Lo di taro di element HTML div ini. Nah pada catatan ini kita bakal pake `mount` untuk merender App ke dalam Element HTML.

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
  target: document.getElementById('app'),
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

<details open>
<summary><h2>📌 Template HTML</h2></summary>

File Svelte atai `.svelte` sebenarnya sama seperti HTML biasa Lo bisa mmenuliskan tag html terserah Lo ga ada aturan harus di bungkus pake `<></>` atau ada component khusus seperti `<Fragment></Fragment>` engga bro, file Svelte sama kaya file HTML. Bedanya ada fitur - fitur tambahan buat mempermudah hidup Lo.

File `.svelte` juga ga mewajibkan Lo buat nutup tag HTML, Nah di Framework lain seperti React atau Leptos Lo wajib nutup tag HTML `Self Closing Tag` seperti `<input />`, `<img />`, `<hr/>`. Kecuali ketika Lo manggil suatu Component seperti `<Counter />` Lo wajib pake tutup tag HTML.

Jadi biar konsisten direkomendasikan buat tetep pake `Self Closing Tag` aja.

### Text Expression

Fitur pertama adalah `Text Expression` ini diguakan untuk mengakses langsung suatu data dari Javascript. Ouh iya sebenarnya fitur pertama di file .svelte ini Lo bisa langsung akses suatu data dari tag `<script></script>` dan tag `<style></style>` juga bisa langsung akses suatu data. Nah dengan begini `Text Expression` lebih mudah digunakan.

Fitur lainnya di Text Expression ini Lo bisa juga lakuin function, method, dan juga object dari Javascript. Seperti `toUpperCase`, `toLowerCase`, `concat`, `split`, `slice`, dan masih banyak lagi.

Untuk cara pakenya itu Lo bisa pake kurung kurawal `{disini valuenya}`. Coba Lo buka file `HelloWorld.svelte` di folder `lib` dan ubah code nya menjadi ini

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

### HTML Tags

Saat menggunakan `Text Expression` di svelte itu udah aman dari `XSS (Cross Site Scripting)`. Jadi misal Lo mau nampilin text yang didalemnya ada tag HTML Svelte bakal lakuin escape text dulu baru di render.

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

</details>

<details open>
<summary><h2>📌 Rune</h2></summary>

`Rune` adalah suatu simbol atau keyword di Svelte yang akan mengontrol Svelte Compiler seperti di awal catatan bahwa Svelte akan melakukan kompilasi code nya ke Javascript DOM murni tanpa Runtime seperti Virtual DOM. Oleh karena itu Rune itu sangat penting untuk mengontrol Svelte Compiler saat menggunakan suatu data, state atau element.

`Rune` memiliki awalan/prefix `$` dan kalo Lo pake VS Code akan terlihat seperti sebbuah function seperti `$state("hello");` artinya Lo pake `Rune state` yang memiliki parameter `hello`. 

Di svelte tersedia banyak Rune dan punya fiturnya masing-masing seperti `$state` contohnya. Lo bisa langsung kunjungi ke website nya di <a href="https://svelte.dev/docs/svelte/$state" target="_blank">Rune State</a> untuk melihatnya.

### Rune State `$state()`

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

Biar ga terlalu bingung karena di `App.svelte` ada banyak element, kita buat halaman baru aja. Coba Lo buat `counter.html` dan `counter.js` seperti ini:

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
  target: document.getElementById('app'),
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
    document.getElementById("count").innerHTML = "" + count;
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

### Rune `$derived()`
</details>