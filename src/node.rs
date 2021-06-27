use std::path::Path;
use configparser::ini::Ini;

use crate::chain::Chain;
use crate::block::Block;

pub struct Node {
  m_chain : Chain,
  m_path : String
}

impl Node {
  // constructor
  pub fn new(config_path: String) -> Self {
    // create a new chain
    let mut chain = Chain::new();
    let mut path = String::new();
    // if the config file exists
    if Path::new(&config_path).exists() {
      // load the local chain
      Node::load_local_chain_data(&config_path, &mut path, &mut chain);
    }
    else
    {
      // initialize a new blockchain
      Node::initialize_blockchain(&config_path, &mut path, &mut chain);
    }
    // return
    Self {
      m_chain: chain,
      m_path: path
    }
  }
  
  fn initialize_blockchain(config_path: &String, block_path: &mut String, chain: &mut Chain) {
    // create a new config file
    Node::create_config_file(config_path, block_path);
    // generate a genesis block
    Node::create_genesis_block(chain);
  }

  fn create_config_file(config_path: &String, block_path: &mut String) {
    // set default block location
    *block_path = String::from("block");
    // load block path into config file and save
    let mut config = Ini::new();
    config.set("default", "local_path", Some((&block_path).to_string()));
    config.write(&config_path);
  }

  fn create_genesis_block(chain: &mut Chain) {
    // add empty block to chain
    chain.add_new_block(Block::new(vec![0], 0, 0));
  }

  fn load_local_chain_data(config_path: &String, block_path: &mut String, chain: &mut Chain) -> bool {
    // check if config file exists
    if Path::new(config_path).exists() {
      // if it exists, read config file
      let mut config = Ini::new();
      // load config files
      config.load(config_path);
      // get chain path value
      *block_path = config.get("default", "local_path").unwrap();
      // check that chain data files exist
      if Path::new(&block_path).exists() {
        println!("Path found");
        // parse data and convert into blocks
        // load blocks onto chain
        true
      } else {
        println!("Path not found");
        false
      }
    } else {
      // cancel startup due to not finding config file
      println!("Nothing found");
      false
    }
  }

  fn save_chain_locally(&self) {
    println!("Kill it: {}", self.m_path);

    if Path::new(&self.m_path).exists() {
      println!("Path found");
      // clear files in location
    } else {
      // create folder location
    }

    // save data to location
  }
}

impl Drop for Node { 
  fn drop(&mut self) {
    self.save_chain_locally();
  }
}
