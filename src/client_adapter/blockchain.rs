pub trait Blockchain {
    fn block_header(&self);
    fn block_body(&self);
    fn block_receipt(&self);
    fn best_block_header(&self);

    
    fn import_block(&self);
    fn import_old_block(&self);
}

