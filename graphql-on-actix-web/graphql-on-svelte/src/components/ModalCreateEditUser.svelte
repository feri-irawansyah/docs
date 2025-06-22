<script>
    import { createUser, updateUser } from "$lib/user";
    import { onMount } from "svelte";

    const { data, icons="bi bi-cart-plus", color="primary", text="", modalId="" } = $props();

    let formData = $state({
        email: "",
        fullName: "",
    })

    onMount(() => {
        if(modalId?.includes("edit")) {
            formData.email = data.email;
            formData.fullName = data.fullName;
        }
    })

    const submit = async (e) => {
        e.preventDefault();

        if(modalId?.includes("edit")) {
            await updateUser(data.userId, formData.email, formData.fullName);
        } else {
            await createUser(formData.email, formData.fullName);
        }

    }

    function formatTitle(str) {
        return str
            .split('-')
            .filter(word => isNaN(word)) // Hapus angka (misal "3")
            .map(word => word.charAt(0).toUpperCase() + word.slice(1))
            .join(' ');
    }
</script>

<button type="button" class="btn btn-sm btn-{color}" data-bs-toggle="modal" data-bs-target="#{modalId}">
    <i class={icons}></i> {text}
</button>

<div class="modal fade" id="{modalId}" tabindex="-1" aria-labelledby="{modalId}Label" aria-hidden="true">
    <div class="modal-dialog">
        <div class="modal-content">
            <div class="modal-header">
                <h5 class="modal-title" id="{modalId}Label">{formatTitle(modalId)}</h5>
                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
            </div>
            <div class="modal-body">
                <form onsubmit={(e) => submit(e)}>
                    <div class="mb-3">
                        <label for="email" class="form-label">Email</label>
                        <input type="email" class="form-control" id="email" bind:value={formData.email} required>
                    </div>
                    <div class="mb-3">
                        <label for="fullName" class="form-label">Full Name</label>
                        <input type="text" class="form-control" id="fullName" bind:value={formData.fullName} required>
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