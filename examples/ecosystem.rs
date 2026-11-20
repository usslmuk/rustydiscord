use rustydiscord::{opcodes, proxy};

fn main() {
    for opcode in [1_u8, 2, 3, 4, 6, 8] {
        assert!(opcodes::is_gateway_send_allowed(opcode));
        println!("gateway send opcode {opcode} allowed");
    }
    assert!(!opcodes::is_gateway_send_allowed(0));
    assert!(opcodes::is_gateway_receive_allowed(10));

    for opcode in [0_u8, 1, 3, 5, 7] {
        assert!(opcodes::is_voice_send_allowed(opcode));
        println!("voice send opcode {opcode} allowed");
    }
    assert!(!opcodes::is_voice_send_allowed(2));
    assert!(opcodes::is_voice_receive_allowed(8));

    let target = ["https:", "", "proxy.local"].join("/");
    let config = proxy::ProxyConfig::new(target, true);
    let headers = vec![
        (String::from("Authorization"), String::from("Bot secret")),
        (
            String::from("Content-Type"),
            String::from("application/json"),
        ),
    ];
    let (url, forwarded) = config.forward("api/v1/users", &headers);
    println!("proxied {url} with {} header(s)", forwarded.len());
}
