Woi brooo....

Jaman sekarang gini Lo masih mikirin selector - selector buat bikin tanda biar website Lo bisa Lo kasih make up. Terus karena Lo punya selector yang sama, jadinya rebutan dan biar itu ga kejadian Lo harus spesifik nyebutin sampe panjang banget kaya tenor pinjaman Lo. Terus Lo coba pake `pre processor` tapi Lo ga paham logic programming, endingnya Lo tetep nulis selector nested hell sampe kaya gunung ketinggan 99999999 mdpl.

```scss
.page {
  .header {
    .nav {
      ul {
        li {
          a {
            span {
              color: red;

              &:hover {
                color: blue;
              }
            }
          }
        }
      }
    }
  }
}
```

Disitulah muncul yang namanya CSS Framework biar mempermudah hidup Lo. Ya tapi balik lagi **sesuatu yang harusnya mempermudah bisa jadi bikin Lo susah**. Sebenernya CSS Framework itu hadir buat bantu dev - dev kaya Lo pada karena dari warna, layout, ukuran, psudo, responsive para engineer - engineer besar udah buatin dan Lo tinggal pake. Nah cuma masalahnya kalo Lo langsung pake tanpa tau dasarnya yaitu `CSS` (Casecading Style Sheet) bisa jadi Lo bakal bikin website yang kalo Lo buka malah tampilannya bikin sakit mata.