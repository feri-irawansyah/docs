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

export default orderController
