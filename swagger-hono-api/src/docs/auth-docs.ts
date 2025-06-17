export const authDocs = {
  "/api/auth/login": {
    post: {
      tags: ["Authentication"],
      summary: "Login User",
      requestBody: {
        required: true,
        content: {
          "application/json": {
            schema: {
              type: "object",
              required: ["email", "password"],
              properties: {
                email: {
                  type: "string",
                  format: "email",
                },
                password: {
                  type: "string",
                },
              },
            },
          },
        },
      },
      responses: {
        200: {
          description: "Login Success",
          content: {
            "application/json": {
              example: { data: { message: "Login Successfully", token: "token123" } },
            },
          },
        },
        401: {
          description: "Unauthorized",
          content: {
            "application/json": {
              example: { error: { message: "Invalid credentials" } },
            },
          },
        },
        500: {
          description: "Internal Server Error",
          content: {
            "application/json": {
              example: { error: { message: "Internal Server Error" } },
            },
          },
        },
      },
    },
  },
  "/api/auth/session": { // <- path parameter pake kurung kurawal {id}
    get: {
      tags: ["Authentication"],
      summary: "Check User Session",
      description: "Check user session",
      security: [{ bearerAuth: [] }],
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
        400: {
          description: "Invalid request",
          content: {
            "application/json": {
              example: {
                error: { message: "File is required" },
              },
            },
          },
        },
      },
    },
  },
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
