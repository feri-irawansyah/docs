IIS adalah singkatan dari Internet Information Server, yang merupakan sebuah HTTP Web Server yang digunakan dalam sistem operasi server Windows seperti hal dengan Linux memiliki Nginx atau Apache sebagai web servernya.

IIS ini tersedia mulai dari Windows NT 4.0 Server, Windows 2000 Server atau Windows Server 2003 sampai Windows Server 2019. Sistem Operasi Windows yang bisa Anda pilih di layanan VPS Alibaba Cloud adalah Windows Server 2008 R2 dan 2012 R2.

### Kelebihan IIS
- IIS mendukung bahasa .net, yang saat ini hanya bisa digunakan di IIS.
- Tersedia fitur bandwidth throttling, fitur ini bertujuan untuk mencegah sebuah aplikasi menggunakan bandwidth secara berlebihan.
- IIS dapat melakukan isolasi resource untuk sebuah aplikasi website yang dibuat, sehingga apabila terjadi kerusakan pada salah satu website, maka tidak berimbas ke website lain yang ada di server.

### Kekurangan IIS
- IIS hanya dapat digunakan di sistem operasi Windows.
- Memerlukan resource server yang cukup besar untuk menggunakannya.
- Keamanan server khususnya pada port http/80 masih rentan, sehingga sangat rawan terkena serangan/hack.

## Requirement Install IIS web server
Sebelum lu install iis ada beberapa kebutuhan terlebih dahulu bang, yaitu:
### Operating System
- Windows Server 2016 +
- Windows Server 2012 R2
- Windows Server 2012 Original
- Windows Server 2008 Original
### Microsoft Internet Information Services (IIS)
- IIS 10
- IIS 8.5
- IIS 8.0
- IIS 7.5
### Hardware Minimal banget
- Prosesor/CPU: 1 gigahertz (GHz) or faster processor or SoC
- RAM: 1 gigabyte (GB) for 32-bit or 2 GB for 64-bit
- Disk: 16 GB for 32-bit OS or 20 GB for 64-bit OS
- VGA: DirectX 9 or later with WDDM 1.0 driver
- Display: 800 x 600

## Cara Install IIS
Tadi harusnya lu udah baca pre requisite nya bang. Jadi sekarang kita install IISnya bang.
### 1. Remote dulu pake RDP
![Server List](https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/cara_install_iis_server/assets/rdp.png)

Kalo udh bisa terhubung ke server langkah selanjutnya ke server manager bang.

---
### 2. Ke Menu Windows
![Windows Menu](https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/cara_install_iis_server/assets/window.png)

Menunya ada di pojok kiri bawah bang, kalo ngga ada tekan tombol windows di keyboard aja.

---
### 3. Pilih Server Manager
![Server Manager](https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/cara_install_iis_server/assets/server-manager.png)

Kalo udh kebuka nanti `Add Role and Features` bang. Kurang lebibh kaya gini tampilannya:

![Role Features](https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/cara_install_iis_server/assets/role-features.png)

---

Setelah di klik biasanya muncul Window baru bang kaya gini, nanti tinggal `Next` aja:

![Preparation](https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/cara_install_iis_server/assets/prepare.png)

---

Setelah next lalu pilih yang `Role-based or feature-based installation` bang, dan selanjutnya `Next` aja:
![Role Based and Feature](https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/cara_install_iis_server/assets/role-based.png)