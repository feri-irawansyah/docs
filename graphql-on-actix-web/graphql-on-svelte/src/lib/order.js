import Swal from "sweetalert2";
import { createOrderMutation, deleteOrderMutation, getUsersWithOrders, updateOrderMutation } from "./query";
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

export const updateOrder = (orderId, userId, orderName, orderStatus, orderPrice) => {

  return Swal.fire({
    icon: 'question',
    title: 'Kamu yakin?',
    text: `Kamu akan mengupdate order ${orderName} dengan harga ${orderPrice} untuk user ${userId}`,
    showCancelButton: true,
    confirmButtonText: 'Yoi, Tambah Aja!',
    cancelButtonText: 'Ga, Ga Jadi!',
    preConfirm: async () => {
      const query = await updateOrderMutation(orderId, userId, orderName, orderStatus, orderPrice);
      const res = await client.mutate({
        mutation: query,
        refetchQueries: [{ query: getUsersWithOrders }]
      });

      if (res.data) {
        return Swal.fire({
          icon: 'success',
          title: 'Order berhasil diupdate',
          text: `Order ${res.data.updateOrder.orderName} berhasil diupdate',`,
          showConfirmButton: false,
          timer: 1500
        })
      } else {
        return Swal.fire({
          icon: 'error',
          title: `Order ${orderName} gagal diupdate',`,
          text: `Error: ${res.errors[0].message}`,
          showConfirmButton: false,
          timer: 1500
        })
      }
    }
  })
}

export const deleteOrder = (order) => {

  return Swal.fire({
    icon: 'question',
    title: 'Kamu yakin?',
    text: `Kamu akan menghapus order ${order.orderName} dengan harga ${order.orderPrice} untuk user ${order.userId}`,
    showCancelButton: true,
    confirmButtonText: 'Yoi, Hapus Aja!',
    cancelButtonText: 'Ga, Ga Jadi!',
    preConfirm: async () => {
      const query = deleteOrderMutation(order.orderId);
      const res = await client.mutate({
        mutation: query,
        refetchQueries: [{ query: getUsersWithOrders }]
      });

      if (res.errors) {
        return Swal.fire({
          icon: 'success',
          title: 'Order berhasil dihapus',
          text: `Order ${order.orderName} berhasil dihapus`,
          showConfirmButton: false,
          timer: 1500
        })
      } else {
        return Swal.fire({
          icon: 'error',
          title: `Order ${order.orderName} gagal dihapus`,
          text: `Error: ${res.errors[0].message}`,
          showConfirmButton: false,
          timer: 1500
        })
      }
    }
  })
}