use ed25519_dalek::Keypair;
use rustblockchainlib::*;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use serde::{Deserialize, Serialize};

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};




use tentacle::{
    builder::{MetaBuilder, ServiceBuilder},
    bytes::Bytes,
    context::{ProtocolContext, ProtocolContextMutRef, ServiceContext},
    secio::{peer_id::PeerId, SecioKeyPair},
    service::{ProtocolHandle, ServiceEvent, TargetProtocol, TargetSession},
    traits::{ServiceHandle, ServiceProtocol},
    SessionId,
};

//using tentacle for peer to peer between nodes and clients

//take transaction requests from clients with scaling
// get client wallet public key
//spawn threads for mining while listening

//set up local network testing with multiplexing 

//message handling should be looped


fn main() {
    let mut blockchain = Blockchain::new();

    let ryan_key = Wallet::new();
    let dan_key = Wallet::new();

    let miner_key = Wallet::new();

    let mut first_transaction = Transaction {
        sender: Some(ryan_key.public),
        receiver: Some(dan_key.public),
        amount: 2000.0,
        signature: None,
    };

    first_transaction.sign_transaction(Keypair {
        public: ryan_key.public,
        secret: ryan_key.secret,
    });

    blockchain.add_new_transaction(first_transaction);

    blockchain.mine_unmined_transactions(miner_key.public);

    let sam_key = Wallet::new();
    let michal_key = Wallet::new();

    let mut second_transaction = Transaction {
        sender: Some(sam_key.public),
        receiver: Some(michal_key.public),
        amount: 2500.0,
        signature: None,
    };

    second_transaction.sign_transaction(Keypair {
        public: sam_key.public,
        secret: sam_key.secret,
    });

    blockchain.add_new_transaction(second_transaction);

    blockchain.mine_unmined_transactions(miner_key.public);

    let mut third_transaction = Transaction {
        sender: Some(michal_key.public),
        receiver: Some(dan_key.public),
        amount: 1000.0,
        signature: None,
    };

    third_transaction.sign_transaction(Keypair {
        public: michal_key.public,
        secret: michal_key.secret,
    });

    blockchain.add_new_transaction(third_transaction);

    blockchain.mine_unmined_transactions(miner_key.public);

    println!("{}", blockchain.is_valid_chain());
    println!("{:#?}", blockchain);
}


//after blocked has been mined broadcast to other nodes
