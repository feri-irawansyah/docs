<script>
    import 'bootstrap/dist/css/bootstrap.min.css';
    const { children } = $props();
</script>

<div class="container mt-5">
    <h3 class="text-center">GraphQL on Actix Web + Svelte</h3>
    {@render children()}
</div>