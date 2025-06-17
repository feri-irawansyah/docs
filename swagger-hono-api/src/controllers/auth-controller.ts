import { Hono } from 'hono'
import { serveStatic } from 'hono/bun' // Plugin static file serving dari Hono Bun

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
          token: 'token123'
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

authController.get('/download', serveStatic({ path: './public/hono-swagger.png' }))

export default authController