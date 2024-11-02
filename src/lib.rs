#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, U256}, alloy_sol_types::sol, block, call::transfer_eth, contract, evm, msg, prelude::*
};

sol! {
    event Visit(address indexed sender, string message);
    error InsufficientPayment(address visitor, uint256 payment);
    error TransferFailed(address recipient, uint256 amount);
    error AlreadyVisited();
    error IndexOutOfBounds();
}

sol_storage! {
    #[entrypoint]
    pub struct Oracle {
        mapping(uint256 => uint256) averages;
    }
}

#[public]
impl Oracle {
    pub fn calculate_average(&mut self) {
        let numbers: Vec<U256> = vec![
            U256::from(1111),
            U256::from(2222),
            U256::from(3333),
            U256::from(4444),
            U256::from(5555),
            U256::from(6666),
            U256::from(7777),
            U256::from(8888),
            U256::from(9999),
            U256::from(1111),
            U256::from(1111),
            U256::from(2222),
            U256::from(3333),
            U256::from(4444),
            U256::from(5555),
            U256::from(6666),
            U256::from(7777),
            U256::from(8888),
            U256::from(9999),
            U256::from(1111),
        ];

        let sum: U256 = numbers.iter().sum();
        let average: U256 = sum / U256::from(numbers.len());

        self.averages.insert(U256::from(block::timestamp()), average);
    }

}