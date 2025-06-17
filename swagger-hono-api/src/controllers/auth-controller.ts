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