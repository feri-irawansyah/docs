<script>
  // import client, { getUsersWithOrders } from "$lib";
  import { onMount } from "svelte";
  import { slide } from "svelte/transition";

  let expandedRow = $state(null);
  let data = $state([]);

  const toggleRow = (index) => {
    expandedRow = expandedRow === index ? null : index;
  };

  // client.query({
  //   query: getUsersWithOrders,
  //   variables: { id: 1 },
  //   fetchPolicy: 'network-only'
  // }).then(res => res);
</script>

<div class="row gap-3">
  <div class="col-12">
    <div class="card">
      <div class="card-body">
        <button class="btn btn-primary">Create User</button>
        <button class="btn btn-success">Create Order</button>
      </div>
    </div>
  </div>
  <div class="col-12">
    <div class="card">
      <div class="card-body">
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
                    onclick={() => toggleRow(i)}
                  >
                    {expandedRow === i ? "Collapse" : "Expand"}
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
      </div>
    </div>
  </div>
</div>
