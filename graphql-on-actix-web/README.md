Biasanya gue kalo bikin api pake Actix Web itu response JSON nya segelondongan langsung di kirim, nah gue mau nyobaik pake GraphQL buat ngatur field nya

Install `@apollo/client` dan `graphql` di frontend.

Code dasar:
```js
<script>
  import { onMount } from "svelte";

  let user = $state(null);
  let loading = $state(true);
  let error = $state(null);

  const fetchUser = async (id) => {
    try {
      const res = await fetch('http://localhost:8080/query', { 
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          query: `
            query GetUser {
              user(userId: ${id}) {
                userId
                email
                handphone
                registerDate
              }
            }
          `
        })
      });

      const data = await res.json();
      user = data.data.user; // ambil user pertama dari list
    } catch (err) {
      error = err;
    } finally {
      loading = false;
    }
  };

  onMount(() => {
    fetchUser(1);
  });

</script>

<div>
  {#if loading}
    <p>Loading...</p>
  {:else if error}
    <p>Error: {error}</p>
  {:else}
    <p>User ID: {user.userId}</p>
    <p>Email: {user.email}</p>
    <p>Handphone: {user.handphone}</p>
    <p>Register Date: {user.registerDate}</p>
  {/if}
</div>
```

### GraphQL Query

Method `GET`:
```js
query GetUser {
  user(userId: 1) {
    userId
    email
    handphone
    registerDate
  }
}
```

Method `POST`:
```js
mutation {
  createUser(input: {
    email: "abc@test.com"
    handphone: "08123456789"
    password: "rahasia"
  }) {
    userId
    email
  }
}
```

Method `PUT`:
```js
mutation {
  updateUser(id: 1, input: {
    email: "newemail@test.com"
    handphone: "08987654321"
  }) {
    userId
    email
  }
}
```

Method `DELETE`:
```js
mutation {
  deleteUser(id: 1) {
    userId
    email
  }
}
```