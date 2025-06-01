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
    