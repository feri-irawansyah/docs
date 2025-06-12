
# 📘 Cheat Sheet Tipe Data C# (.NET)

## 🧱 1. Value Types (Disimpan di Stack)
Tipe data yang menyimpan nilai langsung.

### 🔢 Numerik
| Tipe     | Ukuran  | Rentang Nilai |
|----------|---------|----------------|
| `byte`   | 8-bit   | 0 to 255 |
| `sbyte`  | 8-bit   | -128 to 127 |
| `short`  | 16-bit  | -32,768 to 32,767 |
| `ushort` | 16-bit  | 0 to 65,535 |
| `int`    | 32-bit  | -2,147,483,648 to 2,147,483,647 |
| `uint`   | 32-bit  | 0 to 4,294,967,295 |
| `long`   | 64-bit  | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 |
| `ulong`  | 64-bit  | 0 to 18,446,744,073,709,551,615 |
| `float`  | 32-bit  | ±1.5e−45 to ±3.4e38 |
| `double` | 64-bit  | ±5.0e−324 to ±1.7e308 |
| `decimal`| 128-bit | ±1.0e−28 to ±7.9e28 (presisi tinggi) |

### 🧠 Logika & Karakter
- `bool`: true / false
- `char`: Karakter Unicode tunggal (contoh: `'A'`)

### 📅 Tanggal dan Waktu
- `DateTime`: Representasi tanggal & waktu
- `TimeSpan`: Selisih waktu (durasi)
- `DateTimeOffset`: DateTime + zona waktu

## 🧾 2. Reference Types (Disimpan di Heap)
- `string`: Teks (immutable)
- `object`: Tipe dasar dari semua tipe
- `dynamic`: Ditetapkan saat runtime
- `var`: Inferensi tipe saat compile-time

### 📦 Koleksi (Generic Collections)
- `int[]`: Array statis
- `List<T>`: List dinamis
- `Dictionary<TKey, TValue>`: Key-value pair
- `HashSet<T>`: Kumpulan elemen unik
- `Queue<T>` / `Stack<T>`: Struktur antrian / tumpukan

## ❓ 3. Nullable Types
Tipe value yang bisa menyimpan `null`:
```csharp
int? umur = null;
bool? aktif = true;
```

## 🧬 4. Enum dan Struct
- `enum`: Kumpulan nilai tetap
```csharp
enum Status { Pending, Proses, Selesai }
```
- `struct`: Value type seperti class
```csharp
struct Point { public int X; public int Y; }
```

## 🧠 5. Record (C# 9.0+)
Tipe referensi immutable, cocok untuk data transfer:
```csharp
public record User(string Nama, int Umur);
```

## 🧪 6. Tuple dan ValueTuple
Struktur data ringkas untuk mengembalikan beberapa nilai:
```csharp
var data = (Nama: "Andi", Umur: 25);
Console.WriteLine(data.Nama); // "Andi"
```

## 🛠️ Contoh Implementasi Lengkap
```csharp
using System;
using System.Collections.Generic;

class Program
{
    enum Status { Pending, Sukses, Gagal }

    struct Point { public int X; public int Y; }

    static void Main()
    {
        int angka = 10;
        float pi = 3.14f;
        bool aktif = true;
        string nama = "Joko";
        DateTime sekarang = DateTime.Now;

        List<string> listNama = new List<string> { "A", "B", "C" };
        Dictionary<string, int> umur = new Dictionary<string, int> {
            { "Andi", 25 }, { "Budi", 30 }
        };

        Status status = Status.Sukses;
        Point p = new Point { X = 5, Y = 10 };

        int? nilai = null;

        var user = (Nama: "Dina", Umur: 28);
        Console.WriteLine(user.Nama);
    }
}
```

---
🧠 **Tips:** Gunakan `var` untuk kode lebih ringkas, tapi hindari jika bikin kode susah dibaca.
