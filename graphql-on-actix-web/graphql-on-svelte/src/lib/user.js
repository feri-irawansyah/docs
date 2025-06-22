import Swal from "sweetalert2";
import { createUserMutation, deleteUserMutation, getUsersWithOrders, updateUserMutation } from "./query";
import client from "$lib";

export const createUser = (email, fullName) => {

  return Swal.fire({
    icon: 'question',
    title: 'Kamu yakin?',
    text: `Kamu akan membuat user ${fullName} dengan email ${email}`,
    showCancelButton: true,
    confirmButtonText: 'Yoi, Tambah Aja!',
    cancelButtonText: 'Ga, Ga Jadi!',
    preConfirm: async () => {
      const query = createUserMutation(email, fullName);
      const res = await client.mutate({
        mutation: query,
        refetchQueries: [{ query: getUsersWithOrders }]
      });

      if (res.data) {
        return Swal.fire({
          icon: 'success',
          title: 'User berhasil dibuat',
          text: `User ${res.data.createUser.fullName} berhasil dibuat`,
          showConfirmButton: false,
          timer: 1500
        })
      } else {
        return Swal.fire({
          icon: 'error',
          title: `User ${fullName} gagal dibuat`,
          text: `Error: ${res.errors[0].message}`,
          showConfirmButton: false,
          timer: 1500
        })
      }
    }
  })
}

export const updateUser = (userId, email, fullName) => {

  return Swal.fire({
    icon: 'question',
    title: 'Kamu yakin?',
    text: `Kamu akan mengupdate user ${fullName} dengan email ${email}`,
    showCancelButton: true,
    confirmButtonText: 'Yoi, Edit Aja!',
    cancelButtonText: 'Ga, Ga Jadi!',
    preConfirm: async () => {
      const query = updateUserMutation(userId, email, fullName);
      const res = await client.mutate({
        mutation: query,
        refetchQueries: [{ query: getUsersWithOrders }]
      });

      if (res.data) {
        return Swal.fire({
          icon: 'success',
          title: 'User berhasil diupdate',
          text: `User ${res.data.updateUser.fullName} berhasil diupdate`,
          showConfirmButton: false,
          timer: 1500
        })
      } else {
        return Swal.fire({
          icon: 'error',
          title: `User ${fullName} gagal diupdate`,
          text: `Error: ${res.errors[0].message}`,
          showConfirmButton: false,
          timer: 1500
        })
      }
    }
  })
}

export const deleteUser = (user) => {

  return Swal.fire({
    icon: 'question',
    title: 'Kamu yakin?',
    text: `Kamu akan menghapus user ${user.fullName} dengan email ${user.email}`,
    showCancelButton: true,
    confirmButtonText: 'Yoi, Hapus Aja!',
    cancelButtonText: 'Ga, Ga Jadi!',
    preConfirm: async () => {
      const query = deleteUserMutation(user.userId);
      const res = await client.mutate({
        mutation: query,
        refetchQueries: [{ query: getUsersWithOrders }]
      });

      if (res.errors) {
        return Swal.fire({
          icon: 'success',
          title: 'User berhasil dihapus',
          text: `User ${user.fullName} berhasil dihapus`,
          showConfirmButton: false,
          timer: 1500
        })
      } else {
        return Swal.fire({
          icon: 'error',
          title: `User ${user.fullName} gagal dihapus`,
          text: `Error: ${res.errors[0].message}`,
          showConfirmButton: false,
          timer: 1500
        })
      }
    }
  })
}