Bro Lo seorng backend engineer atau backend developer yang handal kan bro. Lo bisa bikin aplikasi dalam hitungan detik karena Lo pen cepet jadi yaudh password Lo user Lo simpen mentah - mentah di database Lo. Dan saat Lo masuk ke DB dan Lo jalanin `SELECT * FROM users` mantap, Lo bisa liat semua password user yang ada.

Lo tau ga pro Pemerintah terhormat kita di Nusantara ini telah menerbitkan suatu `Undang - Undang Nomor 27 tahun 2022` tentang **Perlindungan Data Pribadi (UU PDP)**. Apa aja yang di larang?:
- Seseorang dilarang secara melawan hukum mengumpulkan atau mendapatkan data pribadi orang lain tanpa izin. 
- Seseorang juga dilarang secara melawan hukum mengungkapkan / membocorkan data pribadi orang lain.
- Menggunakan data pribadi orang lain tanpa izin.

Dan password adalah data pribadi seseorang yang ga boleh diketahui orang lain. Kalo Lo nyimpen password mentah - mentah Lo bisa aja login dengan id dan password user tersebut. Dan amit - amit data LLo dicuri orang, orang lain bisa Login ke user manapun yang dia mau karena tau semua passwordnya. Dan Lo ga tau apa - apa soal kronologinya? Yang terjadi Lo yang bakal kena sanksi bro.

Untuk menghindari hal - hal yang ga Lo inginkan itu Lo perlu implementasi `Hash Password`. Ada banyak bro caranya Lo bisa bikin generate random dan kombinasi banyak karakter dan angka. Atau Lo bisa pake `Cryptograpics` seperti `MD5`, `SHA-1`, `SHA-256` atau Lo bisa pake alghoritma hashing khusus untuk password yang sudah di standarisasi dan di gunakan diseluruh dunia kaya `Bcrypt`, `Scrypt`, `Argon2`, dll. Semua tergantung kebutuhan Lo yang enting **PASSWORD USER TIDAK KELIHATAN**. 

Di catatan ini gue bakal bahas tentang hashing password khususnya untuk `Bcrypt`. Ouh iya nanti gue juga ada contoh implementasinya pake `Rust`, `.Net` dan `NodeJS`. Kenapa di 3 bahasa ini ya karena ketiganya ini yang biasa gue pake dan familiar. Rust bahasa favorit, C# dipake di kantor dan JS ya mau ga mau harus bisa kan katanya Lo bisa javascript Lo adalah raja.

<details>
<summary><h2>Kenalan Sama Hash 📚</h2></summary>

### Apa itu Hashing

Hashing adalah proses mengubah sebuah data (misal teks, angka, file, dan lain - lain) menjadi suatu rangkaian karakter acak dengan panjang yang tetap menggunakan fungsi satu arah (one-way function). Disebut satu arah karena hasil hash tidak dapat dikembalikan ke data aslinya. Proses hashing hanya bisa dilakukan dari input ke output, bukan sebaliknya.

Kalo menurut mbah Wikipedia kaya gini:

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/hash-bcrypt/assets/hash-wiki.png" class="img-fluid" alt="hash Wiki"/>

Contoh:

```bash
Input: "password123"
Output: "e10adc3949ba59abbe56e057f20f883e"
```

Contoh di atas menggunakan algoritma SHA-256, yang selalu menghasilkan output dengan panjang tetap meskipun input berbeda-beda.

### Karakteristik Hashing

Hashing memiliki beberapa karakteristik penting:

- **One-way (Tidak bisa dibalik)**. Data asli tidak bisa didapatkan kembali dari hasil hash.
- **Deterministik**. Input yang sama akan selalu menghasilkan hash yang sama (pada algoritma hashing klasik).
- **Output panjang tetap**. Berapa pun panjang input, hasil hash selalu memiliki panjang yang sama.
- **Perubahan kecil** berdampak besar. Perubahan satu karakter saja pada input akan menghasilkan hash yang sangat berbeda.
- **Cepat untuk dihitung**. Hashing dirancang agar proses perhitungannya cepat.
- **Tidak terlalu sensitif terhadap karakteristik input**. Hashing tidak terlalu sensitif terhadap karakteristik input, seperti spasi, huruf besar / kecil, dan karakter khusus.

### Hash Biasa Kurang Aman Untuk Password

Kenapa hashing biasa seperti pake MD5, SHA-1, atau SHA-256 kurang aman buat password?. Algoritma hash umum seperti MD5, SHA-1, atau SHA-256 sebenarnya tidak dirancang khusus untuk menyimpan password. Tapi hanya untuk melakukan generate random karakter dari suatu text.

- **Terlalu cepat**. Penyerang bisa mencoba jutaan hingga miliaran kombinasi password per detik (brute force) sampe ketemu password yang cocok.
- **Rentan terhadap Rainbow Table**. Jika password umum digunakan, hash-nya bisa dicocokkan dengan tabel hash yang sudah tersedia.
- **Tanpa perlindungan tambahan**. Hash biasa tidak memiliki mekanisme bawaan untuk memperlambat serangan.
- **Hasil hash bisa sama**. Misal Lo punya pasword `123456` ketika dihash akan ada output random. Tapi ketika Lo masukin password yang sama hasilnya bisa sama.

</details>

<details>
<summary><h2>Kenalan Sama Bcrypt 📚</h2></summary>

### Sejarah Bcrypt

bcrypt pertama kali diperkenalkan pada tahun 1999 oleh Niels Provos dan David Mazières. Algoritma ini dipublikasikan bersamaan dengan sistem OpenBSD, sebuah sistem operasi yang terkenal dengan fokus tinggi pada keamanan. 

bcrypt dikembangkan sebagai respons terhadap kelemahan algoritma hash lama seperti DES-based crypt, MD5, dan SHA-1 yang terlalu cepat untuk kebutuhan penyimpanan password. Lebih lengkapnya Lo bisa kunjungi mbah Wikipedia <a href="https://id.wikipedia.org/wiki/Bcrypt" target="_blank" rel="noopener noreferrer">https://id.wikipedia.org/wiki/Bcrypt</a>.

Bcrypt adalah algoritma hashing yang dirancang khusus untuk menyimpan password dengan aman.
 
- Dirancang memamng sengaja lambat sulit untuk dicocokkan
- Pake `salt` secara otomatis
- Punya **cost factor** untuk mengatur lambatnya hash.
- Input sama tapi hasil hash selalu beda.

Contoh hash bcrypt:

```bash
Input: "password123"
Output: "$2a$10$8a6f6Nt2KqE8i7h4xG0z5a8b9c7d8f6g7h8i9j0k1l2m3n4o5p6q7"
```

Nah pas Lo input lagi sama hasilnya beda.

```bash
Input: "password123"
Output: "$2b$12$KIXIDu9LxYpY4ZCqK5f0uOKr6Qx8E0YcT8n9Vn3mZp8NqJ5OeW9p6"
```

### Apa itu Salt

Salt adalah nilai acak yang ditambahkan ke password sebelum proses hashing dilakukan. Salt inilah yang bikin hash password Lo selalu beda meskipun Lo masukin password yang sama. Secra konsep kaya gini misal.

```bash
hash(password + salt)
```

Fungsi utama salt:
- Mencegah dua password yang sama menghasilkan hash yang sama
- Melindungi dari serangan rainbow table
- Memperbesar kompleksitas brute force

Contoh tanpa salt:

```bash
password123 → hashA
password123 → hashA
```

Contoh dengan salt:

```bash
password123 + salt1 → hashA
password123 + salt2 → hashB
```

### Verifikasi Password

Lo mesti bertanya-tanya kalo hash selalu beda - beda gimana cara nya buat verifikasi?. Bcrypt punya fungsi verify untuk verifikasi password. Jadi bukan passwordnya Lo `Dcrypt` terus kelihatan passwordnya, tapi Lo minta brcypt untuk melakukan verifikasi. Bayangin misalnya Lo bikin sistem flownya gini:

- User register hash password Lo simpen di DB
- User login dan Lo tampilin password hashnya di response.
- Ada Attacker bisa dapet hash passwordnya.
- Kemudian dia Decrypt passwordnya otomatis dia tau password dari usernya.

Tapi dengan brcypt meskipun hash passwordnya itu bocor kata bcrypt:  **OOOO.... tidak semudah itu ferguso Lo ga bakal bisa drypt itu password hahahaha**. Karena brcypt akan membuat flow aplikasi Lo kaya gini:

- Pengguna memasukkan password saat login.
- Aplikasi Lo ambil hash password dari database.
- Password input di-hash menggunakan parameter yang sama.
- Brcypt bandingin hasil hash.
- Jika cocok → login berhasil
- Jika tidak cocok → login gagal

Bcrypt menyimpan semua informasi penting di dalam hash itu sendiri kaya versi algoritma, cost factor, salt. Kemudian pas verifikasi 
- bcrypt akan mengambil salt dan cost dari hash
- Password input akan di-hash ulang menggunakan parameter tersebut
- Hasilnya dibandingkan secara aman (constant-time comparison)

### Cost Factor

Cost factor (sering juga disebut work factor) adalah parameter pada bcrypt yang menentukan seberapa berat dan lama proses hashing dilakukan. Sederhananya semakin besar cost factor, semakin lama waktu yang dibutuhkan untuk menghasilkan hash. Cara kerjanya gini:

```bash
cost = n → hashing dilakukan sebanyak 2ⁿ kali
```
```bash
Cost 8 = 2^8 = 256

Cost 10 = 2^10 = 1024

Cost 12 = 2^12 = 4096
```

Meskipun semakin banyak cost factor semakin aman tapi Lo ga bisa sembarangan set cost factor. Karena kalo terlalu kecil jadi gambar di-brute force meski cepet, kalo kegedean server Lo bonyok tapi aman. Recomendasi yang bagus adalah melakukan benchmark di server Lo, karena tergantung dengan seberapa banyak request yang di tangani oleh server Lo juga.

</details>

<details>

<summary><h2>Implementasi Bcrypt (Rust) 📚</h2></summary>

Dari pada kebanyakan pembukaan langsung aja gas cara pakenya. Pertama gue mau coba implementasinya di bahasa pemrograman favorit gue. Yaitu Rust.

### Dependencies

Di Rust dependencies untuk bcrypt ada beberapa tapi yang paling stabil dan mature ada di sini <a href="https://crates.io/crates/bcrypt" target="_blank" rel="noopener noreferrer">https://crates.io/crates/bcrypt</a>.

Lo bisa download pake comand ini:

```bash
cargo add bcrypt
```

Versi bcrypt rust yang dipake di catatan ini adalah `0.17.1`. Lo boleh ikutin pake versi yang sama, karena beberapa tahun kedepan bisa aja berubah.

### Funtions Hashing

Ada beberapa function yang bisa Lo pake untuk lakuin hashing. Lebih lengkapnya Lo bisa kunjungi <a href="https://docs.rs/bcrypt/0.17.1/bcrypt/index.html#functions" target="_blank" rel="noopener noreferrer">https://docs.rs/bcrypt/0.17.1/bcrypt/index.html#functions</a>.

#### bcrypt::bcrypt (Low Level ⚠️)

Function ini punya 3 parameter yaitu `cost`, `salt` dan `password`. Ini adalah versi low levelnya jadi Lo harus nyediain cost dan saltnya sendiri. Kalo Lo buat dipake di aplikasi yang high security dan keamanan berada di control Lo sendiri ini function ini mungkin bisa jadi pilihan. Tapi kalo Lo paksain pake ini dan sebenernya Lo ga paham - paham amat wkwkwk. Mending jangan pake ini.

##### DEFAULT_COST

Rust bcrypt menyediakan default cost factor isinya adalah 12. Kalo server Lo kentang mungkin Lo bisa turunin.

```rust
// src/main.rs
use bcrypt::DEFAULT_COST;

fn main() {
    let hash = bcrypt::bcrypt(
        DEFAULT_COST, 
        *b"0123456789abcdef", 
        b"password"
    );
    println!("Hash {:?}", hash);
}
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/hash-bcrypt/assets/rust-bcrypt.png" class="img-fluid" alt="rust-bcrypt"/>

Outputnya juga dalam bentuk byte. Artinya Lo juga harus mengubahnya jadi string biar bisa Lo masukin ke DB pake base64. Tapi boleh aja kalo dimasukin karena database relational seperti PostgreSQL suport type byte.

#### bcrypt::hash (Recomended ✅)

Fungsi ini adalah fungsi yang paling recomended untuk digunakan, karena Rust bcrypt sudah membuatkan outputnya menjadi output dynamic. Artinya terserah Lo mau dibuat jadi apa outputnya karena pake enum ini:

```rust
pub enum BcryptResult<T> {
    Ok(T),
    Err(BcryptError),
}
```

Selain itu `BcryptResult` memiliki 2 type, yaitu `Ok` dan `Err`. `Ok` jika hashing berhasil dan `Err` jika ada error. Jadi Lo bisa lakuin validasi jika hash nya gagal atau error jangan lakuin action ke database kalo Lo mau simpan hash nya.

```rust
// src/main.rs
fn main() {
    let hash: Result<String, bcrypt::BcryptError> = bcrypt::hash(
        "password", 
        bcrypt::DEFAULT_COST
    );
    
    match hash {
        Ok(h) => println!("Hash: {}", h),
        Err(e) => println!("Error: {}", e)
    }
}
```

```bash
$ cargo run
   Compiling rust-brcypt v0.1.0 (C:\experiment\backend\bcrypt\rust-brcypt)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target\debug\rust-brcypt.exe`

Hash: $2b$12$AOY3JONzhpXK6OZHagO8LugdlU298L86drvW5ZioUj2yIMg5Qhx7m # Result Ok
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/hash-bcrypt/assets/rust-hash.png" class="img-fluid" alt="rust-hash"/>

Di gambar meskipun input sama yaitu `"password"` tapi outputnya beda. Karena hashingnya beda. Kalo input sama hasilnya beda.

Sekarang misalnya cost factornya di set 0 agar error.

```rust
// src/main.rs
fn main() {
    let hash: Result<String, bcrypt::BcryptError> = bcrypt::hash(
        "password", 
        0
    );
    
    match hash {
        Ok(h) => println!("Hash: {}", h),
        Err(e) => println!("Error: {}", e)
    }
}
```

```bash
$ cargo run
   Compiling rust-brcypt v0.1.0 (C:\experiment\backend\bcrypt\rust-brcypt)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
     Running `target\debug\rust-brcypt.exe`

Error: Cost needs to be between 4 and 31, got 0 # Result Err
```

#### bcrypt::hash_with_salt & bcrypt::hash_with_result

Bagian ini sebenernya hampir mirip kaya `bcrypt::hash` paling beda dikit buat `hash_with_salt` karena harus ada parameter salt. Tapi keduanya punya result type yang sama yaitu `HashParts`.

```rust
pub enum BcryptResult<HashParts > {
    Ok(HashParts),
    Err(BcryptError),
}
```

Sebenernya Lo bisa aja masukin `HashParts`. Disini Lo punya hanyak control resultnya. Lo bisa kasih format version hash nya, ambil salt dan cost factor nya.

##### Version

Pertama adalah version. Lo bisa set hash Lo pake version apa defaultnmya adalah pake `TwoB` karena ini Version terbaru yang dipake oleh OpenBSD paling aman. Pendahlunya ada `TwoA`, `TwoY` dan `TwoX`.

##### Get Salt & Cost Factor

Dengan result HashPart Lo bisa tau berapa cost dan salt yang di pake.

```rust
// src/main.rs
fn main() {
    let hash: Result<bcrypt::HashParts, bcrypt::BcryptError> = bcrypt::hash_with_result(
        "password", 
        12,
    );
    
    match hash {
        Ok(parts) => {
            let version_a = parts.format_for_version(bcrypt::Version::TwoA);
            let version_b = parts.format_for_version(bcrypt::Version::TwoB);
            let salt = parts.get_salt();
            let cost = parts.get_cost().to_string();

            println!("Version A: {}", version_a);
            println!("Version B: {}", version_b);
            println!("Salt: {}", salt);
            println!("Cost: {}", cost);
        },
        Err(e) => println!("Error: {}", e)
    }
}
```

```bash
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target\debug\rust-brcypt.exe`

Version A: $2a$12$.BVw70joWTssLh.phvuEXOp5jYWDONTQ0brJLXSXDXH2DLpUnMNxm # Version 2A
Version B: $2b$12$.BVw70joWTssLh.phvuEXOp5jYWDONTQ0brJLXSXDXH2DLpUnMNxm # Version 2B
Salt: .BVw70joWTssLh.phvuEXO # Salt
Cost: 12 # Cost
```

Tapi buat apa ya? Wkwkwkwk. Ini biasanya digunakan untuk audit security dan migration app. Okeh misal kasus gini di tahun 2025 ini Lo lulus kuliah terus join ke perusahaan yang punya aplikasi jadul dan pake bcrypt buat hash password usernya. Nah kalo Lo mau upgrade Lo perlu tau hash yang dipake senior - senior Lo ini apa sebelumnya apakah compatible sama aplikasi baru Lo. Disinilah function ini mungkin berguna karena kalo ada jutaan user gimana cara Lo update semua passwordnya.

#### Non Truncate Hash

Default karakter yang bisa di hash oleh Rust bcrypt adalah 72 bytes. Kalo lebih dari itu maka kepotong ya meski tidak mungkin ada orang masukin password panjannya lebih dari 72 bytes wkwkwk. Tapi ini paling man siapa tau aja ada orang yang lagi tidak sadarkan diri lakuin pendaftaran di aplikasi Lo.

Nama functionnya dna parameternya sama seperti sebelumnya bedanya depannya ada `no_truncating_`. 

```rust
// src/main.rs
fn main() {
    let hash = bcrypt::hash(
        "IniPasswordIsengBangetBuatNgetesBcryptLimit72ByteApakahKepotongAtauNggak1234567890!!!", 
        12,
    );

    let hash_no = bcrypt::non_truncating_hash(
        "IniPasswordIsengBangetBuatNgetesBcryptLimit72ByteApakahKepotongAtauNggak1234567890!!!", 
        12,
    );

    match hash {
        Ok(parts) => {
            println!("Hash: {}", parts);
        },
        Err(e) => println!("Error: {}", e)
    }
    
    match hash_no {
        Ok(parts) => {
            println!("Hash: {}", parts);
        },
        Err(e) => println!("Error: {}", e)
    }
}
```

```bash
$ cargo run
   Compiling rust-brcypt v0.1.0 (C:\experiment\backend\bcrypt\rust-brcypt)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target\debug\rust-brcypt.exe`
Hash: $2b$12$mJN0wSeWm63n3MTBTBAH8OHHAbPFYreV.SZNaIzFUF0aHwjtHCUT2 # Hash tapi kepotong
Error: Expected 72 bytes or fewer; found 86 bytes # Error
```

Kalo Lo pake method tau function `hash` maka resultnya akan terpotong. Kalo Lo pake function `non_truncating_hash` maka resultnya akan error karena melebihi 72 bytes.

### Verification

Biasanya melakukan hash password adalah pada saat registrasi dimana user pertama kali melakukan input yang kemudian Lo hash dan di simpen ke dalam database. Ketika user Login disinilah Lo akan melakukan verifikasi apakah password yang user masukin itu sama dengan password yang sudah di simpen di DB.

Disinilah bcrypt menyediakan function untuk verification. DI Rust bcrypt kalo Lo hash pake function bukan `no_truncating` maka Lo harus pake function `verify` untuk verification. Kalo Lo hash pake function `no_truncating` maka Lo bisa pake function `verify_no_truncating` untuk verification.

#### Verify

Verification dengan function `verify` ini punya 2 parameter yaitu password dan hash. Dengan type resultnya adalah :

```rust
pub enum Result {
    Ok(bool),
    Err(BcryptError),
}
```

Karena ada 2 kemungkinan yaitu `Ok` dan `Err`. Jadi Lo bisa lakuin validasi jika verification gagal atau error.

```rust
// src/main.rs
fn main() {
    let password = r"$2b$12$fWf29lMOg0xwx8tdERFL3e8TbOESlKVkZeUholVwyi8AQptFyqEH2";

    let check = bcrypt::verify("password", password);

    match check {
        Ok(is_valid) => println!("Check: {}", is_valid),
        Err(e) => println!("Check error: {}", e)
    }
}
```

```bash
$ cargo run
   Compiling rust-brcypt v0.1.0 (C:\experiment\backend\bcrypt\rust-brcypt)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target\debug\rust-brcypt.exe`
Check: true
```

Sekarang misalnya Lo ganti passwordnya jadi `password1`

```bash
$ cargo run
   Compiling rust-brcypt v0.1.0 (C:\experiment\backend\bcrypt\rust-brcypt)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.58s
     Running `target\debug\rust-brcypt.exe`
Check: false
```

#### Verify No Truncating

Verification dengan function `verify_no_truncating` ini sama aja punya 2 parameter yaitu password dan hash. Result typenya juga sama. Bedanya verification ini bisa dipake untuk hash password yang kurang dari 72 bytes. aja. Misalnya hash yang simpen ke DB itu < 72 bytes maka Lo masih bisa pake function `verify_no_truncating`. Tapi ketika password yang di simpen ke DB di buat lebih dari 72 bytes maka akan error.

Kalo Lo perhatiin ga ada proses melakukan decrypt atau me nampilkan password dari hashnya. Yang ada bikin hash kemudian verify inilah alasan bcrypt sangat recomended untu hash password.

</details>

<details open>

<summary><h2>Implementasi Bcrypt (.Net) 📚</h2></summary>

Selanjutnya adalah implementasi di bahasa pemrograman enterprice buatan microsoft yaitu `C#` atau `.Net Framework`. Kalo di dotnet package yang paling mature dan stabil adalah `BCrypt.Net-Next` <a href="https://www.nuget.org/packages/BCrypt.Net-Next" target="_blank" rel="noopener noreferrer">https://www.nuget.org/packages/BCrypt.Net-Next</a>.

Di dotnet ini Lo bisa lakuin kaya di Rust juga bisa langsung hash, atau hash dengan truncate. Bedanya di dotnet Lo ga dikasih fungsi low levelnya. Jadi ini lebih mudah dan aman. Lo juga ga ribet harus memahami bcrypt lebih mendalam.

### Funtions Hashing

Meskipun di dotnet tidak memiliki banyak fitur ke low levelnya tapi Lo dikasih control flow yang baik dari beberapa fungsi yang bakal Lo butuhin di kasus - kasus di real world.

#### Bcrypt.HashPassword

Ini adalah function hash password yang umum digunakan, mengharapkan 2 parameter

</details>