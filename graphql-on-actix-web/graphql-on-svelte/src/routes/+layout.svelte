<script>
    import 'bootstrap/dist/css/bootstrap.min.css';
    import "bootstrap-icons/font/bootstrap-icons.css";
    import * as bootstrap from 'bootstrap';
    const { children } = $props();
</script>

<div class="container mt-5">
    <h3 class="text-center">GraphQL on Actix Web + Svelte</h3>
    {@render children()}
</div>