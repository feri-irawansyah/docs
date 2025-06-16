import { swaggerUI } from '@hono/swagger-ui'
import { Hono } from 'hono'
import { authDocs } from './docs/auth-docs'
import authController from './controllers/auth-controller'

const app = new Hono()

app.get('/', (c) => {
  return c.text('Hello Hono!')
})

app.route('/api/auth', authController)

app.get('/openapi', (c) => 
  c.json({
    openapi: '3.0.0',
    info: {
      title: 'Hono API Documentation',
      version: '1.0.0',
    },
    paths: {
      ...authDocs,
    },
  })
)

app.get('/docs', swaggerUI({ url: '/openapi' }))

export default app
