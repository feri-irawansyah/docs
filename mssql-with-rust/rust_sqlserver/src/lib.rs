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
    let client = Client::connect(config, tcp.compat_write()).await?; 
    println!("Connected with Windows Authentication!");
    client.close().await?;

    Ok(())
}

#[tokio::test]
async fn connect_to_sql_server_using_host_port() {
    let result = connect_with_host_port().await;
    assert_eq!(result.is_ok(), true);
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
    let client = Client::connect(config, tcp.compat_write()).await?;
    println!("Connected to SQL Server");
    let _ = client.close().await?;

    Ok(())
}

#[tokio::test]
async fn connect_to_sql_server_using_host_port_username_password() {
    let result = connect_with_host_port_username_password().await;
    assert_eq!(result.is_ok(), true);
}

