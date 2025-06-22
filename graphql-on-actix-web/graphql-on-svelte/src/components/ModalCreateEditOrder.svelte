<script>
    import { createOrder, updateOrder } from "$lib/order";
    import { onMount } from "svelte";

    const { data, icons="bi bi-cart-plus", color="primary", text="", modalId, users } = $props();

    let formData = $state({
        orderName: "",
        userId: 0,
        orderPrice: "",
        orderStatus: "pending"
    })

    $effect(() => {
        if(modalId.includes("edit")) {
            formData.orderName = data.orderName;
            formData.userId = data.userId;
            formData.orderPrice = data.orderPrice;
            formData.orderStatus = data.orderStatus;
        }
    })

    const submit = async (e) => {
        e.preventDefault();
        const price = parseInt(formData.orderPrice);

        if(modalId?.includes("edit")) {
            await updateOrder(data.orderId, formData.userId, formData.orderName, formData.orderStatus, price);
        } else {
            await createOrder(formData.orderName, formData.userId, price);
        }

    }
</script>

<button type="button" class="btn btn-sm btn-{color}" data-bs-toggle="modal" data-bs-target="#{modalId}">
    <i class={icons}></i> {text}
</button>

<div class="modal fade" id="{modalId}" tabindex="-1" aria-labelledby="{modalId}Label" aria-hidden="true">
    <div class="modal-dialog">
        <div class="modal-content">
            <div class="modal-header">
                <h5 class="modal-title" id="{modalId}Label">{text}</h5>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body">
                <form onsubmit={(e) => submit(e)}>
                    <div class="mb-3">
                        <label for="text" class="form-label">Order Name</label>
                        <input type="text" class="form-control" id="text" bind:value={formData.orderName} required>
                    </div>
                   <div class="mb-3">
                        <label for="userId" class="form-label">User</label>
                        <select required id="userId" class="form-control" bind:value={formData.userId}>
                            <option value="">Pilih User</option>
                            {#each users as user}
                                <option value={user.userId}>{user.fullName}</option>
                            {/each}
                        </select>
                   </div>
                   {#if modalId?.includes("edit")}
                    <div class="mb-3">
                        <label for="orderStatus" class="form-label">Status</label>
                        <select required id="orderStatus" class="form-control" bind:value={formData.orderStatus}>
                            <option value="">Pilih Order Status</option>
                            <option value="pending">PENDING</option>
                            <option value="settlement">SETTLEMENT</option>
                            <option value="cancelled">CANCELLED</option>
                        </select>
                    </div>
                   {/if}
                    <div class="mb-3">
                        <label for="orderPrice" class="form-label">Price</label>
                        <input type="text" class="form-control" id="orderPrice" bind:value={formData.orderPrice} required>
                    </div>
                    <div class="d-flex justify-content-end gap-3">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Ga Jadi</button>
                        <button type="submit" class="btn btn-primary">Gas</button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</div>