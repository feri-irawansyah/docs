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

export default authController