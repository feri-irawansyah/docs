import Swal from "sweetalert2";
import { createOrderMutation, getUsersWithOrders } from "./query";
import client from "$lib";

export const createOrder = (orderName, userId, orderPrice) => {

  return Swal.fire({
    icon: 'question',
    title: 'Kamu yakin?',
    text: `Kamu akan membuat order ${orderName} dengan harga ${orderPrice} untuk user ${userId}`,
    showCancelButton: true,
    confirmButtonText: 'Yoi, Tambah Aja!',
    cancelButtonText: 'Ga, Ga Jadi!',
    preConfirm: async () => {
      const query = await createOrderMutation(orderName, userId, orderPrice);
      console.log(query);
      const res = await client.mutate({
        mutation: query,
        refetchQueries: [{ query: getUsersWithOrders }]
      });

      if (res.data) {
        return Swal.fire({
          icon: 'success',
          title: 'Order berhasil dibuat',
          text: `Order ${res.data.createOrder.orderName} berhasil dibuat`,
          showConfirmButton: false,
          timer: 1500
        })
      } else {
        return Swal.fire({
          icon: 'error',
          title: `Order ${orderName} gagal dibuat`,
          text: `Error: ${res.errors[0].message}`,
          showConfirmButton: false,
          timer: 1500
        })
      }
    }
  })
}