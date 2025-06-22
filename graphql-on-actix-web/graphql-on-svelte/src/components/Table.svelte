<script>
    import { slide } from "svelte/transition";
    import ModalCreateEditUser from "./ModalCreateEditUser.svelte";
    import { deleteUser } from "$lib/user";

    const { data=[], expandedRow, toggleRow } = $props();

</script>

<table class="table table-bordered">
  <thead>
    <tr>
      <th>#</th>
      <th>Email</th>
      <th>Full Name</th>
      <th>Action</th>
    </tr>
  </thead>
  <tbody>
    {#each data as user, i}
      <tr>
        <td>{user.userId}</td>
        <td>{user.email}</td>
        <td>{user.fullName}</td>
        <td>
          <button
            class="btn btn-sm btn-primary"
            aria-label="Toggle row"
            onclick={() => toggleRow(i)}
          >
            {#if expandedRow === i}
              <i class="bi bi-dash"></i>
            {:else}
              <i class="bi bi-plus"></i>
            {/if}
          </button>
          <ModalCreateEditUser data={user} icons="bi bi-pencil" color="success" modalId={`edit-user-${user.userId}`} />
          <button class="btn btn-sm btn-danger" aria-label="Delete" onclick={() => deleteUser(user)}>
            <i class="bi bi-trash"></i>
          </button>
        </td>
      </tr>
      {#if expandedRow === i}
        <tr>
          <td colspan="4">
            <div transition:slide>
              <table class="table table-sm table-striped">
                <thead>
                  <tr>
                    <th>Order ID</th>
                    <th>Name</th>
                    <th>Price</th>
                    <th>Status</th>
                    <th>Date</th>
                    <th>Action</th>
                  </tr>
                </thead>
                <tbody>
                  {#each user.orders as order}
                    <tr>
                      <td>{order.orderId}</td>
                      <td>{order.orderName}</td>
                      <td>{order.orderPrice}</td>
                      <td>{order.orderStatus}</td>
                      <td>{order.orderDate}</td>
                      <td>
                        <button
                          class="btn btn-sm btn-success"
                          aria-label="Edit"
                        >
                          <i class="bi bi-pencil"></i>
                        </button>
                        <button
                          class="btn btn-sm btn-danger"
                          aria-label="Delete"
                        >
                          <i class="bi bi-trash"></i>
                        </button>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          </td>
        </tr>
      {/if}
    {/each}
  </tbody>
</table>
