import { gql } from "@apollo/client/core";
import Swal from "sweetalert2";

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

export const createUserMutation = (email, fullName) => {

  if (!email || !fullName) {
    return Swal.fire({
      icon: 'error',
      title: 'Error',
      text: 'Email dan Nama harus diisi',
    });
  }

  return gql`
    mutation {
        createUser(request: {
            email: "${email}",
            fullName: "${fullName}"
        }) {
            userId
            email
            fullName
        }
    }
`;
};

export const createOrderMutation = (orderName, userId, orderPrice) => {

  if (userId === 0 || !orderName || !orderPrice) {
    return Swal.fire({
      icon: 'error',
      title: 'Error',
      text: 'Email dan Nama harus diisi',
    });
  }

  return gql`
    mutation {
        createOrder(request: {
            orderName: "${orderName}",
            userId: ${userId},
            orderPrice: ${orderPrice},
        }) {
            orderId
            orderName
            userId
            orderPrice
            orderStatus
        }
    }
`;
};

export const updateUserMutation = (userId, email, fullName) => {

  if (!userId || !email || !fullName) {
    return Swal.fire({
      icon: 'error',
      title: 'Error',
      text: 'UserId, Email dan Nama harus diisi',
    });
  }

  return gql`
    mutation {
        updateUser(request: {
            userId: ${userId}
            email: "${email}"
            fullName: "${fullName}"
        }) {
            userId
            email
            fullName
        }
    }
`;
};