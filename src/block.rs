use std::convert::TryInto;

pub struct Block {
  m_data : Vec<i32>,
  m_block_id: i32,
  m_prev_block_id : i32
}

impl Block {
  pub fn new(data: Vec<i32>, block_id: i32, old_block_id: i32) -> Block {
    Block {
      m_data: data,
      m_block_id: block_id,
      m_prev_block_id: old_block_id
    }
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut binary= self.m_block_id.to_be_bytes().to_vec();
    binary.append(&mut self.m_prev_block_id.to_be_bytes().to_vec());
    binary.append(&mut self.m_data.len().to_be_bytes().to_vec());
    for data in &self.m_data {
      binary.append(&mut data.to_be_bytes().to_vec());
    }
    binary
  }

  pub fn deserialize(stream: Vec<u8>) -> Block {
    let block_id = i32::from_be_bytes(stream[0..4].try_into().unwrap());
    let prev_block_id = i32::from_be_bytes(stream[4..8].try_into().unwrap());
    
    let size = usize::from_be_bytes(stream[8..16].try_into().unwrap());
    let mut data: Vec<i32> = vec![];
    for index in 0..size {
      let start = 16 + index * 4;
      let subdata = i32::from_be_bytes(stream[start..(start+4)].try_into().unwrap());
      data.push(subdata);
    }

    Block {
      m_data: data,
      m_block_id: block_id,
      m_prev_block_id: prev_block_id
    }
  }
}