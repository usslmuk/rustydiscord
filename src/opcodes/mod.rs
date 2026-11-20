#![cfg_attr(docsrs, feature(doc_cfg))]

pub const GATEWAY_SEND_ALLOWLIST: &[u8] = &[1, 2, 3, 4, 6, 8];

pub const GATEWAY_RECEIVE_ALLOWLIST: &[u8] = &[0, 1, 7, 9, 10, 11];

pub const VOICE_SEND_ALLOWLIST: &[u8] = &[0, 1, 3, 5, 7];

pub const VOICE_RECEIVE_ALLOWLIST: &[u8] = &[2, 4, 5, 6, 8, 9];

#[must_use]
pub fn is_gateway_send_allowed(opcode: u8) -> bool {
    GATEWAY_SEND_ALLOWLIST.contains(&opcode)
}

#[must_use]
pub fn is_gateway_receive_allowed(opcode: u8) -> bool {
    GATEWAY_RECEIVE_ALLOWLIST.contains(&opcode)
}

#[must_use]
pub fn is_voice_send_allowed(opcode: u8) -> bool {
    VOICE_SEND_ALLOWLIST.contains(&opcode)
}

#[must_use]
pub fn is_voice_receive_allowed(opcode: u8) -> bool {
    VOICE_RECEIVE_ALLOWLIST.contains(&opcode)
}

#[cfg(test)]
mod tests {
    use super::{
        is_gateway_receive_allowed, is_gateway_send_allowed, is_voice_receive_allowed,
        is_voice_send_allowed,
    };

    #[test]
    fn gateway_send_allows_client_opcodes() {
        for opcode in [1_u8, 2, 3, 4, 6, 8] {
            assert!(is_gateway_send_allowed(opcode));
        }
        for opcode in [0_u8, 5, 7, 9, 10, 11] {
            assert!(!is_gateway_send_allowed(opcode));
        }
    }

    #[test]
    fn gateway_receive_allows_server_opcodes() {
        for opcode in [0_u8, 1, 7, 9, 10, 11] {
            assert!(is_gateway_receive_allowed(opcode));
        }
        for opcode in [2_u8, 3, 4, 5, 6, 8] {
            assert!(!is_gateway_receive_allowed(opcode));
        }
    }

    #[test]
    fn voice_send_allows_client_opcodes() {
        for opcode in [0_u8, 1, 3, 5, 7] {
            assert!(is_voice_send_allowed(opcode));
        }
        for opcode in [2_u8, 4, 6, 8, 9] {
            assert!(!is_voice_send_allowed(opcode));
        }
    }

    #[test]
    fn voice_receive_allows_server_opcodes() {
        for opcode in [2_u8, 4, 5, 6, 8, 9] {
            assert!(is_voice_receive_allowed(opcode));
        }
        for opcode in [0_u8, 1, 3, 7] {
            assert!(!is_voice_receive_allowed(opcode));
        }
    }
}
