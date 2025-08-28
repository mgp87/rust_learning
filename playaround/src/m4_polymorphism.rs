#[cfg(test)]
mod tests {

    use ethers::types::Address;
    use std::str::FromStr;

    trait EthereumAddress {
        fn convert_address(&self) -> Result<Address, &'static str>;
    }

    impl EthereumAddress for &str {
        fn convert_address(&self) -> Result<Address, &'static str> {
            match Address::from_str(self) {
                Ok(address) => Ok(address),
                Err(_) => Err("Invalid Ethereum address format"),
            }
        }
    }

    impl EthereumAddress for Address {
        fn convert_address(&self) -> Result<Address, &'static str> {
            Ok(*self)
        }
    }

    fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
        let converted_address: Address = address.convert_address().unwrap();
        converted_address
    }

    #[test]
    fn test_polymorphism() {
        let addr: Address =
            Address::from_str("0x0000000000000000000000000000000000000000").unwrap();
        let new_addr: Address = get_ethereum_data(addr);
        assert_eq!(new_addr, addr);

        let new_addr2: Address = get_ethereum_data("0x0000000000000000000000000000000000000000");
        assert_eq!(new_addr2, addr);
    }
}
