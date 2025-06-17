import { Hono } from 'hono'
import { orders } from '../data/order.json'

const orderController = new Hono()

orderController.get('/data', (c) => {
  const limit = Number(c.req.query('limit') || 10) 
  const offset = Number(c.req.query('offset') || 0) 

  const paginatedData = orders.slice(offset, offset + limit)

  return c.json({
    data: paginatedData
  })
})

orderController.get('/data/:id', (c) => {
  const idParam = c.req.param('id')
  const id = Number(idParam)

  // Validasi ID harus angka
  if (isNaN(id)) {
    return c.json({
      error: { message: 'Invalid ID parameter. Must be a number.' }
    }, 400)
  }

  const order = orders.find(o => o.id === id)

  if (!order) {
    return c.json({
      error: { message: 'Order not found' }
    }, 404)
  }

  return c.json({
    data: order
  })
})

export default orderController
