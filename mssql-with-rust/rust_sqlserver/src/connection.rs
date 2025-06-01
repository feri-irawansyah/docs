use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

// Connect with Windows Authentication
pub async fn connect_with_host_port() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();

    // Windows Authentication (SSPI)
    config.authentication(AuthMethod::Integrated);

    // Ganti ini sesuai alamat & port servermu
    config.host("127.0.0.1");
    config.port(1434); // atau port hasil dari konfigurasi
    config.trust_cert(); // opsional jika SSL tidak disertifikasi

    let tcp = TcpStream::connect(config.get_addr()).await?; // koneksi ke server
    let connection = Client::connect(config, tcp.compat_write()).await?; 
    println!("Connected with Windows Authentication!");
    connection.close().await?;

    Ok(())
}

// Connect with SQL Server Authentication
pub async fn connect_with_host_port_username_password() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();

    // Use SQL Server Authentication (user name dan password)
    config.authentication(AuthMethod::sql_server("sa", "Snakesystem@09"));

    config.host("127.0.0.1");
    config.port(1434);
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    let connection = Client::connect(config, tcp.compat_write()).await?;
    println!("Connected to SQL Server");
    let _ = connection.close().await?;

    Ok(())
}

// Connect with ADO.NET
pub async fn connect_with_ado_host_port() -> Result<(), Box<dyn std::error::Error>> {
    // It uses an ADO.NET connection string to connect to SQL Server.
    // Replace with your actual connection string
    let config = Config::from_ado_string(
        &"Server=tcp:127.0.0.1,1434;IntegratedSecurity=true;TrustServerCertificate=true",
    )?;

    let tcp = TcpStream::connect(config.get_addr()).await?;
    let _ = tcp.set_nodelay(true);

    let connection = Client::connect(config, tcp.compat_write()).await?;
    println!("Connected to SQL Server");
    let _ = connection.close().await;

    Ok(())
}

// Connect with JDBC
pub async fn connect_with_jdbc_host_port() -> Result<(), Box<dyn std::error::Error>> {
    // pake JDBC format connection string
    let config = Config::from_jdbc_string(
        &"jdbc:sqlserver://127.0.0.1:1434;integratedSecurity=true;trustServerCertificate=true",
    )?;

    let tcp = TcpStream::connect(config.get_addr()).await?;
    let _ = tcp.set_nodelay(true);

    let connection = Client::connect(config, tcp.compat_write()).await?;
    println!("Connected to SQL Server");
    let _ = connection.close().await;

    Ok(())
}