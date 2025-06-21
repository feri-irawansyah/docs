import { ApolloClient, InMemoryCache, HttpLink, gql } from '@apollo/client/core';

const client = new ApolloClient({
  link: new HttpLink({ uri: 'http://localhost:8000/query' }),
  cache: new InMemoryCache(),
});

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

export default client;