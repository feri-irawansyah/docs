export const orderDocs = {
  "/api/order/data": {
    get: {
      tags: ["Order"],
      summary: "Get list of orders",
      description: "Retrieve list of orders with pagination support",
      parameters: [
        {
          name: "limit",
          in: "query",
          required: true,
          schema: {
            type: "integer",
          },
        },
        {
          name: "offset",
          in: "query",
          required: true,
          schema: {
            type: "integer",
          },
        },
      ],
      responses: {
        200: {
          description: "Successful response",
          content: {
            "application/json": {
              example: {
                data: [
                  { id: 1, name: "Order 1" },
                  { id: 2, name: "Order 2" },
                ],
              },
            },
          },
        },
      },
    },
  },
};
