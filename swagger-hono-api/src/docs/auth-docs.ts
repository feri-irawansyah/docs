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
      },
    },
  },
};
