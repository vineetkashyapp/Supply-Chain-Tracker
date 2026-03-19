#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub origin: String,
    pub current_owner: String,
    pub history: Vec<String>,
}

#[contract]
pub struct SupplyChainTracker;

#[contractimpl]
impl SupplyChainTracker {

    // Add a new product
    pub fn add_product(env: Env, id: u32, name: String, origin: String, owner: String) {
        let history = Vec::new(&env);
        
        let product = Product {
            id,
            name,
            origin: origin.clone(),
            current_owner: owner.clone(),
            history,
        };

        env.storage().instance().set(&id, &product);
    }

    // Transfer ownership
    pub fn transfer_product(env: Env, id: u32, new_owner: String) {
        let mut product: Product = env
            .storage()
            .instance()
            .get(&id)
            .expect("Product not found");

        // Add previous owner to history
        product.history.push_back(product.current_owner.clone());

        // Update owner
        product.current_owner = new_owner;

        env.storage().instance().set(&id, &product);
    }

    // Get product details
    pub fn get_product(env: Env, id: u32) -> Product {
        env.storage()
            .instance()
            .get(&id)
            .expect("Product not found")
    }
}