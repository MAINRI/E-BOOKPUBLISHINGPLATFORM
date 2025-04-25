#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct EBook {
    pub book_id: u64,
    pub title: String,
    pub author: String,
    pub content_hash: String, // To store the IPFS hash or any content identifier
    pub published: bool,
}

#[contracttype]
pub enum EBookBook {
    Book(u64),
}

const BOOK_COUNT: Symbol = symbol_short!("BOOK_COUNT");

#[contract]
pub struct EBookPublishingPlatform;

#[contractimpl]
impl EBookPublishingPlatform {
    // Publish a new eBook
    pub fn publish_book(env: Env, title: String, author: String, content_hash: String) -> u64 {
        let mut count = env.storage().instance().get(&BOOK_COUNT).unwrap_or(0);
        count += 1;

        let book = EBook {
            book_id: count,
            title,
            author,
            content_hash,
            published: true,
        };

        env.storage().instance().set(&EBookBook::Book(count), &book);
        env.storage().instance().set(&BOOK_COUNT, &count);
        count
    }

    // View details of a published book
    pub fn view_book(env: Env, book_id: u64) -> EBook {
        env.storage()
            .instance()
            .get(&EBookBook::Book(book_id))
            .unwrap_or(EBook {
                book_id: 0,
                title: String::from_str(&env, "Not Found"),
                author: String::from_str(&env, "Not Found"),
                content_hash: String::from_str(&env, "Not Found"),
                published: false,
            })
    }

    // Check total number of books published
    pub fn total_books(env: Env) -> u64 {
        env.storage().instance().get(&BOOK_COUNT).unwrap_or(0)
    }
}