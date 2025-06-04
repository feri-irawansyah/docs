
# 🔑 Daftar Lengkap Keyword C# (.NET) + Contoh Implementasi

---

## 📦 1. Deklarasi Variabel & Tipe Data

| Keyword | Contoh |
|---------|--------|
| `int` | `int umur = 25;` |
| `float` | `float tinggi = 1.75f;` |
| `double` | `double berat = 65.5;` |
| `bool` | `bool aktif = true;` |
| `string` | `string nama = "Budi";` |
| `char` | `char huruf = 'A';` |
| `decimal` | `decimal saldo = 1000000.50m;` |
| `byte` | `byte b = 255;` |
| `long` | `long jarak = 10000000000;` |
| `short` | `short kecil = 32000;` |
| `var` | `var hasil = 10 + 20;` |
| `object` | `object o = "Hello";` |
| `dynamic` | `dynamic d = 123; d = "teks";` |

---

## 🔧 2. Fungsi & Metode

| Keyword | Contoh |
|---------|--------|
| `void` | `void Sapa() { Console.WriteLine("Hai"); }` |
| `return` | `int Tambah(int a, int b) { return a + b; }` |
| `static` | `static void Main(string[] args) { }` |
| `ref` | `void Tambah(ref int x) { x += 1; }` |
| `out` | `void Buat(out int x) { x = 10; }` |
| `in` | `void Cetak(in int x) { Console.WriteLine(x); }` |
| `params` | `void Total(params int[] angka) { }` |
| `async` | `async Task CekAsync() { await Task.Delay(1000); }` |
| `await` | `await Task.Delay(100);` |

---

## 🧱 3. Struktur Program

| Keyword | Contoh |
|---------|--------|
| `class` | `class Mobil { }` |
| `struct` | `struct Titik { public int X, Y; }` |
| `record` | `record User(string Nama, int Umur);` |
| `interface` | `interface IPrint { void Cetak(); }` |
| `enum` | `enum Warna { Merah, Hijau, Biru }` |
| `namespace` | `namespace Aplikasi.Sistem { }` |
| `using` | `using System;` |

---

## 🔁 4. Kontrol Alur

| Keyword | Contoh |
|---------|--------|
| `if` | `if (x > 0) { }` |
| `else` | `else { }` |
| `switch` | `switch (angka) { case 1: break; }` |
| `case` | `case 2: Console.WriteLine("Dua"); break;` |
| `default` | `default: Console.WriteLine("Lainnya");` |
| `for` | `for (int i = 0; i < 5; i++) { }` |
| `foreach` | `foreach (var item in list) { }` |
| `while` | `while (x < 10) { x++; }` |
| `do` | `do { x++; } while (x < 10);` |
| `break` | `break;` |
| `continue` | `continue;` |
| `return` | `return;` |
| `yield` | `yield return x;` |
| `goto` | `goto label; label: Console.WriteLine("Lompat");` |

---

## 🔐 5. Access Modifiers

| Keyword | Contoh |
|---------|--------|
| `public` | `public int ID;` |
| `private` | `private string nama;` |
| `protected` | `protected void Cetak() { }` |
| `internal` | `internal class Data { }` |
| `protected internal` | `protected internal int X;` |
| `private protected` | `private protected void Tes() { }` |

---

## 🧬 6. Inheritance & OOP

| Keyword | Contoh |
|---------|--------|
| `abstract` | `abstract class Hewan { public abstract void Bunyi(); }` |
| `override` | `override void Bunyi() { Console.WriteLine("Meee"); }` |
| `virtual` | `virtual void Suara() { }` |
| `new` | `new void Suara() { }` |
| `base` | `base.Suara();` |
| `this` | `this.Nama = "Budi";` |
| `sealed` | `sealed class TidakBisaDiturunkan { }` |

---

## 💣 7. Error Handling

| Keyword | Contoh |
|---------|--------|
| `try` | `try { ... }` |
| `catch` | `catch (Exception ex) { }` |
| `finally` | `finally { }` |
| `throw` | `throw new Exception("Error!");` |

---

## ❓ 8. Logical & Boolean Ops

| Keyword | Contoh |
|---------|--------|
| `true` | `bool aktif = true;` |
| `false` | `bool sukses = false;` |
| `null` | `string nama = null;` |
| `is` | `if (obj is string) { }` |
| `as` | `string teks = obj as string;` |

---

## 🧠 9. Contextual Keywords

| Keyword | Contoh |
|---------|--------|
| `get/set` | `public string Nama { get; set; }` |
| `partial` | `partial class Pegawai { }` |
| `nameof` | `Console.WriteLine(nameof(Nama));` |
| `typeof` | `Console.WriteLine(typeof(int));` |
| `default` | `int x = default;` |
| `checked` | `checked { int y = x + z; }` |
| `unchecked` | `unchecked { int y = x + z; }` |

---

## 🔐 10. Unsafe & Spesifik Memori

| Keyword | Contoh |
|---------|--------|
| `unsafe` | `unsafe void OperasiPointer() { int* p; }` |
| `fixed` | `fixed (int* p = &a) { }` |
| `stackalloc` | `int* p = stackalloc int[10];` |
| `lock` | `lock(obj) { /* thread-safe */ }` |

---

🧠 **Note**: Gunakan keyword sesuai konteks dan versi .NET. Beberapa fitur seperti `record`, `async`, `await`, `nameof` muncul di versi C# lebih baru.

