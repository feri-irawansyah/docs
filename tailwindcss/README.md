Woi brooo....

Jaman sekarang gini Lo masih mikirin selector - selector buat bikin tanda biar website Lo bisa Lo kasih make up. Terus karena Lo punya selector yang sama, jadinya rebutan dan biar itu ga kejadian Lo harus spesifik nyebutin sampe panjang banget kaya tenor pinjaman Lo. Terus Lo coba pake `pre processor` tapi Lo ga paham logic programming, endingnya Lo tetep nulis selector nested hell sampe kaya gunung ketinggan 99999999 mdpl.

```scss
.page {
  .header {
    .nav {
      ul {
        li {
          a {
            span {
              color: red;

              &:hover {
                color: blue;
              }
            }
          }
        }
      }
    }
  }
}
```

Disitulah muncul yang namanya CSS Framework biar mempermudah hidup Lo. Ya tapi balik lagi **sesuatu yang harusnya mempermudah bisa jadi bikin Lo susah**. Sebenernya CSS Framework itu hadir buat bantu dev - dev kaya Lo pada karena dari warna, layout, ukuran, psudo, responsive para engineer - engineer besar udah buatin dan Lo tinggal pake. Nah cuma masalahnya kalo Lo langsung pake tanpa tau dasarnya yaitu `CSS` (Casecading Style Sheet) bisa jadi Lo bakal bikin website yang kalo Lo buka malah tampilannya bikin sakit mata.

<details>
<summary><h2>Kenalan Sama Tailwind CSS 📚</h2></summary>

### Sejarah Tailwind CSS

Tailwind CSS pertama kali dirilis pada 1 November 2017 oleh Bapak <a href="https://adamwathan.me/" target="_blank" rel="noopener noreferrer">Adam Wathan</a> dan bersama timnya (termasuk Steve Schoger, Jonathan Reinink, dan David Hemphill). Framework ini diciptakan sebagai solusi atas keterbatasan framework CSS tradisional yang sering kali <u>terlalu kaku dengan komponen siap pakai</u>. Dengan menggunakan pendekatan `Utility-First`: berfokus pada kelas-kelas utilitas kecil.

Apa maksudnya, jadi Bapak Adam ini sengaja bikin tailwind itu langsung ke class terkecil. Nah kalo sebelumnya Lo pernah pake <a href="https://getbootstrap.com/" target="_blank" rel="noopener noreferrer">Bootstrap</a>, <a href="https://bulma.io/" target="_blank" rel="noopener noreferrer">Bulma</a>, <a href="https://materializecss.com/" target="_blank" rel="noopener noreferrer">Materialize</a> dsb. Biasanya class yang di sediain itu udah berbentuk, seperti `.btn btn-primary` dimana button nya itu udah ada bentuk, psudo, layout itu ada didalam klas tersebut.

Sedangkan kalo Tailwind ini beda, Tailwind menyediakan class - class yang lebih kecil lagi tidak ada bentuk component seperti CSS framework lainnya. Emang sengaja dibikin gitu biar dev itu memiliki kebebasan seperti pake CSS biasa, bedanya sekarang pake class dan bahkan sudah tidak perlu menyentuk native CSS yang pake pendekatan `Component-First`.

### Apa itu Utility-First & Component-First

Biasnya kalo Lo pake CSS biasa misalnya pingin bikin suatu tombol pake CSS biasa karusnya gini.

```html
<button type="button" class="btn btn-primary">Primary</button>
```

```css
.btn {
  display: inline-block;
  font-weight: 400;
  text-align: center;
  white-space: nowrap;
  vertical-align: middle;
  user-select: none;
  border: 1px solid transparent;
  padding: 0.375rem 0.75rem;
  font-size: 1rem;
  line-height: 1.5;
  border-radius: 0.25rem;
  transition: color 0.15s ease-in-out, background-color 0.15s ease-in-out, border-color 0.15s ease-in-out;
}
.btn-primary {
  color: #fff;
  background-color: #007bff;
  border-color: #007bff;
}
```

Jadi ada element dengan class tertentu, kemudian kita buat style secara manual. Nah pendekatan ini disebut `Component-First` artinya ada suatu component yang kita ubah - ubah yaitu button.

Dan jika kita pake pendekatan `Utility-First` di tailwind element button nya seperti ini. Dibalik layar kita juga akan menggunakan class - class seperti `bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline`. Tapi ini nanti tailwind yang buat, kita hanya pake.

```html
<button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">Button</button>
```

### Perbandingan Framework CSS

Nah buat contoh misalnya di bootstrap kan ada gini

```html
<button type="button" class="btn btn-primary">Primary</button>
```

Kalo di bulma ada gini

```html
<button class="button is-primary">Primary</button>
```

Di Tailwind kaya gini

```html
<button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">Button</button>
```

Panjang kan? Wkwkwkwkwk. Jadi kalo Lo pake Tailwind itu sama aja kaya pake CSS biasa cuma sekarang Lo pake class - class yang udah disediain Tailwind. Serasa jadi kotor ya HTML nya? Bener bro itu salah satu kekurangan Tailwind CSS.

Jadi, pendekatan Tailwind CSS itu seperti Lo baru beli alat - alat kuli seperti palu, pake, semen, pasir, kayu balok, dsb. dan Lo sebagai kulinya harus merangkai sendiri biar jadi sesuaitu yang Lo mau ini kenapa Lo perlu paham CSS terlebih dulu sebelum pake Tailwind CSS. Sedangkan untuk framework lain seperti Bootatrap, Bulma dkk. sudah ada furniture nya tinggak mau Lo pasang dibagian mana.

### Kekurangan dan Kelebihan Tailwind CSS

Semua teknologi itu sama bro, selalu ada plus dan minusnya.

#### Kekurangan

- HTML jadi panjang
- Susah dibaca buat pemula
- Duplicate style mesti selalu ada
- Harus belajar utility system dan tau class nya
- Kaya bikin Inline CSS
- Ga semantik keliatannya

#### Kelebihan

- Development super cepat, ga perlu bolak balik dari HTML ke CSS
- Konsistensi desain karena menggunakan utility system
- File CSS sangat kecil di production karena sudah di minify
- Sangat fleksibel Lo bisa bikin apa aja tanpa keterbatasan layout yang udah ditentukan seperti Bootstrap dan Bulma
- Integrasi frontend framework sangat bagus

### Documentation Tailwind CSS

<img class="img-fluid" alt="documentation-tailwindcss" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/tailwindcss-framework/assets/documentation-tailwindcss.png">

<a href="https://tailwindcss.com/" target="_blank" rel="noopener noreferrer">https://tailwindcss.com/</a>

Di situ adalah taman bermain Tailwind CSS dimana cara pake dan semua utility nya bisa Lo pake. Di catatan ini gue pake Tailwind CSS versi 4.2. Kalo beberapa tahun kemudian Lo pake versi terbaru, bisa jadi ada perbedaan.

### Features Tailwind CSS

- Interactivity

</details>


<details open>
<summary><h2>Get Started Tailwind CSS 📚</h2></summary>

Kalo Lo udah ke dokumentasinya tailwind, terus Klik tombol `Get Started` disitu Lo akan dibawa ke bagian Installation. Ada beberapa cara bisa pake Vite, Post CSS (ini CSS preprocessor dibalik layarnya si Tailwind), Tailwind CLI, Play CDN dan Framework Web. Paling mudah pake Play CDN krena tinggal pake Link aja. Tapi tidak direkomendasikan buat aplikaasi beneran. 

Nanti gue pake Play CDN tapi nanti bakal gue contohin pake Sveltekit. Coba sekarang Lo buat folder dan buka di VS Code. Lalu bikin file `index.html` dan `tailwind.config.js`. File `tailwind.config.js` kosongin aja.

```html
<!doctype html>
<html>
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script> <!-- pake CDN -->
  </head>
  <body>
    <h1>
      Hello world!
    </h1>
  </body>
</html>
```

<img class="img-fluid" alt="play-cdn" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/tailwindcss-framework/assets/play-cdn.png">


</details>