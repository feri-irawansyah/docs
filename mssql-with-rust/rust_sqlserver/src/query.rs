use tiberius::{AuthMethod, Client, Config, Query};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

async fn connect_with_host_port() -> Result<Client<Compat<TcpStream>>, Box<dyn std::error::Error>> {
    let mut config = Config::new();
    config.authentication(AuthMethod::Integrated);
    config.host("127.0.0.1");
    config.port(1434);
    config.database("AdventureWorks");
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    let connection = Client::connect(config, tcp.compat_write()).await?;

    Ok(connection)
}

// Create Table
pub async fn create_table() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = connect_with_host_port().await?;

    let query_result = Query::new(
        r#"
        CREATE TABLE dbo.OrderanLaundry
        (
            LaundryOrderID INT IDENTITY CONSTRAINT PK_OrderanLaundry_LaundryOrderID PRIMARY KEY,
            RevisionNumber TINYINT CONSTRAINT DF_OrderanLaundry_RevisionNumber DEFAULT 0 NOT NULL,
            OrderDate DATETIME CONSTRAINT DF_OrderanLaundry_OrderDate DEFAULT getdate() NOT NULL,
            DueDate DATETIME NOT NULL,
            ShipDate DATETIME, 
            Status TINYINT CONSTRAINT DF_OrderanLaundry_Status 
                    DEFAULT 1 NOT NULL CONSTRAINT CK_OrderanLaundry_Status 
                    check ([Status] >= 0 AND [Status] <= 8),
            LaundryOrderNumber AS isnull(N'SO' + CONVERT([NVARCHAR](23), [LaundryOrderID]), N'*** ERROR ***'),
            CreditCardApprovalCode varchar(15),
            SubTotal MONEY CONSTRAINT DF_OrderanLaundry_SubTotal 
                    DEFAULT 0.00 NOT NULL CONSTRAINT CK_OrderanLaundry_SubTotal 
                    check ([SubTotal] >= 0.00),
            TaxAmt MONEY CONSTRAINT DF_OrderanLaundry_TaxAmt 
                    DEFAULT 0.00 NOT NULL CONSTRAINT CK_OrderanLaundry_TaxAmt 
                    check ([TaxAmt] >= 0.00),
            Freight MONEY CONSTRAINT DF_OrderanLaundry_Freight 
                    DEFAULT 0.00 NOT NULL CONSTRAINT CK_OrderanLaundry_Freight 
                    check ([Freight] >= 0.00),
            TotalDue AS isnull([SubTotal] + [TaxAmt] + [Freight], 0),
            Comment NVARCHAR(128),
            rowguid UNIQUEIDENTIFIER DEFAULT newid() NOT NULL,
            ModifiedDate DATETIME
        )
        "#,
    );

    let _ = query_result.execute(&mut connection).await?;
    println!("Created table");

    let _ = connection.close().await?;

    Ok(())
}

