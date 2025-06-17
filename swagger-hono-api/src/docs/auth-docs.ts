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
              example: { data: { message: "Login Successfully" } },
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
};
