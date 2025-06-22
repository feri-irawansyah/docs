<script>
  import client from "$lib";
  import { getUsersWithOrders } from "$lib/query";
  import { onMount } from "svelte";
  import Table from "../components/Table.svelte";
  import ModalCreateEditUser from "../components/ModalCreateEditUser.svelte";
  import ModalCreateEditOrder from "../components/ModalCreateEditOrder.svelte";

  let expandedRow = $state(null);
  let data = $state([]);
  let error = $state(null);

  const toggleRow = (index) => {
    expandedRow = expandedRow === index ? null : index;
  };

  onMount(async () => {
    const res = await client.query({
      query: getUsersWithOrders,
    });
    data = await res.data.users;
    error = await res.errors? res.errors[0] : null;
  });

</script>

<div class="row gap-3">
  <div class="col-12">
    <div class="card">
      <div class="card-body">
        <ModalCreateEditUser data={null} text="Tambah User" modalId="tambah-user" />
        <ModalCreateEditOrder data={data} text="Tambah Order" color="success" modalId="tambah-order" />
      </div>
    </div>
  </div>
  <div class="col-12">
    <div class="card">
      <div class="card-body">
        {#if error}
          <div class="alert alert-danger" role="alert">
            {error.message}
          </div>
        {/if}
        <Table {data} {toggleRow} {expandedRow} />
      </div>
    </div>
  </div>
</div>
