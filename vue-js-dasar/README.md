Woiii brooo... di thumbnail catetan ini harusnya Lo udah liat, disini gue bikin catetan tentang dasar - dasar Vue JS. Disini gue ga jualan ya, ngapain juga gue jualan digaji juga kaga. Tapi intinya disini gue mau berbagi tentang apa yang gue pelajari soal Vue JS.

Kenapa si harus pake Framework pake HTML, CSS, JS juga kelar. Yaaa emng wkwkwk, Lo bikin aplikasi pake 3 anuan itu iya anuan lah pokoknya yang Lu anuin. Tapi itu kalo Lo kerja tim beda cerita bro, tiap individu - individu termasuk Lo itu punya gaya ngoding sendiri. 

Kecuali Lo ini bener - bener suhunya dan Lo bisa guide 1 tim biar ngikutin cara Lo ngoding mungkin lain cerita, tapi realitanya kadang di tim yang udah kaya keluarga pun masih sruduk - srudukan kek orang abis kerasukan kuda lumping. Lo ngajarin eh taunya malah di kucilin😂. Nah disinilah perlu adanya Framework untuk `Standarisasi`. Jadi ga bisa tuh Lo senggol - senggolan atau `adu mekanik`, karena Lo wajib ngikutin standar si frameworknya.

<summary><h2>Salam Kenal Dari Vue JS 📚</h2></summary>

### Documentation Vue JS

<img class="img-fluid" alt="documentation-vuejs" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/documentation-vuejs.png">

<a href="https://vuejs.org/" target="_blank" rel="noopener noreferrer">https://vuejs.org/</a>

### Sejarah Vue JS

Vue ini diperkenalkan pada tahun 2013, udah lumayan tua lah ya 13 tahun dari catatan ini dibuat. Pertama kali di buat sama bapak <a href="https://github.com/yyx990803" target="_blank" rel="noopener noreferrer">Evan You</a> sekarang masih di maintain sama beliau juga beserta antek - anteknya. Dan Vue JS ini open source gratis tanpa di pungut biaya kalo Lo mau pake atau kalo Lo mau jadi contributor Lo bisa ikut join di repositorynya Vue JS <a href="https://github.com/vuejs" target="_blank" rel="noopener noreferrer">https://github.com/vuejs</a>.

### Ekosistem Vue JS

Vue JS bikin ekosistemnya sendiri, jadi kalo Lo butuh sesuatu fitur yang ga ada di core nya Vue JS, ekosistemnya sudah menyediakannya. Ga kaya React yang ekosistemnya gede banget dan luas nyari apa aja ada tapi minusnya misalnya core React nya update dependensi yang Lo pake belum tentu compatible sama React nya.

Vue JS engga bro, timnya bikin ekosistemnya sendiri, jadi buat jangka panjang aman karena misal corenya update, dependensi lainnya juga akan menyesuaikan dengan Vue JS.

### Component Based

Hampir semua frontend library/framework modern semuanya pake Component Base. Apa tuh? Component adalah suatu bagian, element atau unit `mandiri` artinya terisolasi dari component lain dimana didalamnya bisa berisi suatu fungsi, design atau data.

Gambarannya gini Lo bikin web pake component, artinya website Lo itu di buat dari pretelan - pretelan block lego, nah terus pretelan itu Lo susun sampe jadi pretelan yang lebih gede sampe jadi suatu halaman. Kalo Lo bandingan pake native js, Lo kaya lagi bikin patung pake tanah liat cok, Lo basahin, poles - poles, basahin lagi, bentuk anuan sampe jadi, tapi itu kek dewa bisa jadi malah bentuknya kaya jombie.

Nah dekan pendekatan component, Lo harusnya bakal lebih mudah bikinnya.

</details>

<summary><h2>Getting Started 📚</h2></summary>

Biasa kalo Lo mau bikin aplikasi Lo perlu siapin beberapa sesajen dulu bro biar khusyuk.

### Pre Requisites

- Tau fundamental HTML, CSS, JS. (Ini wajib, Lo harus tau 3 anuan ini dulu sebelum pake Framework)
- <a href="https://nodejs.org/" target="_blank" rel="noopener noreferrer">NodeJS</a>
- <a href="https://code.visualstudio.com/" target="_blank" rel="noopener noreferrer">VS Code</a> (Lo bisa pake code editor lain. Tapi menurut gue paling enak pake VS Code)
- <a href="https://vitejs.dev/" target="_blank" rel="noopener noreferrer">Vite</a> Fundamental.

### Membuat Project

Di catatan ini gue pake Vite JS, karena hampir semuanya frontend Framework di Javascript pake ViteJS, tapi Lo juga bisa pake yang lainnya kaya webpack, gulp, rollup dll. 

Buat bikin projectnya Lo bisa ketikkan command ini di terminal:

```bash
npm create vite@latest get-started-vue
```

Nanti bakal ada wizard CLI buat memilih konfigurasi project Vue Lo.

```bash
F:\project>npm create vite@latest get-started-vue

> npx
> create-vite get-started-vue

|
*  Select a framework:
|    Vanilla
|  > Vue
|    React
|    Preact
|    Lit
|    Svelte
|    Solid
|    Qwik
|    Angular
|    Marko
|    Others
—
*  Select a variant:
|    TypeScript
|  > JavaScript
|    Official Vue Starter ↗
|    Nuxt ↗
|    Vike 
*  Use rolldown-vite (Experimental)?:
|    Yes
|  > No
*  Install with npm and start now?
|  > Yes /   No
o  Scaffolding project in F:\project\get-started-vue...
|
o  Installing dependencies with npm...

added 35 packages, and audited 36 packages in 18s

6 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
|
o  Starting dev server...

> get-started-vue@0.0.0 dev
> vite


  VITE v7.3.1  ready in 5662 ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
  ➜  press h + enter to show help
```

Nah sekarang coba Lo buka url http://localhost:5173, Nah Lo udah bikin Vite + Vue project.

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/get-started-vue.png" class="img-fluid" alt="get-started-vue"/>

</details>