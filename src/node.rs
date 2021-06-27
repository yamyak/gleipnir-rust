
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;
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
      println!("Loading local chain");
      Node::load_local_chain_data(&config_path, &mut path, &mut chain);
    }
    else
    {
      // initialize a new blockchain
      println!("Creating new chain");
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
    println!("Create config file");
    Node::create_config_file(config_path, block_path);
    // generate a genesis block
    println!("Create genesis block");
    Node::create_genesis_block(chain);
  }

  fn create_config_file(config_path: &String, block_path: &mut String) {
    // set default block location
    *block_path = String::from("block");
    // load block path into config file and save
    let mut config = Ini::new();
    config.set("default", "local_path", Some((&block_path).to_string()));
    println!("Write path to config");
    config.write(&config_path);
  }

  fn create_genesis_block(chain: &mut Chain) {
    // add empty block to chain
    println!("Add new block to chain");
    chain.add_new_block(Block::new(vec![1], 2, 3));
  }

  fn load_local_chain_data(config_path: &String, block_path: &mut String, chain: &mut Chain) -> bool {
    // check if config file exists
    if Path::new(config_path).exists() {
      println!("Config file exists");
      // if it exists, read config file
      let mut config = Ini::new();
      // load config files
      config.load(config_path);
      // get chain path value
      *block_path = config.get("default", "local_path").unwrap();
      // check that chain data files exist
      if Path::new(&block_path).exists() {
        println!("Block file path exists");
        // iterate through all files in file path
        // each one is 1 block
        for entry in fs::read_dir(&block_path).unwrap() {
          // read binary data from file
          let block_data = fs::read(entry.unwrap().path());
          // convert binary data into a block
          let block = Block::deserialize(block_data.unwrap());
          // add block to chain
          chain.add_new_block(block);
        }
        true
      } else {
        println!("Block file path does not exist");
        false
      }
    } else {
      // cancel startup due to not finding config file
      println!("Config file does not exist");
      false
    }
  }

  fn save_chain_locally(&self) {
    if Path::new(&self.m_path).exists() {
      println!("Clearing out block directory");
      // clear files in location
      fs::remove_dir_all(&self.m_path);
    }
    // create directory
    println!("Create block directory");
    fs::create_dir(&self.m_path);
    // save data to location
    let blocks: &Vec<Block> = self.m_chain.get_chain();
    println!("{}", blocks.len());
    for (i, block) in blocks.iter().enumerate() {
      println!("Save block to file");
      let mut file_name = self.m_path.clone();
      file_name.push_str("\\block.bin");
      let mut file = File::create(&file_name).unwrap();
      file.write_all(block.serialize().as_slice());
    }
  }
}

impl Drop for Node { 
  fn drop(&mut self) {
    self.save_chain_locally();
  }
}
