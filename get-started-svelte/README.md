Woi bro..., Gue tau yang ada di benak lo, pas lo mau bikin tampilan web yang bagus, dinamis, dan banyak populer mesti lo bakal milih **React**. Dan kalo lo pingin yang mudah dipahami oleh programmer yang baru nyemplung tapi pingin cepet - cepet bikin **Web Application Using Frontend Framework** lo mesti bakal nyletuk, gue mau pake **Vue** aja ah....

Perlu gue akui 2 benda itu bagus 👍, modern technology 🤖 dan bisa buat fullstack application bro. Okeh gue mau minggir dulu sebentar mau bahas Frontend Framework yang ASING, JARANG DIPAKE, MINIMALIS, CURANG, SERASA KAYA NGOPLOS HTML + JS. Iyaaa, kita bakal bahas Svelte.

<img class="img-fluid" alt="image" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/svelte-wiki-1.png" />

Dikutip dari <a href="https://en.wikipedia.org/wiki/Svelte" target="_blank">Wikipedia</a> Svelte ini dibuat oleh Bapak - Bapak yang namanya <a href="https://x.com/rich_harris" target="_blank">Rich Harris</a> dan Kroco - Krocony tentunya Svelte Team. Dan Svelte ini langsung di compile ke **JS DOM**, tanpa Runtime, Hasil Kompilasi **Mini Size** dan ga kaya **React** atau **Vue** yang pake Virtual DOM katanya. Serasa bikin murni javascript? Tapi Declarative? Dan tanpa cari-cari class atau id bahkan element?. Wow minimalis sekali tapi apakah sepowerfull itu? Okeh kita coba sekarang.

<details>
<summary><h2>📌 Setup Project Svelte</h2></summary>

Pertama lo coba ketikan di mesin pencarian lo `Svelte` atau pergi aja ke alamat <a href="https://svelte.dev" target="_blank">https://svelte.dev</a> lalu ke pergi ke <a href="https://svelte.dev/docs/svelte/overview" target="_blank">docs Svelte</a>. Atau silahkan coba - coba main disitu, asal jangan tersesat dijalan Pidana, Kejahatan dan Korupsi. 

<img class="img-fluid" alt="image" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/get-started-svelte/public/docs-svelte.png" />

Nah setalah masuk ke webnya dan lo pencet yang Svelte bukan Sveltekit ya, nanti gue bakal buatin terpisah untuk Sveltekit. Pas lu masuk langsung di sugihkan dengan code

```ts
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
Nah apa itu, baru masuk langsung dapet bahasa Alien👽. Tenang bro, itu cuma overview doang kurang lebih codenya seperti itu, script, style, dan html di oplos jadi satu kaya Vue? Iyes bro betul svelte ini arsitekturnya mirip Vue JS ga perlu class component, functional component, atau ya semacam itulah. Jadi semua file dengan extention gampangnya format lah nyebutnya `.svelte` itu adalah component

</details>

