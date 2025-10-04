requirement:
- Visual Studio Code
- NodeJS v22
- .NET core 9

dotnet hosting untuk iis https://builds.dotnet.microsoft.com/dotnet/aspnetcore/Runtime/9.0.9/dotnet-hosting-9.0.9-win.exe

1. Buat project sveltekit

```bash
npx sv create AplikasiSampah
```
- SvelteKit minal (barebones scaffolding for your new app)
- No (ts)
- tailwindcss
- typography (@tailwindcss/typography)
- forms (@tailwindcss/forms)
- npm

```bash
cd AplikasiSampah

npm run dev

http://localhost:5173
```

1. Buat project dotnet

```bash
dotnet new webapi -n services

cd services

dotnet run

http://localhost:5018/weatherforecast
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/dotnet-svelte-tailwind/assets/run-project.png" class="img-fluid">

Cara menyatukan url dan cors origin.

```js
// vite.config.js

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	server: {
		proxy: {
			'/api': {
				target: 'http://localhost:5018/api', // url backend
				changeOrigin: true,
				secure: false,
				rewrite: (path) => path.replace(/^\/api/, ''),
			},
		}
	},
});
```

Tambahkan ini di appsettings.json

```json
{
    "ConnectionStrings": {
    "DefaultConnection": "Server=localhost; Database=DatabaseSampah; User Id=sa; Password=123456; Encrypt=True; TrustServerCertificate=True; MultipleActiveResultSets=True;"
  }
}
```

Ketikkan

```bash
dotnet add package Microsoft.EntityFrameworkCore.SqlServer --version 9.0.9
dotnet add package Microsoft.EntityFrameworkCore.Design --version 9.0.9
dotnet add package Microsoft.EntityFrameworkCore.Tools --version 9.0.9
dotnet add package Swashbuckle.AspNetCore --version 6.6.2
```

Install ini

```bash
dotnet tool install --global dotnet-ef
```

Arsitektur

```bash
AplikasiSampah/
├─ Controllers/
│  └─ TrashController.cs
├─ Context/
│  └─ DataContext.cs
├─ Models/
│  └─ Trash.cs
├─ Services/
│  └─ Trashservice.cs
├─ Repositories/
│  └─ ITrashRepository.cs # ini kita gabung aja ke TrashRepository
│  └─ TrashRepository.cs
├─ appsettings.json
└─ Program.cs
```

```rust
// Context/DataContext.cs
using Microsoft.EntityFrameworkCore;
using Services.Models;

namespace Services.Context;
public class DataContext : DbContext
{
    public DataContext(DbContextOptions<DataContext> options)
        : base(options) { }

    public DbSet<Trash> Trashs => Set<Trash>();

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);
        modelBuilder.Entity<Trash>().HasKey(p => p.TrashNID);
        // tambahan konfigurasi fluent API kalau perlu
    }
}
```

```rust
// models/Sampah.cs
namespace AplikasiSampah.Models;
public class Trash
{
    public int TrashNID { get; set; }
    public string TrashName { get; set; } = string.Empty;
    public decimal Price { get; set; }
    public bool Organic { get; set; }
    public string Image { get; set; } = string.Empty;
    public DateTime LastUpdate { get; set; } = DateTime.UtcNow;
}
```

```rust
// repositories/SampahRepository.cs
using Microsoft.EntityFrameworkCore;
using AplikasiSampah.Context;
using AplikasiSampah.Models;

namespace AplikasiSampah.Repositories
{
    // --- Interface ---
    public interface ITrashRepository
    {
        Task<IEnumerable<Trash>> GetAllAsync();
        Task<Trash?> GetByIdAsync(int id);
        Task AddAsync(Trash trash);
        Task UpdateAsync(Trash trash);
        Task DeleteAsync(int id);
    }

    // --- Implementasi ---
    public class TrashRepository : ITrashRepository
    {
        private readonly DataContext _context;

        public TrashRepository(DataContext context)
        {
            _context = context;
        }

        public async Task<IEnumerable<Trash>> GetAllAsync()
            => await _context.Trashs.OrderBy(p => p.TrashNID).ToListAsync();

        public async Task<Trash?> GetByIdAsync(int id)
            => await _context.Trashs.FindAsync(id);

        public async Task AddAsync(Trash trash)
        {
            _context.Trashs.Add(trash);
            await _context.SaveChangesAsync();
        }

        public async Task UpdateAsync(Trash trash)
        {
            _context.Trashs.Update(trash);
            await _context.SaveChangesAsync();
        }

        public async Task DeleteAsync(int id)
        {
            var p = await _context.Trashs.FindAsync(id);
            if (p is null) return;
            _context.Trashs.Remove(p);
            await _context.SaveChangesAsync();
        }
    }
}
```

```rust
// services/TrashService.cs
using Services.Models;
using Services.Repositories;

public class TrashService
{
    private readonly ITrashRepository _repo;
    public TrashService(ITrashRepository repo) => _repo = repo;

    public Task<IEnumerable<Trash>> GetAll() => _repo.GetAllAsync();
    public Task<Trash?> Get(int id) => _repo.GetByIdAsync(id);
    public Task Add(Trash p) => _repo.AddAsync(p);
    public Task Update(Trash p) => _repo.UpdateAsync(p);
    public Task Delete(int id) => _repo.DeleteAsync(id);
}
```

```rust
// controllers/TrashController.cs
using Microsoft.AspNetCore.Mvc;
using Services.Models;

[ApiController]
[Route("api/[controller]")]
public class TrashController : ControllerBase
{
    private readonly TrashService _service;
    public TrashController(TrashService svc) => _service = svc;

    [HttpGet]
    public async Task<IActionResult> Get() => Ok(await _service.GetAll());

    [HttpGet("{id}")]
    public async Task<IActionResult> Get(int id)
    {
        var p = await _service.Get(id);
        if (p is null) return NotFound();
        return Ok(p);
    }

    [HttpPost]
    public async Task<IActionResult> Post(Trash trash)
    {
        await _service.Add(trash);
        return CreatedAtAction(nameof(Get), new { id = trash.TrashNID }, trash);
    }

    [HttpPut("{id}")]
    public async Task<IActionResult> Put(int id, Trash trash)
    {
        if (id != trash.TrashNID) return BadRequest();
        await _service.Update(trash);
        return NoContent();
    }

    [HttpDelete("{id}")]
    public async Task<IActionResult> Delete(int id)
    {
        await _service.Delete(id);
        return NoContent();
    }
}
```

```rust
// Program.cs
using Microsoft.EntityFrameworkCore;
using Services.Context;
using Services.Repositories;

public class Program
{
    public static void Main(string[] args)
    {
        var builder = WebApplication.CreateBuilder(args);

        // Add services to the container
        builder.Services.AddControllers(); // ⬅️ ini yang wajib
        // Learn more about configuring OpenAPI at https://aka.ms/aspnet/openapi
        builder.Services.AddOpenApi();

        // DbContext
        builder.Services.AddDbContext<DataContext>(options =>
            options.UseSqlServer(builder.Configuration.GetConnectionString("DefaultConnection")));

        builder.Services.AddEndpointsApiExplorer();
        builder.Services.AddSwaggerGen();

        // Registrasi service/repository
        builder.Services.AddScoped<ITrashRepository, TrashRepository>();
        builder.Services.AddScoped<TrashService>();

        // ✅ Add Authorization biar UseAuthorization() ga error
        builder.Services.AddAuthorization();

        var app = builder.Build();

        // Configure the HTTP request pipeline
        if (app.Environment.IsDevelopment())
        {
            app.UseSwagger();
            app.UseSwaggerUI();
        }

        app.UseHttpsRedirection();

        app.UseAuthorization();

        app.MapControllers();

        app.Run();
    }
}
```

```bash
dotnet ef migrations add InitialCreate

dotnet ef database update

dotnet run

http://localhost:5000/api/trash

http://localhost:5000/swagger
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/dotnet-svelte-tailwind/assets/swagger.png" class="img-fluid">

Tambah sampah swagger

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/dotnet-svelte-tailwind/assets/tambah.png" class="img-fluid">

```json
{
  "trashNID": 0,
  "trashName": "Plastik Kresek",
  "price": 2000,
  "organic": false,
  "image": "http://localhost:5173/img/plastik-kerek.png", // url frontend
  "lastUpdate": "2025-10-04T09:36:38.705Z"
}
```

<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/dotnet-svelte-tailwind/assets/add.png" class="img-fluid">

## Frontend

```html
<script>
  import { onMount } from "svelte";

  let rows = [];

    onMount(() => {
        fetch("/api/trash")
        .then(res => res.json())
        .then(data => {
            rows = data
        });
    })

</script>

<h1>Welcome to Aplikasi Sampah</h1>
<p>Buanglah sampah pada tempatnya! Jangan jorok dan jaga lingkungan!</p>

<table>
    <thead>
        <tr>
            <th>Id</th>
            <th>Nama Sampah</th>
            <th>Harga</th>
            <th>Organic</th>
            <th>Gambar</th>
            <th>Last Update</th>
        </tr>
    </thead>
    <tbody>
        {#each rows as row}
            <tr>
                <td>{row.trashNID}</td>
                <td>{row.trashName}</td>
                <td>{row.price}</td>
                <td>{row.organic ? "Yes" : "No"}</td>
                <td>
                    <img width="100" src={row.image} alt={row.trashName} />
                </td>
                <td>{new Date(row.lastUpdate).toLocaleString()}</td>
            </tr>
        {/each}
    </tbody>
</table>
```
<img src="https://raw.githubusercontent.com/feri-irawansyah/docs/refs/heads/main/dotnet-svelte-tailwind/assets/frontend.png" class="img-fluid">

Sebelum mulai CRUD nya buat auth dulu

Install lib

```bash
dotnet add package Microsoft.AspNetCore.Authentication.JwtBearer
dotnet add package Microsoft.IdentityModel.Tokens
dotnet add package Microsoft.AspNetCore.Identity
dotnet add package Swashbuckle.AspNetCore
```

Tambah di bawah connection string

```json
{
    "ConnectionStrings": {
      // ...
    },
    "Jwt": {
        "Key": "masukin-secret-key-panjang-dan-aman-disini-ubah-di-prod",
        "Issuer": "my-app",
        "Audience": "my-app-audience",
        "ExpiresMinutes": 60
    },
}
```