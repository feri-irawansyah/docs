Rust merupakan bahasa pemrograman yang lagi populer (katanya, tapi di indonesia masih banyak yang belum tau). Rust ini tergolong ke bahasa *compiled* atau untuk menjalankannya harus di kompilasi dulu. Termasuk *low-level programming*, bisa buat backend, desktop, ai, embeded dll masih banyak.

Pada catatan gue kali ini, gue mau ngoprek ada type data apa aja dan seperti apa perilakunya.

## Rust Type Data
Type data di rust ini dikelompokan menjadi 2 berdasarkan dimana data disimpan, yaitu `stack` dan `heap`.

Type data di stack ini adalah data yang disimpan di memory stack, sedangkan data di heap ini adalah data yang disimpan di memory heap. Type data yang di stack ini disebut `primitive type`, sedangkan type data di heap ini disebut `reference type`. 

### Primitive Type

#### Scalar Type (Type Data Sederhana)
TYpe data scalar di rust ini terdiri dari `integer`, `float`, `char`, dan `boolean`.

##### Integer
<table border="1" align="center" width="100%">
    <tr>
        <td>Type</td>
        <td>Ukuran</td>
        <td>Signed</td>
    </tr>
    <tr>
        <td>i8</td>
        <td>8-bit</td>
        <td>✅</td>
    </tr>
    <tr>
        <td>u8</td>
        <td>8-bit</td>
        <td>❌</td>
    </tr>
    <tr>
        <td>i16</td>
        <td>16-bit</td>
        <td>✅</td>
    </tr>
    <tr>
        <td>u16</td>
        <td>16-bit</td>
        <td>❌</td>
    </tr>
    <tr>
        <td>i32</td>
        <td>32-bit</td>
        <td>✅</td>
    </tr>
    <tr>
        <td>u32</td>
        <td>32-bit</td>
        <td>❌</td>
    </tr>
    <tr>
        <td>i64</td>
        <td>64-bit</td>
        <td>✅</td>
    </tr>
    <tr>
        <td>u64</td>
        <td>64-bit</td>
        <td>❌</td>
    </tr>
    <tr>
        <td>i128</td>
        <td>128-bit</td>
        <td>✅</td>
    </tr>
    <tr>
        <td>u128</td>
        <td>128-bit</td>
        <td>❌</td>
    </tr>
    <tr>
        <td>isize</td>
        <td>depends on platform (32-bit / 64-bit)</td>
        <td>✅</td>
    </tr>
    <tr>
        <td>usize</td>
        <td>untuk indexing/size (pointer size)</td>
        <td>❌</td>
    </tr>
</table>

Unsigned type ini hanya bisa diisi dengan angka positif, sedangkan signed type bisa diisi dengan angka positif dan negatif.

Ex:
```rust
let x: i32 = 10; ✅// i32 = integer 32-bit

let y: i64 = -20; ✅// i64 = integer 64-bit

let z: u32 = -30; ❌ // ini tidak boleh, karena u32 hanya bisa diisi dengan angka positif

println!("{} {}", x, y);
```

Type number ini bisa di lakukan conversi dari ukuran integer kecil ke lebih besar atau sebaliknya. Namun dengan catatan, jika kita mengkonversi dari integer kecil ke integer besar, maka akan ada overflow. Jika kita mengkonversi dari integer besar ke integer kecil, maka akan ada underflow. Contoh:
```rust
let x: i8 = 10; // i8 = integer 8-bit
println!("{}", x);
let y: i16 = a as i16; // i16 = integer 16-bit
println!("{}", y);

let a: i32 = 10000000; // i16 = integer 16-bit
println!("{}", a);
let b: i8 = a as i8; ❌// i8 = integer 8-bit ini akan overflow
println!("{}", b);
```

##### Float

Type data float di rust ini terdiri dari `f32` dan `f64`. digunakan untuk menyimpan angka desimal
<table border="1" align="center" width="100%">
    <tr>
        <td>Type</td>
        <td>Ukuran</td>
        <td>Precision</td>
    </tr>
    <tr>
        <td>f32</td>
        <td>32-bit</td>
        <td>7-8 digits</td>
    </tr>
    <tr>
        <td>f64</td>
        <td>64-bit</td>
        <td>15-16 digits</td>
    </tr>
</table>

```rust
let x: f32 = 10.0; // f32 = float 32-bit
let y: f64 = 20.0; // f64 = float 64-bit

println!("{} {}", x, y);
```

##### Char

Type data char di rust ini digunakan untuk menyimpan karakter dan hanya boleh diisi oleh satu karakter.
```rust
let a: char = 'a'; // char = character
let b: char = 'A';
let c: char = '1';
let d: char = '!';
let e: char = '😎';

let f: char = 'AA'; ❌// ini tidak boleh

println!("{} {} {} {} {}", a, b, c, d, e);
```

##### Boolean

Type data boolean di rust ini digunakan untuk menyimpan nilai true atau false atau sering disebut type data kebenaran.
```rust
let a: bool = true; // bool = boolean
let b: bool = false;

let c: bool = a == b;

println!("{} {}", a, b);
```

### Compound Type
Compound Type adalah type data yang terdiri dari beberapa type data lainnya. Compound Type di rust ini terdiri dari `tuple` dan `array`.

#### Tuple

Tuple adalah type data yang terdiri dari beberapa type data lainnya. Tuple di rust ini terdiri dari beberapa type data lainnya.
```rust
let x: (i32, f64, char, bool) = (10, 20.0, 'a', true);

println!("{} {} {}", x.0, x.1, x.2);
```

Tuple ini bisa mutable. mutable artinya bisa diubah. Namun perlu diingat, type nya harus sesuai.
```rust
let mut x: (i32, f64, char, bool) = (10, 20.0, 'a', true);

println!("{} {} {}", x.0, x.1, x.2);

x.0 = 20;
x.1 = 30.0;
x.2 = 'b';
x.3 = false;

x.3 = 10 ❌// ini tidak boleh

println!("{} {} {}", x.0, x.1, x.2);
```

#### Array

Array adalah type data yang terdiri dari beberapa type data lainnya. Array di rust ini terdiri dari beberapa type data lainnya. Array di rust ini terdiri dari beberapa type data lainnya.
```rust
let x: [i32; 5] = [10, 20, 30, 40, 50];

println!("{} {} {} {} {}", x[0], x[1], x[2], x[3], x[4]);
```

Array ini bisa mutable. mutable artinya bisa diubah. Namun perlu diingat, type nya harus sesuai.
```rust
let mut x: [i32; 5] = [10, 20, 30, 40, 50];

println!("{} {} {} {} {}", x[0], x[1], x[2], x[3], x[4]);

x[0] = 20;
x[1] = 30;
x[2] = 40;
x[3] = 50;
x[4] = 60;

x[4] = false ❌// ini tidak boleh

println!("{} {} {} {} {}", x[0], x[1], x[2], x[3], x[4]);
```
Array juga bisa dibuat menjadi 2 dimensi.
```rust
let x: [[i32; 5]; 2] = [[10, 20, 30, 40, 50], [60, 70, 80, 90, 100]];

println!("{} {} {} {} {}", x[0][0], x[0][1], x[0][2], x[0][3], x[0][4]);
println!("{} {} {} {} {}", x[1][0], x[1][1], x[1][2], x[1][3], x[1][4]);
```

### Reference Type
Data dari reference type akan disimpan di Heap, type data diheap ini sangat berbeda perilakunya dengan type data di stack. Sebelumnya di stack ketika kita membuat variable yang di isi kemudian kita gunakan, datanya bisa digunakan di semua tempat, sedangkan di heap ketika kita menggunakan data maka data akan langsung hilang. Contoh:
```rust
let a = "hello world";
let b = a; // data punya a akan di ambil oleh b
println!("{}", a); ❌// ini akan error, karena a sudah hilang
println!("{}", b);
```

#### String dan &str

`String` dan `&str` adalah type data text di rust. `String` adalah type data yang terdiri dari beberapa karakter.

##### &str

Ukuran &str ini fixed, artinya ukurannya tidak bisa diubah. Jadi ketika kita mengubah isi dari variable yang typenya &str, maka yang terjadi adalah mengganti isinya, bukan merubah data aslinya. Jadi, misalnya kita menggunakan method yang ada di &str, maka akan membuat nilai baru. (namun perlu diingat, nilai baru ini typenya `String`, bukan `&str`). Contoh:
```rust
let a: &str = "hello world";
println!("{}", a);
let b: String = a.trim();
println!("{}", b);
```

##### String
String di Rust merupakan tipe data text UTF-8, dan bisa berkembang ukurannya. Namun Ketika kita buat dalam bentuk immutable variable, maka String tidak bisa berkembang, namun tetap disimpan di Heap. Contoh:
```rust
let a: String = "hello world".to_string();
println!("{}", a);
let b: String = a.trim().to_string();
println!("{}", b);
```