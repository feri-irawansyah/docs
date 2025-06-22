<script>
    import { createOrder } from "$lib/order";

    const { data, icons="bi bi-cart-plus", color="primary", text="", modalId } = $props();

    let formData = $state({
        orderName: "",
        userId: 0,
        orderPrice: ""
    })

    const submit = async (e) => {
        const price = parseInt(formData.orderPrice);
        e.preventDefault();
        await createOrder(formData.orderName, formData.userId, price);
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
                            {#each data as user}
                                <option value={user.userId}>{user.fullName}</option>
                            {/each}
                        </select>
                   </div>
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