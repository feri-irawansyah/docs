<script>
  import { slide } from 'svelte/transition';

  let expandedRow = null;

  const toggleRow = (index) => {
    expandedRow = expandedRow === index ? null : index;
  };

  const users = [
    {
      userId: 1,
      email: "user1@example.com",
      fullName: "User 1",
      orders: [
        {
          orderId: 2,
          orderName: "Beli Martabak",
          orderPrice: 10000,
          orderStatus: "pending",
          orderDate: "2025-06-19T07:47:46.463842+00:00",
          lastUpdate: "2025-06-19T07:47:46.463850+00:00"
        }
      ]
    },
    {
      userId: 2,
      email: "user2@example.com",
      fullName: "User 2",
      orders: [
        {
          orderId: 3,
          orderName: "Beli Bakso",
          orderPrice: 10000,
          orderStatus: "pending",
          orderDate: "2025-06-19T07:47:58.439901+00:00",
          lastUpdate: "2025-06-19T07:47:58.439913+00:00"
        },
        {
          orderId: 4,
          orderName: "Beli Nasi Goreng",
          orderPrice: 15000,
          orderStatus: "pending",
          orderDate: "2025-06-19T07:48:08.741318+00:00",
          lastUpdate: "2025-06-19T07:48:08.741331+00:00"
        },
        {
          orderId: 5,
          orderName: "Beli Nasi Bakwan",
          orderPrice: 1000,
          orderStatus: "pending",
          orderDate: "2025-06-19T07:48:18.143382+00:00",
          lastUpdate: "2025-06-19T07:48:18.143390+00:00"
        }
      ]
    }
  ];
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
                        {#each users as user, i}
                        <tr>
                            <td>{user.userId}</td>
                            <td>{user.email}</td>
                            <td>{user.fullName}</td>
                            <td>
                            <button class="btn btn-sm btn-primary" on:click={() => toggleRow(i)}>
                                {expandedRow === i ? 'Collapse' : 'Expand'}
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