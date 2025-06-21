import { gql } from "@apollo/client/core";

export const getUsersWithOrders = gql`
    query {
        users {
            userId
            email
            fullName,
            orders {
                orderId
                orderName,
                orderPrice,
                orderStatus,
                orderDate,
                lastUpdate
            }
        }
    }
`;