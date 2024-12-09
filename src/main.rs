mod dns;

use dns::DnsServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建服务器实例
    let addr = "127.0.0.1:53";
    let server = DnsServer::new(addr).await?;
    println!("[+] DNS server listening on {}", addr);

    // 启动服务器
    server.handle_request().await?;

    Ok(())
}
