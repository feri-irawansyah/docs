Dokumentasi API merupakan hal yang sangat penting bagi seorang backend, bukan hanya backend harusnya frontend juga mampu menggunakannya. Pada catatan gue kali ini gue mau membuat dokumentasi API menggunakan [Swagger](https://swagger.io/) pada [Hono](https://hono.dev/).

Agenda pada catatan ini:
- Membuat Aplikasi Hono
- Install Swagger
- Membuat Dokumentasi API

## Membuat Aplikasi Hono
Pertama-tama, kita harus membuat aplikasi Hono. Sebelum membuatnya coba cek dulu apakan udah install [Bun](https://bun.sh/). Jika belum, install dulu dengan cara seperti berikut:

Windows
```bash
powershell -c "irm bun.sh/install.ps1 | iex"
```
Atau kalo di komputer lu udah ada npm atau node, install dulu dengan cara seperti berikut:
npm
```bash
npm install -g bun
```

Kalo udh di install check dulu pake terminal atau cmd, kaya gini:
```bash
bun --version
1.2.15 // ini versi bun yang gue install
```

Kalo udh di install, kita buat aplikasi Hono dengan cara seperti berikut:
```bash
bun create hono@latest swagger-hono-api
```

Nanti ada beberapa pilihan seperti template pake bun, package manager pilih bun, dan langsung install dependencies.

Setelah terbuat nanti harusnya dibuatkan otomatis struktur folder, kaya gini:
```bash
swagger-hono-api
├── src
│   └── index.ts // entry point
├── bun.lock
├── README.md
├── .gitignore
├── package.json
└── tsconfig.json
```

## Install Swagger
Kalo udah terbuat aplikasi Hono, kita install swagger dengan cara seperti berikut:
```bash
bun add @hono/swagger-ui
```

Setupnya seperti berikut:
```js
import { swaggerUI } from '@hono/swagger-ui'
import { Hono } from 'hono'

const app = new Hono()

app.get('/docs', swaggerUI({ url: '/openapi' })) // memerlikan url openapi

app.get('/openapi', (c) => {
    return c.json({
        openapi: '3.0.0',
        info: {
            title: 'Hono API Documentation',
            version: '1.0.0',
        },
    })
})
```

Setup dasarnya kira kira seperti itu bang. Setelah selsai coba jalankan hono app nya, kaya gini:
```bash
bun run dev

Started http server on http://localhost:3000
```

Kemudian buka url http://localhost:3000/docs di browser.

![Hono Swagger](https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/hono-swagger.png)

## Membuat Dokumentasi API
Sebelum membuat dokumentasi API, kita buat code project kita dulu biar lebih rapih, kaya gini:
```bash
swagger-hono-api
├── src
│   ├── controllers // disini kita untuk taro controller
│   │   └── auth-controller.ts // file controller untuk auth
│   ├── docs // disini kita untuk taro file dokumentasi
│   │   └── auth-docs.ts // file dokumentasi API untuk auth
│   └── index.ts // entry point
├── bun.lock
├── README.md
├── .gitignore
├── package.json
└── tsconfig.json
```

Kemudian tambahkan pada endpoint `/openapi` di file `index.ts` seperti berikut:
```js
import { authDocs } from './docs/auth-docs'

app.get('/openapi', (c) => 
  c.json({
    openapi: '3.0.0',
    info: {
      title: 'Hono API Documentation',
      version: '1.0.0',
    },
    paths: {
      ...authDocs, // import dan tambahkan authDocs
    },
  })
)
```

Untuk authDocs, kita buat file baru di folder `docs` dengan nama `auth-docs.ts` seperti berikut:
```js
export const authDocs = {
  '/login': {
    post: {
      tags: ['Authentication'],
      summary: 'Login User',
    },
  },
}
```

Setelah itu jalankan kembali hono app, namun secara default ketika menjalankan `bun run dev`, maka bun akan melakukan hot reload, jadi kita hanya perlu merefresh browser. Kaya gini:
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/hono-swagger-reload.png" alt="Hono Swagger" />

### JSON Request Body
Untuk API dengan method POST, kita bisa menggunakan JSON request body. Kaya gini:
```json
{
  "email": "satriabajaringan@gmail.com",
  "password": "amanbanget123"
}
```
Untuk membuat JSON request body pada swagger, kita bisa menggunakan `application/json` seperti berikut:
```js
export const authDocs = {
  '/login': {
    post: {
      tags: ['Authentication'],
      summary: 'Login User',
      requestBody: { // tambahkan requestBody
        // disini kita bisa menambahkan konfigurasi requestBody nya
      }
    },
  },
}
```

Kira kira seperti ini:

```js
export const authDocs = {
  '/api/auth/login': { // disini kita ubah /login menjadi /api/auth/login
    post: {
      tags: ['Authentication'],
      summary: 'Login User',
      requestBody: {
        required: true, // untuk menandakan bahwa field email dan password wajib diisi
        content: {
          'application/json': { // content type json
            schema: {
              type: 'object', // type object
              required: ['email', 'password'], // field email dan password wajib diisi
              properties: { // properties
                email: {
                  type: 'string', // type string
                  format: 'email', // format email
                },
                password: {
                  type: 'string', // type string
                },
              },
            },
          },
        },
      }
    },
  },
}
```

Setelah menambahkan coba refresh browser, kaya gini:

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/json-request-body.png" alt="Hono Swagger" />

### JSON Response Body
Ada request, maka ada response. Untuk response, kita bisa menggunakan JSON response body. Kaya gini:
```json
{
  "data": {
    "message": "Login successful"
  }
}
```
Untuk membuat JSON response body pada swagger, kita bisa menggunakan `application/json` seperti berikut:
```js
export const authDocs = {
  '/login': {
    post: {
      tags: ['Authentication'],
      summary: 'Login User',
      responses: { // tambahkan responses
        // disini kita bisa menambahkan konfigurasi responses nya
      }
    },
  },
}
```

Kira kira seperti ini:

```js
export const authDocs = {
  "/api/auth/login": {
    post: {
      tags: ["Authentication"],
      summary: "Login User",
      // requestBody: {
        // required: true
      // }
      responses: { // tambahkan responses 
        200: { // status code 200
          description: "Login Success", // deskripsi
          content: {
            "application/json": { // content type json
              example: { data: { message: "Login Successfully" } }, // contoh response
            },
          },
        },
      },
    },
  },
};
```

Setelah menambahkan coba refresh browser, kaya gini:

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/response-200.png" alt="Hono Swagger" />