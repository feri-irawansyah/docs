export const authDocs = {
  '/login': {
    post: {
      tags: ['Authentication'],
      summary: 'Login User',
      requestBody: {
        required: true,
        content: {
          'application/json': {
            schema: {
              type: 'object',
              required: ['email', 'password'],
              properties: {
                email: {
                  type: 'string',
                  format: 'email',
                },
                password: {
                  type: 'string',
                },
              },
            },
          },
        },
      }
    },
  },
}
