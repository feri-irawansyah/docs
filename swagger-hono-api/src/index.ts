import { swaggerUI } from '@hono/swagger-ui'
import { Hono } from 'hono'

const app = new Hono()

app.get('/', (c) => {
  return c.text('Hello Hono!')
})

app.get('/openapi', (c) => {
    return c.json({
        openapi: '3.0.0',
        info: {
            title: 'Hono API Documentation',
            version: '1.0.0',
        },
    })
})

app.get('/docs', swaggerUI({ url: '/openapi' }))

export default app
