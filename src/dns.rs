use bytes::{BufMut, BytesMut};
use std::sync::Arc;
use tokio::net::UdpSocket;

// 使用常量字节数组预分配响应模板
const RESPONSE_START: &[u8] = &[0x81, 0x80, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01];
const RESPONSE_END: &[u8] = &[
    0xc0, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x01, 0xf4, 0x00, 0x04, 0x7f, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x29, 0x05, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

// 预分配缓冲区大小
const BUFFER_SIZE: usize = 512;

pub struct DnsServer {
    socket: Arc<UdpSocket>,
}

impl DnsServer {
    pub async fn new(addr: &str) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(addr).await?;
        Ok(Self {
            socket: Arc::new(socket),
        })
    }

    fn fake_response(request: &[u8]) -> BytesMut {
        let mut response = BytesMut::with_capacity(BUFFER_SIZE);

        // 复制事务ID
        response.put_slice(&request[..2]);

        // 添加响应头
        response.put_slice(RESPONSE_START);

        // 找到查询名称的结束位置
        let mut offset = 12;
        while offset < request.len() {
            let label_len = request[offset] as usize;
            if label_len == 0 {
                break;
            }
            offset += label_len + 1;
        }

        // 复制查询部分
        response.put_slice(&request[12..offset + 5]);

        // 添加响应尾部
        response.put_slice(RESPONSE_END);

        response
    }

    pub async fn handle_request(&self) -> Result<(), std::io::Error> {
        let mut buf = BytesMut::with_capacity(BUFFER_SIZE);
        buf.resize(BUFFER_SIZE, 0);

        loop {
            let (len, addr) = self.socket.recv_from(&mut buf).await?;

            let request = buf[..len].to_vec();
            let socket = Arc::clone(&self.socket);

            tokio::spawn(async move {
                let response = Self::fake_response(&request);
                if let Err(e) = socket.send_to(&response, &addr).await {
                    eprintln!("Error sending response: {}", e);
                }
            });
        }
    }
}
