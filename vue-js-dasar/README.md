Woiii brooo... di thumbnail catetan ini harusnya Lo udah liat, disini gue bikin catetan tentang dasar - dasar Vue JS. Disini gue ga jualan ya, ngapain juga gue jualan digaji juga kaga. Tapi intinya disini gue mau berbagi tentang apa yang gue pelajari soal Vue JS.

Kenapa si harus pake Framework pake HTML, CSS, JS juga kelar. Yaaa emng wkwkwk, Lo bikin aplikasi pake 3 anuan itu iya anuan lah pokoknya yang Lu anuin. Tapi itu kalo Lo kerja tim beda cerita bro, tiap individu - individu termasuk Lo itu punya gaya ngoding sendiri. 

Kecuali Lo ini bener - bener suhunya dan Lo bisa guide 1 tim biar ngikutin cara Lo ngoding mungkin lain cerita, tapi realitanya kadang di tim yang udah kaya keluarga pun masih sruduk - srudukan kek orang abis kerasukan kuda lumping. Lo ngajarin eh taunya malah di kucilin😂. Nah disinilah perlu adanya Framework untuk `Standarisasi`. Jadi ga bisa tuh Lo senggol - senggolan atau `adu mekanik`, karena Lo wajib ngikutin standar si frameworknya.

<details>
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

Gambarannya gini Lo bikin web pake component, artinya website Lo itu di buat dari pretelan - pretelan block lego, nah terus pretelan itu Lo susun sampe jadi pretelan yang lebih gede sampe jadi suatu halaman. Kalo Lo bandingan pake native js, Lo kaya lagi bikin patung pake tanah liat cok, Lo basahin, poles - poles, basahin lagi, bentuk anuan sampe jadi, tapi itu Lo harus punya skill dewa dan kalo biasa - biasa aja bisa jadi malah bentuknya kaya jombie.

Nah dekan pendekatan component, Lo harusnya bakal lebih mudah bikinnya.

</details>

<details>
<summary><h2>Getting Started 📚</h2></summary>

Biasa kalo Lo mau bikin aplikasi Lo perlu siapin beberapa sesajen dulu bro biar khusyuk.

### Pre Requisites

- Tau fundamental HTML, CSS, JS. (Ini wajib, Lo harus tau 3 anuan ini dulu sebelum pake Framework)
- <a href="https://nodejs.org/" target="_blank" rel="noopener noreferrer">NodeJS</a>
- <a href="https://code.visualstudio.com/" target="_blank" rel="noopener noreferrer">VS Code</a> (Lo bisa pake code editor lain. Tapi menurut gue paling enak pake VS Code)
- <a href="https://code.visualstudio.com/docs/nodejs/vuejs-tutorial#_vue-official-extension" target="_blank" rel="noopener noreferrer">Vue Official Extention</a>
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

### Vite + Vue Project Structure

Untuk Arsitektur projectnya akan seperti ini:

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/structure-vite.png" class="img-fluid" alt="structure-vite"/>

Ini memang structure project yang Vite JS buat, untuk belajar dasar - dasar Vue JS. Nantinya ketika Lo mau bikin aplikasi Vue JS Lo ga bakal pake arsitektur ini, karena ada arsitektur khusus yang udah dibuatin sama tim Vue JS. Nanti bakal gue bahas.

### Component Vue JS

Kalo Lo buka file `App.vue` isinya harusnya gini:

```html
<script setup>
import HelloWorld from './components/HelloWorld.vue'
</script>

<template>
  <div>
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo" alt="Vite logo" />
    </a>
    <a href="https://vuejs.org/" target="_blank">
      <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
    </a>
  </div>
  <HelloWorld msg="Vite + Vue" />
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
```

Ini adalah struktur component di Vue JS. Kalo Lo sebelumnya pake React Lo mesti harus bikin suatu function atau class untuk component. Nah di Vue engga bro, Lo hanya perlu bikin file dengan format `.vue` maka file itu udah jadi component. DIdalamnya wajib ada tag`<template>` dan `<script setup>`.

```html
<script setup>
    // code Javascript
</script>
<template>
  <!-- code HTML -->
</template>
```

- Tag `<script setup></script>` ini wajib punya atribut `setup`. Untuk menandakan bahwa didalamnya Lo akan menuliskan fitur khusus punya Vue. Kalo Lo hapus maka fitur Vue tidak akan jalan dan akan di anggap tag `<script>` biasa.
- Tag `<template>` ini wajib punya atribut `template`. Untuk menandakan bahwa didalamnya Lo akan menuliskan HTML. Kalo Lo hapus maka akan error. Karena Vue JS hanya mengenali element yang ada dalam tag `<template>`.
- Tag `style` ini optional, bisa ada bisa tidak.

### Create App Vue JS

Kalo Lo buka file main.js isinya gini:

```js
import { createApp } from 'vue'
import './style.css'
import App from './App.vue'

createApp(App).mount('#app')
```

Function `createApp` ini digunakan untuk membuat App Vue JS. Dimana aplikasi Vue JS Lo itu jalan di element dengan id `app`. Element itu ada di file `index.html`:

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>get-started-vue</title>
  </head>
  <body>
    <div id="app"></div> <!-- elemen di panggil di file main.js -->
    <script type="module" src="/src/main.js"></script>
  </body>
</html>
```

### Membuat Component Vue JS

Sebeperti sebelumnya ketika membuat file dengan extention `.vue` maka file tersebut akan menjadi sebuah component. Okeh sekarang coba Lo buat beberapa file baru.
- `src/components/HelloVue.vue`
- `src/hello.js`
- `hello.html`

```html
<!-- src/components/HelloVue.vue -->
<script>
    console.log('Hello Vue');
</script>

<template>
    <h1>Hello Vue</h1>
</template>
```

```js
// src/hello.js
import { createApp } from 'vue'
import HelloVue from './components/HelloVue.vue'

createApp(HelloVue).mount('#hello')
```

```html
<!-- hello.html -->
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Hello Vue</title>
  </head>
  <body>
    <div id="hello"></div> <!-- elemen di panggil di file hello.js -->
    <script type="module" src="/src/hello.js"></script>
  </body>
</html>
```

```js
// vite.config.js
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  build: {
    rollupOptions: {
      input: {
        main: 'index.html',
        hello: 'hello.html',
      }
    }
  }
})
```

Nah sekarang coba Lo buka url http://localhost:5173/hello, Nah Lo udah bikin Hello Vue.

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/hello-vue.png" class="img-fluid" alt="hello-vue"/>

Disini component gue ini tanpa tag `style` dan tapi jalan aman tanpa setup karena gue hanya pake code Javascript biasa, beda cerita kalo misalnya gue pake fitur - fitur dari Vue JS. Defaultnya 1 file `.vue` itu Single File Component (SFC) jadi componentnya akan mengikuti nama filenya, makanya di rekomendasikan menggunakan format PascalCase biar punya standarisasi.

Kalo component di panggil di HTML maka akan jadi `<HelloVue />` seperti tag Html biasa. Untu tag `<script setup></script>` direkomendasikan pake atribut `setup` jadi gue bisa pake fitur khusus dari Vue JS.

</details>

<details>
<summary><h2>Templating Engine 📚</h2></summary>

Vue menggunakan Templating Engine berupa component base, nah didalam templating ini ada banyak fitur yang Vue sediain buat Lo pake.

### Component API Style

Di catatan gue ini sekarang Vue menyediakan 2 API Style yaitu Options API dan Composition API.

#### Options API

Option API ini ada sejak Vue versi 2 dan masih stable sampai Vue 2, jadi kalo Lo pake style ini masih compatible asalkan jangan di campur dengan composition api.

Disebut Options API karena Lo akan membuat Logic Component menggunakan Object Options, yang berisikan data, method dan mounted. Jadi nanti Lo bikin logic di scriptnya pake bentuk object. Keliatannya rapi ya karena didalan satu object nanti tinggal di panggil objectnya.

Contoh:
```html
<script>
export default {
    // Define data atau state pake keyword data
    data() {
        return {
            count: 0
        }
    },

    // Define method
    methods: {
        increment() {
            this.count++
        }
    },

    // Define mounted atau lifecycle
    mounted() {
        console.log(`The initial count is ${this.count}.`)
    }
}
</script>

<template>
  <button @click="increment">Count is: {{ count }}</button>
</template>
```

Tapi kalo kasusnya makin gede kompleks gitu ini akan lebih ribet, misalnya ada banyak state itu Lo harus define keyword `data` nambah method juga sama. Code script mungkin bisa jadi akan lebih panjang kalo logicnya semakin banyak.

#### Composition API

Composition API ini ada sejak Vue versi 3 dan masih stable sampai Vue 3, jadi pake style composition ini Lo udah ga perlu bikin pake object options lagi. Vue udah nyediain beberapa api yang bisa Lo pake. Kurang lebihnya kaya gini untuk compotition api.

```html
<script setup>
import { ref, onMounted } from 'vue'

// reactive state
const count = ref(0)

// functions or methods
function increment() {
  count.value++
}

// lifecycle hooks
onMounted(() => {
  console.log(`The initial count is ${count.value}.`)
})
</script>

<template>
  <button @click="increment">Count is: {{ count }}</button>
</template>
```

Codenya terlihat lebih sedikit meskipun tidak dibungkus dalam 1 object, tapi pada intinya sama. DI catatan ini gue bakal sering pake Vue 3 artinya bakal pake Composition API.

### Template

Sebelumnya Lo harusnya udah beberapa kali pake `template` tag, nah disinilah Lo bisa pake HTML di dalam tag `<template></template>`. Tapin sebenernya yang terjadi itu tidak langsung tampil di HTML, sebenernya Vue akan melakukan kompilasi dulu jadi code javascript, nah jadi element yang ditampilkan itu adalah element HTML yang dibuat pake Javascript, bukan langsung di tulis ke file html.

```html
<template>
  <h1>Hello Vue</h1>
  <p>Gue adalah Satria Baja Ringan</p>
</template>
```

### Text Interpolation / Text Expression

Salah satu feature yang ada di templating engine Vue adalah `text interpolation`, atau orang kadang menyebutnya `text expression`. Fitu ini berfungsi menampilkan suatu data ke element HTML. 

- Text Interpolation menggunakan <a href="https://mustache.github.io/" target="_blank" rel="noopener noreferrer">Mustache</a> format yaitu `{{}}` pake kurung kurawal 2 kali.
- Text Interpolation ini akan menampilkan data dalam bentuk plain text, jadi bukan code atau element html.

```html
<!-- src/components/HelloWorld.vue -->
 <script setup>
    const name = 'Satria Baja Ringan'
    const heading = '<h1>Hello Vue</h1>'
</script>

<template>
    {{ heading }}
    <p>My name is {{ name }}</p>
</template>
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/text-interpolation.png" class="img-fluid" alt="text-interpolation"/>

Nah dengan ini artinya aman dari serangan XSS (Cross Site Scripting.), jadi misalnya ada yang iseng gitu kirim kode text dalam bentuk HTML, Vue ga akan tuh nampilin dalam bentuk HTML, tetapi akan nampilin sebagai plain text.

Lo juga bisa pake method - method Javascipt di dalam `{{}}`. Tapi ga semua method bro, cuma beberapa aja nah lebih detailnya Lo bisa check disini bro <a href="https://github.com/vuejs/core/blob/main/packages/shared/src/globalsAllowList.ts#L3" target="_blank" rel="noopener noreferrer">Javascript Expressions</a>. Ini benerapa code Javascript yang ga boleh dipake di dalam `{{}}`.

```html
<p>My name is {{ name.toUpperCase() }}</p>
```

### Raw HTML

Tapi kadang Lo juga mesti ada tuh perlu menampilkan suatu HTML text, tapi sayangnya di Text Interpolation ini ga bisa. Vue punya directive untuk menanganinya yaitu `v-html`. `v-html` ini menjadi atribut HTML, Jadi yang sebelumnya `{{ heading }}` yang isinya `<h1>Hello Vue</h1>` maka akan tampil.

Di Vue atribut yang di awali dengan `v-` ini disebut directive, ada banyak directive yang bisa Lo pake. Nanti bakal kita coba satu-satu.

```html
<!-- src/components/HelloWorld.vue -->
<script setup>
    const name = 'Satria Baja Ringan'
    const heading = '<h1>Hello Vue</h1>'
</script>

<template>
    <div v-html="heading"></div>
    <p>My name is {{ name }}</p>
</template>
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/v-html.png" class="img-fluid" alt="v-html"/>

### Attributes Binding

Mustache ga bisa digunakan pada attribute di Element kalo Lo pingin pake variable pada attribute di Element, Lo perlu menggunakan Directive `v-bind:nama-attribute`. Misalnya kaya Lo mau pake attribute `class` di Element HTML yang isinya itu variable.

Nah ini mungkin bakal sering banget dipake jadi Vue ini nyediain shortcut untuk membuat attribute binding `v-bind:nama-attribute` jadi `:nama-attribute`.

Termasuk juga untuk value yang berupa boolean. Misalnya kalo Lo mau pake attribute `disabled` di Element HTML, Lo bisa pake `v-bind:disabled` atau `:disabled` juga.

```html
<!-- src/components/HelloWorld.vue -->
<script setup>
    const name = 'Satria Baja Ringan'
    const heading = '<h1>Hello Vue</h1>'
    const classHeading = 'heading'
    const className = 'name'
    const disabledButton = true
</script>

<template>
    <div v-html="heading" v-bind:class="classHeading"></div>
    <p :class="className">My name is {{ name }}</p>
    <button :disabled="disabledButton">Submit</button>
</template>

<style scoped>
    .heading {
        color: red;
    }
    .name {
        color: green;
    }
</style>
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/v-bind.png" class="img-fluid" alt="v-bind"/>

</details>

<details open>
<summary><h2>Reactive State 📚</h2></summary>

Lo kalo bikin website mesti bakal nyimpen data, state atau keadaan di Javascript, misalnya Lo pingin bikin angka yang misalnya kalo Lo pencet tombol nanti angkanya nambah 1. Contohnya gini, coba Lo bikin halaman baru sama kaya sebelumnya bikin file `src/components/Counter.vue`, `src/counter.js`, `counter.html` terus daftarin di `vite.config.js`:

```html
<!-- src/components/Counter.vue -->
<script setup>
    let count = 0;

    function increment() {
        count++;
        document.getElementById("count").innerText = `Count: ${count}`;
    }
</script>

<template>
    <h1 id="count">Count: {{ count }}</h1>
    <button v-on:click="increment">Increment</button>
</template>
```

### Ref API

Cara kaya gini biasa di JavaScript jadi Lo bikin mutable variable terus Lo ubah pake JS DOM. Nah di Vue ada cara yang lebih baik dan Lo udah ga perlu lagi pake DOM Manipulation. Namanya adalah `Reactive State`. Nah terus apa bedanya sama DOM Manipulation? Bedanya ketika Lo pake Reactive State, Vue akan melakukan render ulang component nya, jadi componentnya kaya di refresh ketika terjadi perubahan data.

Lo bisa pake keyword `ref` dari Vue untuk membuat reactive state. Kalo misalnya Lo mau bikin reactive state `count` maka bisa pake `const count = ref(0)`. Tapi ga itu doang, ada beberapa Reactive State yang bisa Lo pake, detaionya ada disini : <a href="https://vuejs.org/api/reactivity-core.html" target="_blank" rel="noopener noreferrer">https://vuejs.org/api/reactivity-core.html</a>.

```html
<!-- src/components/Counter.vue -->
<script setup>
  import { ref } from 'vue';

  let count = ref(0);
  console.log('Loaded Counter.vue', count);

  function increment() {
    count.value++;
  }
</script>

<template>
  <h1 id="count">Count: {{ count }}</h1>
  <p>{{ Math.random() }}</p>
  <button v-on:click="increment">Increment</button>
</template>
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/ref-state.png" class="img-fluid" alt="ref-state"/>

Harusnya pak Lo klik maka `<p>{{ Math.random() }}</p>` juga akan berubah, ini karena Vue akan merender ulang component nya. Nah tapi hanya componentnya aja yaitu pada `<template></template>`, untuk `<style></style>` dan `<script></script>` tidak akan berubah hanya akan dirender sekali aja.

Saat membuat state pake `ref` Vue akan membuat statenya menjadi object, jadi kalo Lo pingin mengubah nilai Lo bisa ubah object `value` tapi ada beberapa object lain nya. Tapi ketika statenya di render di element, maka ga perlu nyebutin objectnya, Lo cukup render statenya aja kaya `{{ count }}`. Disini Lo mesti bertanya, Kenapa si kok harus pake Reactive State? Pake DOM juga bisa kan?

Jawaban simplenya ya ngapain Lo pake Vue😂. 

Reactive state akan selalu di pantau sama Vue nya, jadi ketika ada perubahan di state Lo, Vue langsung tau. Vue menggunakan object dengan attribute value sebagai State, agar bisa meng-intercept perubahan data dari get dan set operations, sehingga dengan mudah Vue bisa mendeteksi State mana yang berubah, dan melakukan render ulang Component tersebut di DOM. <a href="https://javascript.info/property-accessors" target="_blank" rel="noopener noreferrer">https://javascript.info/property-accessors</a>

Selain Lo pake single Value, Lo juga bisa lakuin ke Object, Array, atau Map misalnya gini:

```html
<!-- src/components/Counter.vue -->
<script setup>
    import { ref } from 'vue';

    let counter = ref({
        count: 0,
        name: 'Satria'
    });
    console.log('Loaded Counter.vue', counter);

    function increment() {
        counter.value = {
            ...counter.value,
            count: counter.value.count + 1
        }
        
    }
</script>

<template>
    <h1 id="count">Count: {{ counter.name }} {{ counter.count }}</h1>
    <p>{{ Math.random() }}</p>
    <button v-on:click="increment">Increment</button>
</template>
```

### Lifecycle DOM

Kalo Lo ubah state di Vue, sebenernya Vue juga ga akan lakuin render saat itu juga bro. Jadi ada jadwalnya buat lakuin render, nah ini bagus misalnya ada anyak state dalam satu waktu berubah bareng Vue bakal nunggu sampai semua state selesai beruah, baru Vue akan melakukan render ulang.

Jadwal Vue lakuin render ini di sebut `next tick`, Jadi ada keyword `nextTick()` lebih detailnya Lo bisa kunjungi ini <a href="https://vuejs.org/api/general.html#nexttick" target="_blank" rel="noopener noreferrer">https://vuejs.org/api/scheduler.html#nextTick</a>

```js
async function increment() {
    console.log('increment');
    counter.value = {
        ...counter.value,
        count: counter.value.count + 1
    }
    await nextTick();
    console.log('Next tick render', counter);
}
```

### Reactive Keyword

Selain pake `ref` Lo juga bisa pake `reactive()` <a href="https://vuejs.org/api/reactivity-core.html#reactive" target="_blank" rel="noopener noreferrer">https://vuejs.org/api/reactivity-core.html#reactive</a>. Bedanya `reactive` itu dipake di kasus yang kompleks karena hasil dari `reactive` ini bukan object, melainkan Proxy dimana Proxy ini punya metadata. <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy" target="_blank" rel="noopener noreferrer">https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy</a>.

Coba Lo bikin halaman baru namanya `src/components/Reactive.vue`, `src/reactive.js`, `reactive.html` terus daftarin di `vite.config.js`:

```html
<!-- src/components/Reactive.vue -->
<script setup>
    import { reactive } from 'vue';

    const person = reactive({
        firstName: '',
        lastName: ''
    });

    function submit() {
        person.firstName = document.getElementById('firstName').value;
        person.lastName = document.getElementById('lastName').value;
    }
</script>

<template>
    <input type="text" id="firstName">
    <input type="text" id="lastName">
    <button v-on:click="submit">Submit</button>
    <p>First Name: {{ person.firstName }}</p>
    <p>Last Name: {{ person.lastName }}</p>
</template>
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/reactive.png" class="img-fluid" alt="reactive"/>

Tapi perlu Lo garis bawahi bro, saat Lo pake `reactive` ada keterbatasan nya dianding `ref`.
- Karena Proxy itu adalah objects type (object, array, collection) yang bisa dipake, jadi `reactive` ga punya primitive type kaya `string`, `number`, `boolean` dll.
- Ga isa di replace semua objectnya. Karena Proxy bakal berubah ke Object baru jadi Vue bakal kehilangan kendali atas track nya.
- Ga aman buat Destructuring Object, karena saat melakukan Destructuring Object secara otomatis hasil Destructuring tersebut keluar dari JavaScript Procy.

### Computed Properties

</details>

<details>
<summary><h2>Directives 📚</h2></summary>

Sebelumnya Lo udah nyoba 2 directive Vue yaitu `v-html` sama `v-bind` selain itu masih banyak lagi bro, tapi atribut directive selalu berawalan `v-`. Directive ini bisa punya argument atau engga, kalo misalnya punya argument maka Lo bisa pake `:` tapi kalo ya ga punya argument kaya `v-html` itu ga boleh pake `:`.

Selain itu Argument pada directive juga bisa menerima dynamic object atau data, misalnya Lo pingin isi atribut class `red`, `bold`, `uppercase` dll dalam satu directive bisa caranya pake kurung kotak `:class="['red', 'bold', 'uppercase']"` atau bisa menggunakan object `:class="{ red: red, bold: bold, uppercase: uppercase }"`.



</details>