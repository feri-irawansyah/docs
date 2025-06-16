export const authDocs = {
  '/login': {
    post: {
      tags: ['Authentication'],
      summary: 'Login User',
      responses: {
        200: {
          description: 'Login Success',
        },
      },
    },
  },
}
