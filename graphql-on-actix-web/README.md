Biasanya gue kalo bikin api pake Actix Web itu response JSON nya segelondongan langsung di kirim, nah gue mau nyobaik pake GraphQL buat ngatur field nya

Install `@apollo/client` dan `graphql` di frontend.

app/index.js:
```js
// src/lib/apolloClient.js
import { ApolloClient, InMemoryCache, HttpLink, gql } from '@apollo/client/core';

const client = new ApolloClient({
  link: new HttpLink({ uri: 'http://localhost:8080/query' }),
  cache: new InMemoryCache(),
});

export const USER = gql`
    query GetUser($id: Int!) {
      user(userId: $id) {
        userId
        email
        handphone
        registerDate
        disableLogin
      }
    }
  `;

export default client;
```

Code dasar:
```ts
<script>
  import client, { USER } from '../app';

  let userPromise = client.query({
    query: USER,
    variables: { id: 1 },
    fetchPolicy: 'network-only'
  }).then(res => res.data.user);

  function refetch() {
    userPromise = client.query({
      query: USER,
      variables: { id: 1 },
      fetchPolicy: 'network-only'
    }).then(res => res.data.user);
  }
</script>

<button on:click={refetch}>Refetch User (Fresh)</button>

{#await userPromise}
  <p>Loading...</p>
{:then user}
  <p>User ID: {user.userId}</p>
  <p>Email: {user.email}</p>
  <p>Handphone: {user.handphone}</p>
  <p>Register Date: {user.registerDate}</p>
  <p>Disable Login: {user.disableLogin ? 'Yes' : 'No'}</p>
{:catch error}
  <p>Error: {error.message}</p>
{/await}
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