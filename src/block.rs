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
}