// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod credentials;
mod verus_rpc;
mod settings;

fn main() {
    chat_dapp_lib::run()
}

// This is a separate crate, defined in lib.rs 
// We'll need to update lib.rs to register our commands
