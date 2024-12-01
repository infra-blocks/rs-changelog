use itertools::PutBackN;

pub trait PutBackChunk<C> {
    fn put_back_chunk(&mut self, chunk: C);
}

impl<I, Chunk, ChunkIntoIter, ChunkItem> PutBackChunk<Chunk> for PutBackN<I>
where
    // The type inside PutBackN has to be an iterator.
    I: Iterator,
    // The chunk has to be convertible into an iterator.
    Chunk: IntoIterator<IntoIter = ChunkIntoIter>,
    // The chunk into iter has to be double ended iterator of chunk items.
    ChunkIntoIter: DoubleEndedIterator<Item = ChunkItem>,
    // The chunk items can be converted into the iterator item.
    ChunkItem: Into<I::Item>,
{
    fn put_back_chunk(&mut self, chunk: Chunk) {
        for item in chunk.into_iter().rev() {
            self.put_back(item.into());
        }
    }
}
