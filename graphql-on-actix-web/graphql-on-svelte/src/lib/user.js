import { gql } from "@apollo/client/core";
import Swal from "sweetalert2";

export const createUser = (email, fullName) => {

    return Swal.fire({
        icon: 'question',
        title: 'Kamu yakin?',
        text: `Kamu akan membuat user ${fullName} dengan email ${email}`,
        showCancelButton: true,
        confirmButtonText: 'Yoi, Tambah Aja!',
        cancelButtonText: 'Ga, Ga Jadi!',
        preConfirm: async () => {
            const res = await client.query({
              query: gql`
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
              `,
            });

            if(res.data) {
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