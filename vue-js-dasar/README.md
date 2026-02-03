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

<details >
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

### Template Reference

Template ref ini memungkinkan Lo bisa akses ke DOM dari suatu element, jadi Lo kaya seolah - olah pake `document.getElementById`, bedanya sekarang Lo pake cara Vue dengan pake atrobut `ref` di tag HTML.

```html
<!-- src/components/HelloVue.vue -->
 <script setup>
import { useTemplateRef } from 'vue'

    const name = 'Satria Baja Ringan'
    const heading = '<h1>Hello Vue</h1>'
    const classHeading = 'heading'
    const className = 'name'
    const inputRef = useTemplateRef('inputRef')

    function focus() {
        console.log("focus");
        inputRef.value.focus()
    }
</script>

<template>
    <div v-html="heading" v-bind:class="classHeading"></div>
    <p :class="className">My name is {{ name }}</p>
    <button v-on:click="focus">Submit</button>
    <input type="text" ref="inputRef">
</template>
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/template-ref.png" class="img-fluid" alt="template-ref"/>

Untik directive `v-on:click` nanti gue bahas terpisah di materi tentang Directive.

</details>

<details open>
<summary><h2>Temtang Component 📚</h2></summary>

Sebelumnya Lo udah kenalan sama Component di Vue JS, sekarang gue mau bahas lebih spesifik tentang component.

### SFC (Single File Component)

Default component Vue JS itu adalah SFC atau Single File Component, jadi kalo Lo pake file `.vue` itu itu akan jadi SFC. Kemudian kalo Lo pingin bikin component baru, nanti Lo bikin file baru dengan extention `.vue` biar jadi SFC. Misalnya sekarang gue mau bikin component baru yaitu `src/components/Profile.vue` yang akan gue panggil di component HelloVue.

```html
<!-- src/components/Profile.vue -->
 <script setup>
    const name = 'Satria Baja Ringan'

</script>

<template>
    <p>My name is {{ name }}</p>
</template>
```

```html
<!-- src/components/HelloVue.vue -->
<script setup>
    import Profile from './Profile.vue';

    const heading = '<h1>Hello Vue</h1>'
</script>

<template>
    <div v-html="heading"></div>
    <Profile/>
</template>
```

Setiap component itu punya scopenya sendiri (isolated) baik itu data, function, state, bahkan style. Tapi untuk style ada perilaku sendiri nanti gue bahas lebih spesifik soal style. Lo juga bisa panggil component secara berulang.

```html
<!-- src/components/HelloVue.vue -->
<script setup>
    import Profile from './Profile.vue';

    const heading = '<h1>Hello Vue</h1>'
</script>

<template>
    <div v-html="heading"></div>
    <Profile/>
    <Profile/>
    <Profile/>
</template>
```

```html
<!-- src/components/Profile.vue -->
<script setup>
    const name = 'Satria Baja Ringan'

</script>

<template>
    <div>My name is {{ name }}</div>
</template>

<style scoped>
    div {
        color: green;
    }
</style>
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/sfc.png" class="img-fluid" alt="sfc"/>

Meskipun di child gue bikin warna hijau pada tag `div` tapi `HelloVue` tidak terpengaruh oleh stylenya, termasuk data juga, keduanya tidak bisa share data satu sama lain.

### Props

Ketika Lo render 3 kali component `Profile` Lo jadi punya text yang sama, nah gimana kalo misalya Lo pingin render 1 component secara berulang tapi datanya berbeda. Di Vue Lo bisa lakuin itu dengan `props`. 

#### Define Props

Ketika Lo pake props, nanti component Lo akan punya atribut kata tag HTML tapi dengan nama yang Lo atur sendiri. Lebih detailnya Lo bisa kunjungi ini bro <a href="https://vuejs.org/api/sfc-script-setup.html#defineprops-defineemits" target="_blank" rel="noopener noreferrer">https://vuejs.org/api/sfc-script-setup.html#defineprops-defineemits</a>.

```html
<!-- src/components/Profile.vue -->
 <script setup>
    const props = defineProps(["name"]);
</script>

<template>
    <div>My name is {{ props.name }}</div>
</template>

<style scoped>
    div {
        color: green;
    }
</style>
```

```html
<!-- src/components/HelloVue.vue -->
 <script setup>
    import Profile from './Profile.vue';

    const heading = '<h1>Hello Vue</h1>'
</script>

<template>
    <div v-html="heading"></div>
    <Profile name="Satria Baja Ringan"/>
    <Profile name="Tolak Misqueen"/>
    <Profile name="Uchiha Versi Beta"/>
</template>
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/props.png" class="img-fluid" alt="props"/>

#### Aturan Props

Tapi props ini juga punya aturannya bro.
- Saat Lo define Props, direkomendasikan pake format `camelCase` misal 2 atau lebih kata
- Namun saat menambahkan attribute pada Component, Lo pake `kebab-case`
- Saat pake attribute untuk mengubah value Props, Lo juga bisa pake Directive v-bind, sama seperti pada attribute biasanya di DOM element
- Props bersifat One-Way Data Flow, artinya hanya jalan 1 arah dari parent ke child, tidak bisa dibalik.
- Props bersifat readonly, artinya yang bisa ubah data hanya parent component saja.
- Atribut props ini bersifat optional, jadi kalo Lo ga perlu pake atau ngirim data itu gpp.
- Lo juga bisa ngasih nilai default ketika define propsnya

```js
const props = defineProps(["name", "totalCount"]);
```

#### Props Validation

Nah sebelumnya Lo kasih parameter pada `defineProps` adalah array, itu okeh tapi ga ada validasinya, biar Lo bisa lakuin validasi Lo bisa kasih parameter object dan key nya bisa Lo kasih tipe data. Lebih detailnya Lo bisa kunjungi ini bro <a href="https://vuejs.org/guide/components/props.html#prop-validation" target="_blank" rel="noopener noreferrer">https://vuejs.org/guide/components/props.html#prop-validation</a>.

```html
<!-- src/components/Profile.vue -->
 <script setup>
    const props = defineProps({
        name: String,
        totalCount: {
            type: Number,
            default: 0
        }
    })
</script>

<template>
    <div>My name is {{ props.name }}, count {{ props.totalCount }}</div>
</template>

<style scoped>
    div {
        color: green;
    }
</style>
```

```html
<!-- src/components/HelloVue.vue -->
 <script setup>
    import Profile from './Profile.vue';

    const heading = '<h1>Hello Vue</h1>'
</script>

<template>
    <div v-html="heading"></div>
    <Profile name="Satria Baja Ringan" total-count="100"/>
    <Profile name="Tolak Misqueen" />
    <Profile name="Uchiha Versi Beta" :total-count="300"/>
</template>
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/props-validate.png" class="img-fluid" alt="props-validate"/>

Nah dengan validate props Lo bisa lakuin validasi, contohnya ada warning 

```bash
heck failed for prop "totalCount". Expected Number with value 100, got String with value "100". 
  at <Profile name="Satria Baja Ringan" total-count="100" > 
```

Karena menggunakan string, agar menjadi number Lo perlu pake `v-bind` dengan `:total-count="100"` agar mendapatkan nilai sebenernya bukan plain text.

#### Component Event (Emit)

Selain mengirimkan data atau istilahnya Lo kaya bikin atribut HTML yang typenya itu readonly nah Lo juga bisa nambahin custom event handler di Component, caranya pake `defineEmits`. Implementasinya sama kaya props, Lo panggil `defineEmits` isi parameternya dengan array terus isi dengan nama handler yang Lo mau. Cara pakenya beda dikit karena Lo perlu pake `@nama-handler` untuk mengirim data.

```html
<!-- src/components/HelloVue.vue -->
<Profile name="Tolak Misqueen" @tambah-data="(e) => console.log(e)"/>
```

```html
<!-- src/components/Profile.vue -->
<script setup>
    const props = defineProps({
        name: String,
        totalCount: {
            type: Number,
            default: 0
        }
    })

    const emits = defineEmits(['tambahData']); // custom event
</script>

<template>
    <div>My name is {{ props.name }}, count {{ props.totalCount }}</div>
    <button v-on:click="emits('tambahData', 10)">Tambah Data</button>
</template>

<style scoped>
    div {
        color: green;
    }
</style>
```




### Lifecycle Hooks

Component itu punya alur hidupnya, jadi ketika di render suatu component juga bisa update dan bisa destroy juga sama kaya Lo bro, ada lahir tumbuh dan meninggoy. Nah flow tersebut di sebut Lifecycle Hooks di Vue JS. Lo bisa liat digram flow alur hidup suatu component di Vue JS disini bro <a href="https://vuejs.org/guide/essentials/lifecycle.html#lifecycle-diagram" target="_blank" rel="noopener noreferrer">https://vuejs.org/guide/essentials/lifecycle.html#lifecycle-diagram</a>.

Untuk lebih Vue API detailnya Lo bisa kunjungi ini <a href="https://vuejs.org/api/composition-api-lifecycle.html" target="_blank" rel="noopener noreferrer">https://vuejs.org/api/composition-api-lifecycle.html</a>.

- `onBeforeMount`: ketika component akan di render pertama kali
- `onMounted`: ketika component di render pertama kali
- `onUpdated`: Ketika component di render ulang
- `onUnmounted`: Ketika component di destroy

Mungkin gue akan jelasin ke 4 lifecycle hooks ini aja. Sisanya mungkin Lo bisa baca di documentasinya, karena ada beberapa yang intinya sama.

</details>

<details>
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

Saat merender suatu data di template, kurang di rekomendasikan bikin banyak Logic di HTML. Ya Lo bayangin aja misal banyak data Lo render tapi sekalian di bikin logic di HTML nya Lo mesti bakal susah maintain nya. Jadi lebih baik Lo taro di satu function atau method gitu. Contohnya gini

```html
<!-- src/components/Counter.vue -->
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

    function fullName() {
        console.log('fullName call');
        return `${person.firstName} ${person.lastName}`
    }
</script>

<template>
    <input type="text" id="firstName">
    <input type="text" id="lastName">
    <button v-on:click="submit">Submit</button>
    <p>Hello {{ fullName() }}</p>
</template>
```

Lancar ya keliatannya, tapi sebenernya ga juga bro. Ini aman karena Vue ga lakuin render ulang, dan kebetulan `reactive` di render ketika tombolnya di click baru function fullName di jalanin. Nah coba Lo bikin reactive state pake `ref` misalnya count.

```html
<!-- src/components/Reactive.vue -->
<script setup>
    import { reactive, ref } from 'vue';

    const person = reactive({
        firstName: '',
        lastName: ''
    });

    const count = ref(0);

    function submit() {
        person.firstName = document.getElementById('firstName').value;
        person.lastName = document.getElementById('lastName').value;
    }

    function increment() {
        console.log('increment');
        count.value++;
    }

    function fullName() {
        console.log('fullName call');
        return `${person.firstName} ${person.lastName}`
    }
</script>

<template>
    <input type="text" id="firstName">
    <input type="text" id="lastName">
    <button v-on:click="submit">Submit</button>
    <button v-on:click="increment">Increment {{ count }}</button>
    <p>Hello {{ fullName() }}</p>
</template>
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/computed-1.png" class="img-fluid" alt="computed-1"/>

Nah function jadi dipanggil berkali - kali kan. Nah untuk mengatasinya Vue punya keyword `computed` untuk membuat computed properties. <a href="https://vuejs.org/api/reactivity-core.html#computed" target="_blank" rel="noopener noreferrer">https://vuejs.org/api/reactivity-core.html#computed</a>. Dengan `computed` Vue bisa tau isi state yang dipake dalam `computed`, jadi ketika ada perubahan pada state yang ada didalam nya maka `computed` akan dipanggil ulang. Jadi kalo ada state diluarnya berubah, `computed` tidak akan dipanggil ulang.

```html
<!-- src/components/Reactive.vue -->
 <script setup>
    import { computed, reactive, ref } from 'vue';

    const person = reactive({
        firstName: '',
        lastName: ''
    });

    const count = ref(0);

    function submit() {
        person.firstName = document.getElementById('firstName').value;
        person.lastName = document.getElementById('lastName').value;
    }

    function increment() {
        console.log('increment');
        count.value++;
    }

    const fullName = computed(() => {
        console.log('fullName call');
        return `${person.firstName} ${person.lastName}`
    })
</script>

<template>
    <input type="text" id="firstName">
    <input type="text" id="lastName">
    <button v-on:click="submit">Submit</button>
    <button v-on:click="increment">Increment {{ count }}</button>
    <p>Hello {{ fullName }}</p>
</template>
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/computed-2.png" class="img-fluid" alt="computed-2"/>

Nah dengan begini aman, ketika count berubah fullName ga akan dipanggil ulang. Return dari `computed` ini sama kaya `ref` yaitu object dan ada `value` yang merupakan nilai dari statenya.

`Computed` ini punya parameter, dimana parameter ini berisi nilai sebelumnya. Misalnya sebelumnya `lastName` gue isi **baja Ringan** terus diganti ke **Baja Hitam**, maka `computed` akan menyimpan nilai sebelumnya `lastName` yaitu **Baja Ringan**.

```js
const fullName = computed((prev) => {
    console.log('fullName call', prev);
    return `${person.firstName} ${person.lastName}`
})
```

### Watcher

Vue memiliki fitur bernama watcher function, yang digunakan untuk reregistrasi callback function yang akan di trigger otomatis ketika sebuah state berubah. Di dalem watcher ini Lo bisa lakuin macam - macam kaya ubah DOM, Call API, dll ketika state berubah. Watcher ini sifatnya *lazy*, artinya callbacknya akan di trigger ketika state berubah.

#### Watch()

Watcher ini punya 3 parameter yaitu source, callback function dan opstions. Sourcenya ini berupa getter function, ref, reactive atau array yang berisi data tententu. Jadi watcher ini akan memantau source, ketika sourcenya ada perubahan, maka Lo bisa lakuin apa aja di callbacknya. Lebih lengkapnya Lo bisa liat di sini <a href="https://vuejs.org/api/reactivity-core.html#watch" target="_blank" rel="noopener noreferrer">https://vuejs.org/api/reactivity-core.html#watch</a>.

Contoh coba Lo buat 3 file json di folder `public`.
- `product-1.json`
- `product-2.json`
- `product-3.json`

```json
// product-1.json
{
    "id": 1,
    "name": "Product 1",
    "description": "Description for Product 1",
    "price": 19.99
}

// product-2.json
{
    "id": 2,
    "name": "Product 2",
    "description": "Description for Product 2",
    "price": 20.99
}

// product-3.json
{
    "id": 3,
    "name": "Product 3",
    "description": "Description for Product 3",
    "price": 14.99
}
```

Terus coba Lo bikin halaman baru kaya sebelumnya `src/components/Watcher.vue`, `src/watcher.js`, `watcher.html` terus daftarin di `vite.config.js`:

```html
<!-- src/components/Watcher.vue -->
<script setup>
    import { ref, watch } from 'vue';

    const productId = ref(0);
    const product = ref(null);

    watch(productId, async (newValue, oldValue) => {
        if(newValue) {
            const response = await fetch(`/${newValue}.json`);
            product.value = await response.json();
        } else {
            product.value = null;
        }
    });

</script>

<template>
    <select id="product" v-model="productId">
        <option value="product-1">Product 1</option>
        <option value="product-2">Product 2</option>
        <option value="product-3">Product 3</option>
    </select>

    <div v-if="product">
        <h1>{{ product.name }}</h1>
        <p>{{ product.description }}</p>
        <p>{{ product.price }}</p>
    </div>
</template>
```

Secara default, watch() function itu lazy, artinya menunggu source berubah dulu, baru callback function akan ditrigger. Dan akan di trigger ulang jika source berubah lagi tapi Lo bisa menambah options di watch() untuk mengubah behavior dari watch() function. Options dari watcher ini `immediate: true` jika ingin watch() langsung mengeksekusi callback saat pertama kali, ini cocok jika kita ingin load data awal langsung. Lo bisa nambahin options `once: true`, jika hanya ingin mentrigger callback function hanya sekali, sehingga ketika source berubah, tidak akan di trigger ulang.   

```js
watch(productId, async (newValue, oldValue) => {
    const response = await fetch(`/${newValue}.json`);
    product.value = await response.json();
}, { immediate: true });
```

#### WatchEffect()

Watcher pake `watch` mungkin bakal sering Lo pake, tapi sayangnya Lo harus pake `immediate: true` kalo mau langsung di jalanin tanpa harus lakuin trigger source nya. Nah kalo pake code sebelumnya misal.

```js
watch(productId, async (newValue, oldValue) => {
    if(newValue) {
        const response = await fetch(`/${newValue}.json`);
        product.value = await response.json();
    } else {
        product.value = null;
    }
}, { immediate: true });
```

Ini juga hanya akan menjalankan fetch didalamnya, tapi dengan newValue 0 karena belum memilih apapun. Jadi Lo harus isi producId nya pake default value agar langsung di jalankan. Tapi sebaiknya `watch` hanya digunanya untuk memantau aja. Vue punya Watcher lain yang berperilaku sama kaya `watch` + `immediate: true`, yaitu `watchEffect(callback)`. Lebih detailnya Lo bisa kunjungi ini <a href="https://vuejs.org/api/reactivity-core.html#watcheffect" target="_blank" rel="noopener noreferrer">https://vuejs.org/api/reactivity-core.html#watcheffect</a>.

```html
<!-- src/components/Watcher.vue -->
<script setup>
    import { ref, watchEffect } from 'vue';

    const productId = ref("product-1");
    const product = ref(null);

    watchEffect(async() => {
        const response = await fetch(`/${productId.value}.json`);
        product.value = await response.json();
    })
</script>

<template>
    <select id="product" v-model="productId">
        <option value="product-1">Product 1</option>
        <option value="product-2">Product 2</option>
        <option value="product-3">Product 3</option>
    </select>

    <div v-if="product">
        <h1>{{ product.name }}</h1>
        <p>{{ product.description }}</p>
        <p>{{ product.price }}</p>
    </div>
</template>
```

#### Cleanup Watcher

Nah di beberapa kasus mungkin kita perlu melakukan clean up sebelum state berubah ketiga ada suatu trigger. Vue nyediain function namanya `onWatcherCleanup()`. Nah tapi `onWatcherCleanup` tidak suport async, jadi kalo Lo mau pake Lo harus lakuin clean up nya terlebih dahulu sebelum await misalnya pada watcher Lo itu callbacknya adalah async. Lebih detailnya Lo bisa liat disini <a href="https://vuejs.org/api/reactivity-core.html#onwatchercleanup" target="_blank" rel="noopener noreferrer">https://vuejs.org/api/reactivity-core.html#onwatchercleanup</a>.

```html
<!-- src/components/Watcher.vue -->
<script setup>
    import { onWatcherCleanup, ref, watchEffect } from 'vue';

    const productId = ref("product-1");
    const product = ref(null);

    watchEffect(async() => {
        onWatcherCleanup(() => console.log('watcher cleanup'));
        const response = await fetch(`/${productId.value}.json`);
        product.value = await response.json();
    })
</script>
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/vue-js-dasar/assets/watcher.png" class="img-fluid" alt="watcher"/>

</details>

<details>
<summary><h2>Directives 📚</h2></summary>

Sebelumnya Lo udah nyoba 2 directive Vue yaitu `v-html` sama `v-bind` selain itu masih banyak lagi bro, tapi atribut directive selalu berawalan `v-`. Directive ini bisa punya argument atau engga, kalo misalnya punya argument maka Lo bisa pake `:` tapi kalo ya ga punya argument kaya `v-html` itu ga boleh pake `:`.

Selain itu Argument pada directive juga bisa menerima dynamic object atau data, misalnya Lo pingin isi atribut class `red`, `bold`, `uppercase` dll dalam satu directive bisa caranya pake kurung kotak `:class="['red', 'bold', 'uppercase']"` atau bisa menggunakan object `:class="{ red: red, bold: bold, uppercase: uppercase }"`.



</details>