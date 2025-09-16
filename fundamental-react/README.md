Gue 100000% yakin, Lo mesti tau React kan? Lo mau bikin mobile app React bisa, web app React bisa, ssr React bisa, api bisa juga, seo bisa juga apa aja bisa asal Lo sama react. Dan yang paling bikin React terkenal itu komunitasnya gede banget, komunitas tech framework paling gede sealam semesta. Bayangin Lo nyari library khusus wibu aja ada. Tapi sebelum melangkah lebih jauh gue pingin kita bahas soal fundamentalnya dulu. Gue tau Lo udah pada jago, tapi gue mau bikin catetan ini buat mengingatkan hamba - hamba React kalo tersesat di jalan kemusyrikan😁.

Gue ada 12 Agenda nih. Banyak? Iya lah lu kira cuma tinggal minta chat GPT terus minta *`Buatkan saya component react dengan styling yang bagus, interaktive dan responsive`*. Udah kaya nyuruh kang nasgor buatin bakso aja.

<details>
<summary><h2>📌 Apa itu React?</h2></summary>

<h4>React itu Library atau Framework?</h4>

<img class="img-fluid" alt="image" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/fundamental-react/assets/react-docs.png" />

Kalo lu nyasar atau cari *`React`* di google search. Nah mesti bakal muncul web `https://react.dev`. Nah terus lu masuk di halaman pertama langsung muncul tulisan yang harusnya Lo bisa baca atau kalo Lo translate ke bahasa indonesia jadi 

<h3 class="text-danger text-underlined text-uppercase">Perpustakaan untuk antarmuka pengguna web dan asli</h3>

React itu library, dependencies, package, atau benda - benda semacam itu bukan `framework`. Framework itu kerangka kerja bro kaya Lo misal mau masak di resto itu udh ada tata caranya, alat - alat nya lengkap dan ada aturan nya harus steril, bersih, rapi, harus ikutin sop nya.

Beda kaya Lo masak di dapur orang jawa

<img class="img-fluid" alt="image" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/fundamental-react/assets/orang-jawa.jpg" />

Lo ngga perlu rapi, ngga perlu pake seragam, steril, ngga ada sop yang penting masakan halal dan ngga membunuh asal Lo jangan sampe bakar rumah aja.

Itulah framework suatu kerangka yang udah dibuatin orang atau organisasi biar Lo kaga sembarangan ngoplos source code Lo. React beda bro dia bukan framework, tapi dia library yang isinya itu fitur - fitur mempermudah hidup Lo, bukan mempersulit hidup Lo dan Lo terserah mau lakuin apa aja pake react. Misalnya gini bro, Lo mau masak nasgor di dapur orang jawa nih tanpa framework Lo nyalain api, panasin minyak, siapin bumbu, tumis, masukin nasi, aduk - aduk, angkat, lalu jadi. Nah fitur - fitur dan bahan nya itulah React bro.

Jadi kalo misal Lo ulek bumbu sendiri artinya Lo buat UI pake React Lo jahit sendiri, kalo Lo beli bumbu Racik, Sasa, Sajiku dkk Lo artinya pake third party library buatan orang kaya Ant Design, Chakra UI, MUI, Bootstrap dll.

<h4>Bedanya React dan Vanila JS</h4>

Javascript DOM Lo mesti pernah pake pas kuliah atau pas liat tutorial javascript. Nah sekarang mesti Lo nanya apa bedanya React sama Valina JavaScript? Gue bikin pake Javascript juga bisa kali?.

Analoginya gini Lo misal mau bikin patung, nah Lo di kasih tanah liat segelondongan Lo mesti harus punya skill dewa dulu tuh buat bikin idup,muka,kuping dll, iya kalo bagus? kalo lebih mirip kaya alien? kan kocak.

Bedain kalo Lo misal di kasih Lego nah Lo tinggal susun tuh kepingan - kepingannya.  Bahkan bukan cuma patungnya Lo bisa bikin istananya sekalian sama politiknya 😎. React itu kaya Lego jadi kalo lo mau buat patung Lo tinggal rakit kepingan/component nya, ngga perlu jadi Dewa Javascript dulu dan manual pake DOM. Gue kasih contoh

```html
<!-- contoh pake Vanila JS -->
<!DOCTYPE html>
<html>
  <body>
    <div id="app"></div>
    <script>
      let count = 0;

      const app = document.getElementById("app");
      const p = document.createElement("p");
      p.textContent = "Count: 0";

      const button = document.createElement("button");
      button.textContent = "Tambah";

      button.addEventListener("click", () => {
        count++;
        p.textContent = "Count: " + count;
      });

      app.appendChild(p);
      app.appendChild(button);
    </script>
  </body>
</html>
```

```html
<!-- contoh pake React -->
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>React Example</title>
    <!-- React dan ReactDOM dari CDN -->
    <script src="https://unpkg.com/react@18/umd/react.development.js"></script>
    <script src="https://unpkg.com/react-dom@18/umd/react-dom.development.js"></script>
  </head>
  <body>
    <div id="app"></div>

    <script>
      let count = 0;

      function render() {
        const element = React.createElement(
          "div",
          null,
          React.createElement("p", null, "Count: " + count),
          React.createElement(
            "button",
            {
              onClick: () => {
                count++;
                render();
              },
            },
            "Tambah"
          )
        );

        ReactDOM.createRoot(document.getElementById("app")).render(element);
      }

      render();
    </script>
  </body>
</html>
```

`React.createElement("element HTML", atribut, "Content"),`

Sekilas kaya lebih panjang React? iya lo ngga salah liat kok, karena masih pake `createElement` dan `render`. Tapi coba bayangin misal gue mau nambah element baru. Misal gue pake Vanila JS jadi gue harus `document.createElement('elemnt html')` terus gue isi content nya apa, atributnya apa dan tambahin `appendChild`. Bayangin misalnya gue bikin satu halaman web bakal berapa banyak gue melakukan proses mondar - mandir gitu?

Nah sekarang kalo pake React `React.createElement('elemnt html')` di dalemnya `React.createElement('parent')` otomatis React akan memasukan element baru beserta content dan atributnya didalam satu element `<div>` yang sama.

`ReactDOM.createRoot(document.getElementById("app")).render(element);` lalu ini apa? Nah ini cuma buat initialisasi aja bro. Jadi `ReactDOM` akan membuat root project lo di dalam `<div id="app"></div>` dan `render(element)` element HTML di dalamnya. `id="tidak harus app`, bebas apa aja asal ketika di panggil di `getElementById` itu harus related. Jadi semua aplikasi dan element HTML Lo akan masuk di dalam root project ini `<div id="app"></div>`.

Sedangkan `.render(disini wajib berisi elemnt html)` Lo bisa isi pake `React.createElement` atau pake JSX. 

Kebayang ya React mempermudah hidup Lo yang udah awal bulan tapi gajian belom cair. React juga menyediakan fitur yang lebih membantu hidup Lo lagi bro, yaitu JSX dan Component. Apa itu bro 🤔?


</details>

<details>
<summary><h2>📌 Dasar-Dasar React</h2></summary>

<h4>JSX (JavaScript XML)</h4>

Sebelumnya React udah mempermudah Lo bikin elemnt HTML di Javascript kan bro? Nah tapi kalo Lo nulis begitu dan code Lo banyak sampe ada banyak file itu bakal susah dibaca dan ketika Lo selsai ngoding, Lo mesti bakal jijik liatnya. 

Nah di React kita bisa pake JSX & Component untuk mempermudah hidup Lo bro.
1. JSX (JavaScript XML) itu Lo bisa nulis element HTML di Javascript (tanpa "", tanpa '' atau mantra - mantra lain. Lo tinggal tulis langsung element HTML nya). Contoh `<div></div> atau <div></div>`.
2. Component itu kaya kepingan - kepingan yang isinya itu JSX pake function atau class (udah jarang dipake sejak Recat V18) Javascript.

```jsx
const App = () => {
  return <div><h1>Hello World</h1></div>
};

// Atau pake ini sama aja
function App() {
  return <div><h1>Hello World</h1></div>
}
```

function `App` ini adalah Component bro dimana isinya itu HTML tanpa '', "", createElement, atau cari atribut, cari elemnt. React ga butuh itu bro tinggal tulis di Return nya. Dan kalo gue breakdown dari code sebelumnya jadi

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>React Example</title>
    <!-- React dan ReactDOM dari CDN -->
    <script src="https://unpkg.com/react@18/umd/react.development.js"></script>
    <script src="https://unpkg.com/react-dom@18/umd/react-dom.development.js"></script>
  </head>
  <body>
    <div id="app"></div>

    <script>
      function App() {
        return <div><h1>Hello World</h1></div>
      }

      ReactDOM.createRoot(document.getElementById("app")).render(<App />);

    //   <App /> JSX bikin function jadi element HTML
    </script>
  </body>
</html>
```

Simple kan? Okeh sekarang misal gue pingin pisahin `<h1>` di function terpisah, atau bikin element yang lebih banyak.

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>React Example</title>
    <!-- React dan ReactDOM dari CDN -->
    <script src="https://unpkg.com/react@18/umd/react.development.js"></script>
    <script src="https://unpkg.com/react-dom@18/umd/react-dom.development.js"></script>
  </head>
  <body>
    <div id="app"></div>

    <script>
      function App() {
        return (
          <div>
            <Header /> // element Header di panggil di function App
            <div>
                Content
            </div>
            <Footer /> // element Footer di panggil di function App
          </div>
        );
      }

      function Header() {
        return <h1>Hello World</h1>
      }

      function Footer() {
        return <p>Footer</p>
      }

      ReactDOM.createRoot(document.getElementById("app")).render(<App />);

    //   <App /> JSX bikin function jadi element HTML    
    </script>
  </body>
</html>
```
Lo bisa isi element HTML langsung di function dan bisa lo rangkai sendiri. Lebih mempermudah hidup Lo kan bro?. Udah ngga perlu createElemnt, getElement, bikin atribut di elemnt html, cari - cari elemnt, cari atribut, dan sebagainya. Jadi udah kebayang kan Lo ngoding tapi serasa lagi main Lego kaya pas waktu Lo masih kecil.

<h4>Aturan Component & JSX</h4>
Nah Component dan JSX di React juga punya aturan pake bro kaya obat dokter kalo Lo minum kebanyakan Lo bisa Overdosis dan Kalo kurang sakit Lo ga sembuh - sembuh.

1. Component harus di pake pake function.
Nah aturan ini hanya berlaku di React V18 keatas di versi sebelumnya Lo bisa pake class atau function dan return harus element HTML bisa `return <div></div>` atau `return (<div></div>)` biar lebih rapi. Dan lo ngga bisa mentah - mentah langsung nulis element HTML di file Javascript. atau di tag JavaScript.
```html
<script>
 < div><h1>Hello World</h1></div> // kaya gini ngga boleh
</script>
```

2. Component hanya boleh return/render 1 element HTML.
Tadi kok bisa return 2 sampe 5 elemnt HTML, tapi sekarang hanya bisa return 1 elemnt HTML? Beda bro, maksudnya return 1 element HTML itu `return <div><element atau jsx lain/></div>` yang ngga boleh itu:
```jsx
// Ga boleh return banyak element HTML
return (
    <h1>Hello World</h1>
    <h2>React</h2>
    <div>
    </div>
);
// Yang diperbolehkan
return (
    <> 
        <h1>Hello World</h1>
        <h2>React</h2>
        <div>
        </div>
    </>
);
```
Atau kalo Lo ga butuh tag HTML Lo bisa pake `<></>` atau `<>` untuk membungkus elemnt HTML.

3. Component ga boleh return kurung kurawal `{}` atau kurung kotak `[]`.
Kalo Lo render `{}` React bakal mengira kalo function yang Lo bikin itu return Object atau Array, bukan JSX atau Element HTML. Jadi ga boleh return `{}`.

4. Format functional Component harus `PascalCase` (ga boleh `camelCase`, `kebab-case`, `snake_case`).
```jsx
function PascalCase() {
  return <h1>Hello World</h1>
}

// ga boleh
function camelCase() {
  return <h1>Hello World</h1>
}
function kebab-case() {
  return <h1>Hello World</h1>
}
function snake_case() {
  return <h1>Hello World</h1>
}
```

5. Element HTML harus punya tutup (close tag).
```jsx
<img/> 
<br/>
<input/>

// ga boleh (wajib ada />)
<img>
<br>
<input>
```

6. Atribut HTML harus dengan format `camelCase`.
```jsx
// ga boleh pake onclick
<button onClick={() => console.log("Hello World")}>Submit</button>
```
Ada beberapa pengecualian untuk atribut `for` dan `class`. Di React ini di pake `htmlFor` dan `className` untuk atribut `for` dan `class` HTML. Kenapa? Karena React menggunakan file .js, .ts, .jsx, .tsx jadi `for` dan `class` akan dianggap keyword `for` untuk looping dan `class` untuk object class.
```jsx
<label htmlFor="submit">Submit</label>
<button className="submit">Submit</button>
```

7. Untuk menampilkan value harus menggunakan kurung kurawal `{}`.
```jsx
function PanggilNama() {

    const name = "Satria Baja Ringan";
  return <h1>Hello {name}</h1>
}
```

8. Jika terdapat kondisi maka harus menggunakan operator ternary.
```jsx
function PanggilNama() {

    const name = "Satria Baja Ringan";
  return name ? <h1>Hello {name}</h1> : <h1>Hello World</h1>
}
```

9. Gunakan `key` pada looping di HTML.
```jsx
function PanggilNama() {

    const name = ["Satria", "Baja", "Ringan"];
  return (
    <>
        {name.map((item, index) => {
            return <h1 key={index}>{item}</h1>
        })}
    </>
  )
}
```
10. Jika ingin menambahkan inline style di elemnt HTML, maka harus menggunakan `camelCase` untuk nama style nya dan di pake kurung kurawal `{}` berupa objek.
```jsx
function PanggilNama() {

    const name = ["Satria", "Baja", "Ringan"];
  return (
    <>
        {name.map((item, index) => {
            return <h1 key={index} style={{color: "red", fontSize: "20px"}}>{item}</h1>
        })}
    </>
  )
}
```

Okeh jadi itu aturan dalam Component dan JSX di React. Ibarat Lo mau mau kerja lewat Ordal (Orang Dalam) Lo harus baik ke orangnya, jaga nama baiknya, dan Lo siap ga enakan orangnya. Tapi masuknya mudah dibandingkan Lo lamar sendiri Lo harus Lulusan Kampus Terbaik, Portfolio Keren, Skill Dewa, dan Punya Psikolog normal dan itu belum tentu keterima kaya Lo pake Vanila JS yang ujung - ujungnya banyak error <span class="text-danger fw-bold">Undefined is not a function<span>

<h4>Aturan Component & JSX</h4>

</details>

<details>
<summary><h2>📌 React Hooks (Core)</h2></summary>

Lagi ditulis...

</details>

<details>
<summary><h2>📌 React Hooks (Core)</h2></summary>

Lagi ditulis...

</details>

<details>
<summary><h2>📌 Data Flow dan Thinking in React</h2></summary>

Lagi ditulis...

</details>

<details>
<summary><h2>📌 Styling di React</h2></summary>

Lagi ditulis...

</details>

<details>
<summary><h2>📌 Routing</h2></summary>

Lagi ditulis...

</details>

<details>
<summary><h2>📌 Data Fetching</h2></summary>

Lagi ditulis...

</details>

<details>
<summary><h2>📌 Form Handling & Validation</h2></summary>

Lagi ditulis...

</details>

<details>
<summary><h2>📌 State Management Lanjutan</h2></summary>

Lagi ditulis...

</details>

<details>
<summary><h2>📌 Ecosystem & Best Practices</h2></summary>

Lagi ditulis...

</details>

<details>
<summary><h2>📌 Unitest & Deployment</h2></summary>

Lagi ditulis...

</details>

