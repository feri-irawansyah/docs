Woi brooo....

Lo harusnya dah tau kan soal `Tailwind CSS` yang katanya lagi viral di dunia per frontend nan. Apa yang bikin Tailwind CSS ini powerfull? Tapi ada yang ga suka juga katanya kotor, Ribet, Pake CSS aja bisa kaya gitu. Sebenernya tergantung pakenya aja. Statement gitu ada karena males belajar hal baru aja. Tapi ga salah juga, kadang legacy itu lebih baik daripada FOMO ikut - ikutan trend.

Catatan gue ini buat dokumentasi belajar gue sebelumnya, dan beberapa kali gue pake di project - project yang butuh styling compleks. Tapi disini gue ga jualan ya, ga di endors juga ini ikhlas murni dari programmer kampung.

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

Jadi ada element dengan class tertentu, kemudian kita buat style secara manual. Nah pendekatan ini disebut `Component-First` artinya ada suatu component yang kita ubah - ubah yaitu button. Contohnya seperti:

- Bootstrap
- Material UI
- Foundation
- Bulma
- Materialize
- Semantic UI
- Dll

Dan jika kita pake pendekatan `Utility-First` di tailwind element button nya seperti ini. Dibalik layar kita juga akan menggunakan class - class seperti `bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline`. Tapi ini nanti tailwind yang buat, kita hanya pake.

```html
<button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline">Button</button>
```

Contoh CSS Framework yang pake pendekatan `Utility-First` ga cuma Tailwind CSS doang:

- Tailwind CSS
- Techons
- Shed.css
- Basscss
- Expressive CSS
- Dll

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

Jadi kalo di bandingkan Bootstrap atau Bulma itu adalah `UI Framework` dimana tidak hanya Utility aja, tapi juga ada UI untuk component. Sedangkan Tailwind itu adalah `Utility Framework` dimana hanya Utility aja dan tidak ada UI untuk component.

### Kekurangan dan Kelebihan Tailwind CSS

Semua teknologi itu sama bro, selalu ada plus dan minusnya. Kalo misalnya kita bandingin dengan CSS, Bootstrap dan Tailwind itu.

<div class="table-responsive">
  <table class="table">
    <thead>
      <tr>
        <th scope="col">#</th>
        <th scope="col">Aspect</th>
        <th scope="col">Vanila CSS</th>
        <th scope="col">UI Framework (Bootstrap)</th>
        <th scope="col">Utility Framework (Tailwind)</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <th scope="row">1</th>
        <td>Dev Speed</td>
        <td>❌ Pembuatan Lebih lama</td>
        <td>✅ Pembuatan sangat cepat</td>
        <td>⚠️ Pembuatan sangat cepat (kalo dah jago)</td>
      </tr>
      <tr>
        <th scope="row">2</th>
        <td>Dev Experience</td>
        <td>❌ Buat dari 0</td>
        <td>✅ Ramah Pemula (Meskipun kurang jago CSS)</td>
        <td>⚠️ Harus ngerti CSS</td>
      </tr>
      <tr>
        <th scope="row">3</th>
        <td>Optimization</td>
        <td>⚠️ Paling Optimal (kalo dah ngerti)</td>
        <td>❌ Banyak CSS ga kepake</td>
        <td>✅ Code CSS Optimal</td>
      </tr>
      <tr>
        <th scope="row">4</th>
        <td>Customization</td>
        <td>✅ Kendali penuh</td>
        <td>❌ Tidak ada kendali</td>
        <td>✅ Kendali penuh</td>
      </tr>
      <tr>
        <th scope="row">5</th>
        <td>Hasil Website</td>
        <td>✅ Full Custom</td>
        <td>❌ Mirip - mirip kaya website lain</td>
        <td>✅ Custom sesuai kebutuhan</td>
      </tr>
      <tr>
        <th scope="row">6</th>
        <td>Semantic</td>
        <td>✅ HTML Rapi dan simpel</td>
        <td>⚠️ Sedikit lebih panjang</td>
        <td>❌ HTML panjang dan jelek</td>
      </tr>
    </tbody>
  </table>
</div>

### Documentation Tailwind CSS

<img class="img-fluid" alt="documentation-tailwindcss" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/tailwindcss/assets/documentation-tailwindcss.png">

<a href="https://tailwindcss.com/" target="_blank" rel="noopener noreferrer">https://tailwindcss.com/</a>

Di situ adalah taman bermain Tailwind CSS dimana cara pake dan semua utility nya bisa Lo pake. Di catatan ini gue pake Tailwind CSS versi 4.2. Kalo beberapa tahun kemudian Lo pake versi terbaru, bisa jadi ada perbedaan.

### Features Tailwind CSS

1. Interactivity
    - Interactivity disini adalah sebuat style interaksi seperti ketika di sorot
    - Focus atau mengubah suatu style dan ukuran.
    - Tailwind bisa melakukan itu menggunakan class `focus:`, `hover:`, `active:`, `group-hover:`, dsb.
    - Termasuk juga dengan Psudo-elemnt seperti `::before`, `::after`, `::selection`, dsb.
2. Responsive Design
    - Untuk responsive design ini, Tailwind menyediakan beberapa breakpoint seperti `sm`, `md`, `lg`, `xl`, `2xl`, dsb.
3. Dark Mode simple
   - Tailwind menyediakan class `dark:` untuk mode dark mode.
   - Ini keren menurut gue, karena langsung bisa pake class `dark:` untuk mode dark mode tanpa harus pake JS.
   - Selain itu Tailwind juga punya fitur `prefersdark` untuk mendeteksi mode dark mode dari tema operating system.
4. Reusability (Relate sama Frontend Framework)
    - Kalo integrasi sama Frontend Framework bisa nyediain component yang siap pake.
5. Customization Style
    - Bisa seperti CSS biasa tapi tetep pake konsep utility system.
6. Functions & Directives
    - Bisa pake function seperti `@apply`, `@screen`, `@media`, `@supports`, `@layer`, dsb.
    - Bisa pake directives seperti `@tailwind base`, `@tailwind components`, `@tailwind utilities`, dsb.
    - Semuanya bisa Lo pake atau bisa custom sendiri.

### Kenapa pake Tailwind CSS? 

</details>


<details open>
<summary><h2>Get Started Tailwind CSS 📚</h2></summary>

### Installation

Kalo Lo udah ke dokumentasinya tailwind, terus Klik tombol `Get Started` disitu Lo akan dibawa ke bagian Installation. Ada beberapa cara bisa pake Vite, Post CSS (ini CSS preprocessor dibalik layarnya si Tailwind), Tailwind CLI, Play CDN dan Framework Web. Paling mudah pake Play CDN krena tinggal pake Link aja. Tapi tidak direkomendasikan buat aplikaasi beneran. 

#### Play CDN 

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

<img class="img-fluid" alt="play-cdn" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/tailwindcss/assets/play-cdn.png">


</details>