Dokumentasi API merupakan hal yang sangat penting bagi seorang backend, bukan hanya backend harusnya frontend juga mampu menggunakannya. Pada catatan gue kali ini gue mau membuat dokumentasi API menggunakan [Swagger](https://swagger.io/) pada [Hono](https://hono.dev/).

Agenda pada catatan ini:
- Membuat Aplikasi Hono
- Install Swagger
- Membuat Dokumentasi API
- 1. Json Request Body
- 2. Json Response Body
- 3. Query Parameter
- 4. Path Parameter
- 5. Header Parameter
- 6. File Upload
- 7. File Download

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

<hr/>

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

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/hono-swagger.png" alt="Hono Swagger" />

<hr/>

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

<hr/>

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

<hr/>

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

Untuk menambahkan response lainnya, kaya gini:

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
        400: { // status code 400
          description: "Bad Request", // deskripsi
          content: {
            "application/json": { // content type json
              example: { data: { message: "Invalid credentials" } }, // contoh response
            },
          },
        },
        500: { // status code 500
          description: "Internal Server Error", // deskripsi
          content: {
            "application/json": { // content type json
              example: { data: { message: "Internal Server Error" } }, // contoh response
            },
          }
        },
      },
    },
  }
};
```

Setelah menambahkan coba refresh browser, kaya gini:

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/response-400-500.png" alt="Hono Swagger" />

Untuk response `500` lebih baik tampilkan message Internal Server Error aja, untuk detail error lebih baik gunakan logger karena error sensitif kurang bagus jika ditampilkan ke user.

Untuk response lainnya bisa disesuaikan sesuai kebutuhan.

<hr/>

### Try It Out
Kita akan mencoba menjalankan swagger di browser. Namun sebelum itu, kita buat api login seperti berikut:

```js

// src/controllers/auth-controller.ts

import { Hono } from 'hono'

const authController = new Hono()

authController.post('/login', async (c) => {
  try {
    // Ambil data dari body
    const { email, password } = await c.req.json()

    // Simulasi error internal server
    if (email === 'error@example.com') {
      throw new Error('Internal server error')
    }

    // Simulasi validasi login
    if (email === 'admin@example.com' && password === 'password') {
      return c.json({
        data: {
          message: 'Login successful',
        }
      })
    } else {
      return c.json({
        error: {
          message: 'Invalid credentials',
        }
      }, 400)
    }

  } catch (error) {
    // Handling error 500
    return c.json({
      error: {
        message: (error as Error).message
      }
    }, 500)
  }
})

export default authController
```

```js
// src/index.ts

import authController from './controllers/auth-controller'

app.route('/api/auth', authController)
```

Setelah menambahkan coba refresh browser dan lakukan steps seperti ini:

1. Klik menu Try It Out
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/try-it-out.png" alt="Hono Swagger" />

2. Isikan email dan password (email: user@example.com, password: string) kita masukan email dan password salah dulu
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/try-it-out-1.png" alt="Hono Swagger" />

3. Execute
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/try-it-out-2.png" alt="Hono Swagger" />
Response 400 karena username dan password salah.
Coba ganti username dan password yang benar `email: admin@example.com` , `password: password`.
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/try-it-out-3.png" alt="Hono Swagger" />
Response 200 karena username dan password benar.
Coba ganti username dan password yang salah `email: error@example.com`,  `password: password`.
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/try-it-out-4.png" alt="Hono Swagger" />
Response 500 karena terjadi error internal server.

<hr/>

### Query Parameter
Biasanya ketika membuat api untuk mengambil data, kita menggunakan query parameter pada url untuk menentukan data apa yang ingin kita ambil. Misalnya `http://localhost:3000/api/order/data?limit=10&offset=0`. Untuk membuat query parameter pada swagger, kita akan membuat file baru yaitu `src/docs/order-docs.ts` dan `src/controllers/order-controller.ts` seperti berikut:

```bash
swagger-hono-api
├── src
│   ├── controllers 
│   │   └── auth-controller.ts
│   │   └── order-controller.ts // file baru
│   ├── docs 
│   │   └── auth-docs.ts 
│   │   └── order-docs.ts // file baru
│   ├── data
│   │   └── order.json // file baru untuk data
│   └── index.ts
├── bun.lock
├── README.md
├── .gitignore
├── package.json
└── tsconfig.json
```

`src/controllers/order-controller.ts`
```js
// src/controllers/order-controller.ts

import { Hono } from 'hono'
import { orders } from '../data/order.json'

const orderController = new Hono()

orderController.get('/data', (c) => {
  const limit = Number(c.req.query('limit') || 10) 
  const offset = Number(c.req.query('offset') || 0) 

  const paginatedData = orders.slice(offset, offset + limit)

  return c.json({
    data: paginatedData
  })
})

export default orderController
```

`src/docs/order-docs.ts`
```js
// src/docs/order-docs.ts

export const orderDocs = {
  "/api/order/data": {
    get: {
      tags: ["Order"],
      summary: "Get list of orders",
      description: "Retrieve list of orders with pagination support",
      parameters: [
        {
          name: "limit", // Tambahkan name: "limit" sesuai kebutuhan
          in: "query", // Tambahkan in: "query"
          required: true, // Tambahkan required: true agar field wajib diisi
          schema: {
            type: "integer", // Tambahkan type: "integer" agar field hanya bisa diisi dengan angka
            default: 5, // Tambahkan default: 10 agar field memiliki nilai default
          },
        },
        {
          name: "offset",
          in: "query",
          required: true,
          schema: {
            type: "integer",
          },
        },
      ],
      responses: {
        200: {
          description: "Successful response",
          content: {
            "application/json": {
              example: {
                data: [
                  { id: 1, name: "Order 1" },
                  { id: 2, name: "Order 2" },
                ],
              },
            },
          },
        },
        // Tambahkan response lainnya sesuai kebutuhan
      },
    },
  },
};
```

`src/data/order.json`
```json
// src/data/order.json

{
    "orders": [
        { "id": 1, "name": "Order 1" },
        { "id": 2, "name": "Order 2" },
        { "id": 3, "name": "Order 3" },
        { "id": 4, "name": "Order 4" },
        { "id": 5, "name": "Order 5" },
        { "id": 6, "name": "Order 6" },
        { "id": 7, "name": "Order 7" },
        { "id": 8, "name": "Order 8" },
        { "id": 9, "name": "Order 9" },
        { "id": 10, "name": "Order 10" },
        { "id": 11, "name": "Order 11" },
        { "id": 12, "name": "Order 12" },
        { "id": 13, "name": "Order 13" },
        { "id": 14, "name": "Order 14" },
        { "id": 15, "name": "Order 15" },
        { "id": 16, "name": "Order 16" },
        { "id": 17, "name": "Order 17" },
        { "id": 18, "name": "Order 18" },
        { "id": 19, "name": "Order 19" },
        { "id": 20, "name": "Order 20" }
    ]
}
```

`src/index.ts`
```js
// src/index.ts

import orderController from './controllers/order-controller'

app.route('/api/order', orderController)
```

`tsconfig.json`
```json
// tsconfig.json

{
  "compilerOptions": {
    "resolveJsonModule": true, // tambahkan ini untuk mengaktifkan resolveJsonModule
    "strict": true,
    "jsx": "react-jsx",
    "jsxImportSource": "hono/jsx"
  }
}
```

Jika setup sudah benar, kemudian refresh url http://localhost:3000/docs, maka akan muncul tab baru `Order` di swagger kaya gini:
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/parameter.png" alt="swagger-hono-api/assets/1.png" width="100%" />

<hr/>

### Path Parameter
Terkadang suatu endpoint membutuhkan parameter dinamis, misalnya id. Untuk itu, kita bisa menggunakan path parameter misal `http://localhost:3000/api/order/:id`. 

Untuk membuat path parameter pada swagger, kita akan menambahkan endpoint baru di file `src/controllers/order-controller.ts` seperti berikut:

```js
// src/controllers/order-controller.ts

orderController.get('/data/:id', (c) => {
  const idParam = c.req.param('id')
  const id = Number(idParam)

  // Validasi ID harus angka
  if (isNaN(id)) {
    return c.json({
      error: { message: 'Invalid ID parameter. Must be a number.' }
    }, 400)
  }

  const order = orders.find(o => o.id === id)

  if (!order) {
    return c.json({
      error: { message: 'Order not found' }
    }, 404)
  }

  return c.json({
    data: order
  })
})
```

Kemudian kita akan menambahkan endpoint baru di file `src/docs/order-docs.ts` seperti berikut:

```js
export const orderDocs = {
  // "/api/order/data": {
    // get: {........
  // },
  "/api/order/data/{id}": { // <- path parameter pake kurung kurawal {id}
    get: {
      tags: ["Order"],
      summary: "Get order by ID",
      description: "Retrieve a single order by its ID",
      parameters: [
        {
          name: "id",
          in: "path", // <- path param
          required: true,
          schema: {
            type: "integer",
          },
        },
      ],
      responses: {
        200: {
          description: "Successful response",
          content: {
            "application/json": {
              example: {
                data: { id: 1, name: "Order 1" },
              },
            },
          },
        },
        // Tambahkan response sesuai kebutuhan
      },
    },
  },
};
```

Kemudian refresh url http://localhost:3000/docs, maka akan muncul tab baru `Order` di swagger kaya gini:
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/path-parameter.png" alt="swagger-hono-api/assets/1.png" width="100%" />

<hr/>

### Header Parameter
Terkadang suatu endpoint membutuhkan header dinamis, misalnya token. Untuk itu, kita bisa menggunakan header parameter misal `http://localhost:3000/api/auth/session`. Kita akan mengimplementasikan untuk Authorization pake Bareer Token, meskipun masih banyak cara lain seperti pake Cookies, OAuth, dan lain-lain.

Untuk membuat header parameter pada swagger, kita akan menambahkan endpoint baru di file `src/controllers/auth-controller.ts` seperti berikut:

```js
// src/controllers/order-controller.ts

authController.get('/session', async (c) => {
  const authHeader = c.req.header('Authorization')
  
  if (!authHeader) {
    return c.json({
      error: { message: 'Authorization header is required' }
    }, 401)
  }

  const token = authHeader.replace('Bearer ', ''); // 🚀 ambil tokennya aja

  if (token === 'token123') {
    return c.json({
      data: {
        email: 'admin@example.com',
        role: 'admin',
        name: 'Satria Baja Ringan'
      }
    }, 200)
  } else {
    return c.json({
      error: {
        message: 'Invalid token',
      }
    }, 400)
  }
})
```

Kemudian kita akan menambahkan endpoint baru di file `src/docs/auth-docs.ts` seperti berikut:

```js
export const authDocs = {
  // "/api/auth/login": {
    // post: {........
  // },
  "/api/auth/session": { // <- path parameter pake kurung kurawal {id}
    get: {
      tags: ["Authentication"],
      summary: "Check User Session",
      description: "Check user session",
      security: [{ bearerAuth: [] }], // <- header param
      responses: {
        200: {
          description: "Successful response",
          content: {
            "application/json": {
              example: {
                data: {
                  email: 'admin@example.com',
                  role: 'admin',
                  name: 'Satria Baja Ringan'
                }
              },
            },
          },
        },
        // Tambahkan response sesuai kebutuhan
      },
    },
  },
};
```

Kemudian kita tambahkan header parameter di file `src/index.ts` seperti berikut:
```js
// src/index.ts

app.get('/openapi', (c) => 
  c.json({
    openapi: '3.0.0',
    info: {
      title: 'Hono API Documentation',
      version: '1.0.0',
    },
    components: { // <- tambahkan components
      securitySchemes: {
        bearerAuth: {               // <- nama bebas tapi harus sama dengan di docs
          type: "http",
          scheme: "bearer",
          bearerFormat: "JWT",     // cuma info, Swagger ignore ini
        },
      },
    },
    paths: {
      ...authDocs,
      ...orderDocs
    },
  })
)
```

Kemudian refresh url http://localhost:3000/docs, akan muncul tombol di pojok kanan swagger kaya gini:
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/header-parameter.png" alt="swagger-hono-api/assets/1.png" width="100%" />

Saat tombol di klik, nanti akan muncul pop up seperti berikut:
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/header-parameter-1.png" alt="swagger-hono-api/assets/1.png" width="100%" />

Kemudian isikan token response dari endpoint `/api/auth/login` kaya gini `token123`. Lalu klik execute pada endpoint `/api/auth/session`.

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/header-parameter-2.png" alt="swagger-hono-api/assets/1.png" width="100%" />

<hr/>

### File Upload
Terkadang suatu aplikasi juga membutuhkan fitur untuk upload file seperti gambar, csv, dll. Untuk membuat dokumentasi api file upload di swagger kita bisa menggunakan [File Upload Object](https://swagger.io/docs/specification/v2_0/file-upload/?sbsearch=upload) seperti berikut:

```js
export const authDocs = {
  // "/api/auth/login": {
  //   // post: {...
  // },
    "/api/auth/upload": {
      post: {
        tags: ["Authentication"],
        summary: "Upload a file",
        description: "Upload a file (image, CSV, etc.)",
        requestBody: {
          required: true,
          content: {
            "multipart/form-data": {
              schema: {
                type: "object",
                properties: {
                  file: { // 'file' harus sama dengan field di form/body
                    type: "string",
                    format: "binary", // <-- wajib untuk file upload
                  },
                },
                required: ["file"],
              },
            },
          },
        },
        responses: {
          200: {
            description: "Successful upload",
            content: {
              "application/json": {
                example: {
                  data: {
                    filename: "photo.png",
                    type: "image/png",
                    size: 123456,
                  },
                },
              },
            },
          },
          // Tambahkan response sesuai kebutuhan
      },
    },
  },
};
```

Kemudian buat endpoint baru di file `src/controllers/auth-controller.ts` seperti berikut:
```js
// src/controllers/auth-controller.ts
authController.post('/upload', async (c) => {
  const contentType = c.req.header('content-type')
  
  if (!contentType?.includes('multipart/form-data')) {
    return c.json({ error: { message: 'Content-Type must be multipart/form-data' } }, 400)
  }

  const body = await c.req.parseBody() as { [key: string]: File }
  const file = body['file'] // 'file' adalah nama field

  if (!file) {
    return c.json({ error: { message: 'File is required' } }, 400)
  }

  return c.json({
    data: {
      filename: file.name,
      type: file.type,
      size: file.size
    }
  })
})
```

Kemudian refresh url http://localhost:3000/docs, kemudian akan muncul endpoint `/api/auth/upload` seperti berikut:
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/file-upload.png" alt="swagger-hono-api/assets/1.png" width="100%" />
Saat choice file di klik, nanti akan muncul pop up windows yang mengarahkan kita untuk mengambil file yang ada di komputer kita. Kemudian klik open lalu Execute.

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/file-upload-1.png" alt="swagger-hono-api/assets/1.png" width="100%" />

Lu juga bisa buat custom - custom bang, dokumentasi api file upload di swagger, bisa lihat di [link ini](https://swagger.io/docs/specification/v2_0/file-upload/?sbsearch=upload). Seperti file yang di upload harus berupa gambar, csv, dll.

### File Download
Untuk membuat dokumentasi api file download di swagger kita bisa menggunakan [File Download Object](https://swagger.io/docs/open-source-tools/swagger-codegen/codegen-v2/prerequisites/?sbsearch=download) seperti berikut:

Pertama buat file baru di folder `public` kaya gini:
```bash
swagger-hono-api
├── public
│   └── hono-swagger.png // file baru
├── node_modules
├── src
│   ├── controllers 
│   │   └── auth-controller.ts
│   │   └── order-controller.ts // file baru
│   ├── docs 
│   │   └── auth-docs.ts 
│   │   └── order-docs.ts // file baru
│   ├── data
│   │   └── order.json // file baru untuk data
│   └── index.ts
├── bun.lock
├── README.md
├── .gitignore
├── package.json
└── tsconfig.json
```

Kemudian buat file baru di folder `docs` dengan nama `auth-docs.ts` seperti berikut:

```js
export const authDocs = {
  // "/api/auth/login": {
  //   // post: {...
  // },
  "/api/auth/download": {
    get: {
      tags: ["Authentication"],
      summary: "Download sample image",
      description: "Download a PNG image file",
      responses: {
        200: {
          description: "Image file",
          content: {
            "image/png": {
              example: "(binary image data)"
            }
          }
        }
      }
    }
  },
};
```

Kemudian buat endpoint baru di file `src/controllers/auth-controller.ts` seperti berikut:
```js
// src/controllers/auth-controller.ts

import { serveStatic } from 'hono/bun' // Plugin static file serving dari Hono Bun

authController.get('/download', serveStatic({ path: './public/hono-swagger.png' }))
```

Kemudian refresh url http://localhost:3000/docs, kemudian akan muncul endpoint `/api/auth/download` seperti berikut:
<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/file-download.png" alt="swagger-hono-api/assets/1.png" width="100%" />

Response akan berupa file `image/png` seperti gambar di atas lalu klik execute.

<img class="img-fluid" src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/swagger-hono-api/assets/file-download-1.png" alt="swagger-hono-api/assets/1.png" width="100%" />

Gambar akan tampil di swagger. Jika lu mau download filenya copy dan pastekan url `http://localhost:3000/api/auth/download` di browser, maka file akan otomatis ter download.

### Referensi
- [Swagger](https://swagger.io/)
- [Hono](https://hono.dev/)
- [Hono Swagger](https://hono.dev/examples/swagger-ui)

Mungkin segitu dulu catatan gue bang. Semoga bermanfaat. Nanti kalo misal gue ada temuan baru atau lu mau request bisa contact aja di halaman [Contact](https://snakesystem-library.vercel.app/#/contact).