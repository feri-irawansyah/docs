Woi bro..., Gue tau yang ada di benak lo, pas lo mau bikin tampilan web yang bagus, dinamis, dan banyak populer mesti lo bakal milih **React**. Dan kalo lo pingin yang mudah dipahami oleh programmer yang baru nyemplung tapi pingin cepet - cepet bikin **Web Application Using Frontend Framework** lo mesti bakal nyletuk, gue mau pake **Vue** aja ah....

Perlu gue akui 2 benda itu bagus 👍, modern technology 🤖 dan bisa buat fullstack application bro. Okeh gue mau minggir dulu sebentar mau bahas Frontend Framework yang ASING, JARANG DIPAKE, MINIMALIS, CURANG, SERASA KAYA NGOPLOS HTML + JS. Iyaaa, kita bakal bahas Svelte.

<img class="img-fluid" alt="image" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/svelte-wiki-1.png" />

Dikutip dari <a href="https://en.wikipedia.org/wiki/Svelte" target="_blank">Wikipedia</a> Svelte ini dibuat oleh Bapak - Bapak yang namanya <a href="https://x.com/rich_harris" target="_blank">Rich Harris</a> dan Kroco - Krocony tentunya Svelte Team. Dan Svelte ini langsung di compile ke **JS DOM**, tanpa Runtime, Hasil Kompilasi **Mini Size** dan ga kaya **React** atau **Vue** yang pake Virtual DOM katanya. Serasa bikin murni javascript? Tapi Declarative? Dan tanpa cari-cari class atau id bahkan element?. Wow minimalis sekali tapi apakah sepowerfull itu? Okeh kita coba sekarang.

<details>
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
Nah apa itu, baru masuk langsung dapet bahasa Alien👽. Tenang bro, itu cuma overview doang kurang lebih codenya seperti itu, script, style, dan html di oplos jadi satu kaya Vue? Iyes bro betul svelte ini arsitekturnya mirip Vue JS ga perlu class component, functional component, atau ya semacam itulah. Jadi semua file dengan extention gampangnya format lah nyebutnya `.svelte` itu adalah component.

</details>

<details>
<summary><h2>📌 Get Started Svelte</h2></summary>

Sebelum Lo mulai membuat Svete Project, Lo perlu beberapa hal yang harus dipenuhi dulu
### Pre Requisites
- Pemahaman tentang Fundamental Javascript
- <a href="https://nodejs.org/" target="_blank" rel="">NodeJS </a> (npm, yarn, deno, bun, pnpm)
- VS Code (bisa cod eeditor lain)
- Pemahaman fundamental Vite

### Vite & Svelte

Untuk membuat Svelte project kita akan menggunakan <a href="https://vite.dev/" target="_blank" rel="">Vite</a> frontend build tools. Sebenarnya ada banyak build tools seperti gulp, webpack, rollup dll. Tapi framework modern hampir mayoritas menggunakan Vite sebagai default nya.

### Membuat project Svelte 

Untuk membuat project Svelte Lo cukup buka terminal dan arahkan ke folder dimana Lo pingin nyimpen source aplikasi nya lalu ketikkan

```bash
npm create vite@latest get-started-svelte
```

Nanti ada beberapa pilihan kaya gini
</details>

