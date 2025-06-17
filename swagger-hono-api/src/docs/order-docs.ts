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
  "/api/order/data/{id}": { // <- path parameter pake kurung kurawal {id}
    get: {
      tags: ["Order"],
      summary: "Get order by ID",
      description: "Retrieve a single order by its ID",
      parameters: [
        {
          name: "id",
          in: "path", // <- path param
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
                data: { id: 1, name: "Order 1" },
              },
            },
          },
        },
        // Tambahkan response sesuai kebutuhan
      },
    },
  },
};
