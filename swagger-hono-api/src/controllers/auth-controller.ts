import { Hono } from 'hono'

const authController = new Hono()

authController.post('/login', async (c) => {
  // Ambil data dari body
  const { email, password } = await c.req.json()

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
    }, 401)
  }
})

export default authController
