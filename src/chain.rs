use crate::block::Block;

pub struct Chain {
  m_blocks : Vec<Block>
}

impl Chain {
  pub fn new() -> Chain {
    Chain {
      m_blocks: Vec::new()
    }
  }

  pub fn add_new_block(&mut self, block: Block)
  {
    self.m_blocks.push(block);
  }
}