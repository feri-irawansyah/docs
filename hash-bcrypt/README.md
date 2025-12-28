Bro Lo seorng backend engineer atau backend developer yang handal kan bro. Lo bisa bikin aplikasi dalam hitungan detik karena Lo pen cepet jadi yaudh password Lo user Lo simpen mentah - mentah di database Lo. Dan saat Lo masuk ke DB dan Lo jalanin `SELECT * FROM users` mantap, Lo bisa liat semua password user yang ada.

Lo tau ga pro Pemerintah terhormat kita di Nusantara ini telah menerbitkan suatu `Undang - Undang Nomor 27 tahun 2022` tentang **Perlindungan Data Pribadi (UU PDP)**. Apa aja yang di larang?:
- Seseorang dilarang secara melawan hukum mengumpulkan atau mendapatkan data pribadi orang lain tanpa izin. 
- Seseorang juga dilarang secara melawan hukum mengungkapkan / membocorkan data pribadi orang lain.
- Menggunakan data pribadi orang lain tanpa izin.

Dan password adalah data pribadi seseorang yang ga boleh diketahui orang lain. Kalo Lo nyimpen password mentah - mentah Lo bisa aja login dengan id dan password user tersebut. Dan amit - amit data LLo dicuri orang, orang lain bisa Login ke user manapun yang dia mau karena tau semua passwordnya. Dan Lo ga tau apa - apa soal kronologinya? Yang terjadi Lo yang bakal kena sanksi bro.

Untuk menghindari hal - hal yang ga Lo inginkan itu Lo perlu implementasi `Hash Password`. Ada banyak bro caranya Lo bisa bikin generate random dan kombinasi banyak karakter dan angka. Atau Lo bisa pake `Cryptograpics` seperti `MD5`, `SHA-1`, `SHA-256` atau Lo bisa pake alghoritma hashing khusus untuk password yang sudah di standarisasi dan di gunakan diseluruh dunia kaya `Bcrypt`, `Scrypt`, `Argon2`, dll. Semua tergantung kebutuhan Lo yang enting **PASSWORD USER TIDAK KELIHATAN**. 

Di catatan ini gue bakal bahas tentang hashing password khususnya untuk `Bcrypt` dan `Argon2`. Ouh iya nanti gue juga ada contoh implementasinya pake `Rust`, `.Net` dan `NodeJS`.

## Apa itu Hashing?

Hashing adalah proses mengubah sebuah data (misal teks, angka, file, dan lain - lain) menjadi suatu rangkaian karakter acak dengan panjang yang tetap menggunakan fungsi satu arah (one-way function).

Halo menurut mbah Wikipedia kaya gini
<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/hash-password/assets/hash-wiki.png" class="img-fluid" alt="hash Wiki"/>

Contoh:

```bash
Input: "password123"
Output: "e10adc3949ba59abbe56e057f20f883e"
```

