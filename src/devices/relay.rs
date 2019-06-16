#[derive(Debug)]
pub struct Relay {
    address: u8,
    controller_address: u8,
    state: bool
}

impl Relay {
    fn new(address: u8, controller_address: u8) -> Relay {
        Relay {
            address,
            controller_address,
            state: false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_relay() {
        let r = Relay::new(0, 2);
        assert_eq!(r.address, 0);
        assert_eq!(r.controller_address, 2);
    }
}
