use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn};
use openapi::{
    apis::{blockflow_api, configuration::Configuration},
    models::BlockAndEvents,
};
use std::{collections::VecDeque, fmt};
use tokio::sync::mpsc;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, Utf8Bytes},
};

use crate::{Notification, SimpleEventType, SubscribeParams, WsResponse, build_subscribe_request};

const GROUP_SIZE: u8 = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChainIndex {
    pub from: u8,
    pub to: u8,
}

trait BlockExt {
    fn hash(&self) -> &str;
    fn height(&self) -> u32;
    fn chain_index(&self) -> ChainIndex;
    fn parent_hash(&self) -> &str;
}

impl BlockExt for BlockAndEvents {
    #[inline]
    fn hash(&self) -> &str {
        self.block.hash.as_str()
    }

    #[inline]
    fn height(&self) -> u32 {
        self.block.height as u32
    }

    #[inline]
    fn chain_index(&self) -> ChainIndex {
        ChainIndex {
            from: self.block.chain_from as u8,
            to: self.block.chain_to as u8,
        }
    }

    #[inline]
    fn parent_hash(&self) -> &str {
        let to_index = self.block.chain_to as u8;
        let parent_index = GROUP_SIZE - 1 + to_index;
        self.block.deps[parent_index as usize].as_str()
    }
}

#[derive(Debug)]
pub struct BlockAtHeight {
    pub height: u32,
    pub blocks: Vec<BlockAndEvents>,
}

impl BlockAtHeight {
    pub fn from(block: &BlockAndEvents) -> BlockAtHeight {
        BlockAtHeight {
            height: block.height(),
            blocks: vec![block.clone()],
        }
    }

    pub fn add_unsafe(&mut self, block: BlockAndEvents) {
        self.blocks.push(block);
    }

    pub fn add(&mut self, block: &BlockAndEvents) {
        assert!(block.height() == self.height);
        if self
            .blocks
            .iter()
            .find(|b| b.hash() == block.hash())
            .is_none()
        {
            self.add_unsafe(block.clone());
        }
    }

    pub fn mainchain_block(&self) -> &BlockAndEvents {
        assert!(!self.blocks.is_empty());
        self.blocks.first().unwrap()
    }

    pub fn mainchain_hash(&self) -> &str {
        self.mainchain_block().hash()
    }

    pub fn reorg(&mut self, hash: &str) -> bool {
        let index = self.blocks.iter().position(|b| b.hash() == hash);
        match index {
            Some(index) => {
                self.blocks.swap(0, index);
                true
            }
            None => false,
        }
    }
}

const MAX_REORG_DEPTH: usize = 50;

pub struct Handler {
    chain_index: ChainIndex,
    ws_url: String,
    config: Configuration,
    blocks: VecDeque<BlockAtHeight>,
}

pub enum BlockEvent {
    NewBlock(BlockAndEvents),
    Reorg(Vec<BlockAndEvents>, Vec<BlockAndEvents>),
    Nothing,
}

impl fmt::Debug for BlockEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::NewBlock(block) => write!(
                f,
                "NewBlock(hash: {}, height: {})",
                block.hash(),
                block.height()
            ),
            Self::Reorg(old, new) => write!(
                f,
                "Reorg(old: {}, new: {})",
                old.iter()
                    .map(|b| b.hash().to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                new.iter()
                    .map(|b| b.hash().to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            Self::Nothing => write!(f, "Nothing"),
        }
    }
}

impl Handler {
    pub fn from(ip: &str, port: u16, chain_index: ChainIndex) -> Handler {
        Self {
            chain_index,
            config: Configuration {
                base_path: format!("http://{}:{}", ip, port),
                ..Default::default()
            },
            ws_url: format!("ws://{}:{}/ws", ip, port),
            blocks: VecDeque::new(),
        }
    }

    pub async fn start(&mut self, last_height: u32, last_hash: &str) -> Result<(), String> {
        self.recover(last_hash).await?;
        self.fetch_missing_blocks(last_height + 1).await?;
        self.subscribe_blocks().await
    }

    async fn recover(&mut self, last_hash: &str) -> Result<(), String> {
        let mut blocks: Vec<BlockAndEvents> = vec![];
        let mut hash = last_hash;
        for _ in 0..MAX_REORG_DEPTH {
            let block =
                blockflow_api::get_blockflow_blocks_with_events_block_hash(&self.config, hash)
                    .await
                    .map_err(|e| e.to_string())?;
            blocks.push(block);
            hash = blocks.last().unwrap().parent_hash();
        }
        for block in blocks.into_iter().rev() {
            let block_at_height = BlockAtHeight {
                height: block.height(),
                blocks: vec![block],
            };
            self.blocks.push_back(block_at_height);
        }
        assert!(self.blocks.len() == MAX_REORG_DEPTH);
        Ok(())
    }

    async fn fetch_missing_blocks(&mut self, from_height: u32) -> Result<(), String> {
        let chain_info = blockflow_api::get_blockflow_chain_info(
            &self.config,
            self.chain_index.from as i32,
            self.chain_index.to as i32,
        )
        .await
        .map_err(|e| format!("failed to request chain info due to {:?}", e))?;
        let to_height = chain_info.current_height as u32;
        if from_height < to_height {
            let mut from = from_height;
            let batch_size: u32 = 100;
            while from <= to_height {
                let end = (from + batch_size - 1).min(to_height);
                let blocks = get_blocks(&self.config, &self.chain_index, from, end).await?;
                self.handle_blocks(blocks).await?;
                from = end + 1;
            }
            Box::pin(self.fetch_missing_blocks(to_height + 1)).await
        } else {
            Ok(())
        }
    }

    async fn subscribe_blocks(&mut self) -> Result<(), String> {
        let (ws_stream, _) = connect_async(&self.ws_url)
            .await
            .map_err(|e| e.to_string())?;
        let params = SubscribeParams::Simple(SimpleEventType::Block);
        let (mut write, mut read) = ws_stream.split();
        let request = build_subscribe_request(params, 0).unwrap();
        write
            .send(Message::Text(Utf8Bytes::from(request)))
            .await
            .map_err(|e| e.to_string())?;

        let (tx, mut rx) = mpsc::channel::<BlockAndEvents>(1024);
        let chain_index = self.chain_index.clone();

        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        let response: WsResponse = serde_json::from_str(text.as_str()).unwrap();
                        match response {
                            WsResponse::Notification(notification) => {
                                match notification.params {
                                    Notification::Block(block)
                                        if chain_index == block.result.chain_index() =>
                                    {
                                        match tx.send(block.result).await {
                                            Ok(_) => (),
                                            Err(error) => {
                                                // TODO: better error handler
                                                error!(
                                                    "failed to send block to channel: {:?}",
                                                    error
                                                );
                                                break;
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                            WsResponse::JsonRpcResponse(_) => (),
                        }
                    }
                    Ok(msg) => warn!("received unknown message: {:?}", msg),
                    Err(e) => {
                        // TODO: better error handler
                        error!("block subscription error: {:?}", e);
                        break;
                    }
                }
            }
        });

        while let Some(block) = rx.recv().await {
            let block_event = self.handle_new_block(block).await?;
            self.handle_block_event(block_event).await?;
        }
        Ok(())
    }

    async fn handle_block_event(&self, event: BlockEvent) -> Result<(), String> {
        info!("block event: {:?}", event);
        Ok(())
    }

    pub async fn handle_new_block(&mut self, block: BlockAndEvents) -> Result<BlockEvent, String> {
        if self.blocks.is_empty() {
            self.add_new_block(&block);
            return Ok(BlockEvent::NewBlock(block));
        }

        let last_height = self.blocks.back().unwrap();
        if block.height() == last_height.height + 1 {
            if last_height.mainchain_hash() == block.parent_hash() {
                self.add_new_block(&block);
                return Ok(BlockEvent::NewBlock(block));
            } else {
                let (old, mut new) = self.reorg(block.parent_hash()).await?;
                self.add_new_block(&block);
                info!("new block {}, height: {}", block.hash(), block.height());
                new.push(block);
                return Ok(BlockEvent::Reorg(old, new));
            }
        }

        if block.height() > last_height.height + 1 {
            let mut missing_blocks = get_blocks(
                &self.config,
                &self.chain_index,
                last_height.height + 1,
                block.height() - 1,
            )
            .await?;
            missing_blocks.push(block);
            Box::pin(self.handle_blocks(missing_blocks)).await?;
            return Ok(BlockEvent::Nothing);
        }

        let first_height = self.blocks.front().unwrap().height;
        if block.height() < first_height {
            info!(
                "ignore stale uncle block {:?}, height: {:?}",
                block.hash(),
                block.height()
            );
            return Ok(BlockEvent::Nothing);
        }

        let index = (block.height() - first_height) as usize;
        self.blocks[index].add(&block);
        info!(
            "new uncle block {}, height: {}",
            block.hash(),
            block.height()
        );
        Ok(BlockEvent::Nothing)
    }

    async fn handle_blocks(&mut self, blocks: Vec<BlockAndEvents>) -> Result<(), String> {
        for block in blocks {
            let result = self.handle_new_block(block).await?;
            self.handle_block_event(result).await?;
        }
        Ok(())
    }

    fn add_new_block(&mut self, block: &BlockAndEvents) {
        self.blocks.push_back(BlockAtHeight::from(&block));
        if self.blocks.len() > MAX_REORG_DEPTH {
            self.blocks.drain(0..(self.blocks.len() - MAX_REORG_DEPTH));
        }
    }

    async fn reorg(
        &mut self,
        from_hash: &str,
    ) -> Result<(Vec<BlockAndEvents>, Vec<BlockAndEvents>), String> {
        let mut hash = from_hash;
        let mut old: Vec<BlockAndEvents> = vec![];
        let mut new: Vec<BlockAndEvents> = vec![];
        for b in self.blocks.iter_mut().rev() {
            if b.mainchain_hash() == hash {
                break;
            }
            old.push(b.mainchain_block().clone());
            if !b.reorg(hash) {
                let block =
                    blockflow_api::get_blockflow_blocks_with_events_block_hash(&self.config, hash)
                        .await
                        .map_err(|e| format!("failed to request block due to {:?}", e))?;
                b.add_unsafe(block);
                b.reorg(hash);
            }
            new.push(b.mainchain_block().clone());
            hash = b.mainchain_block().parent_hash();
        }
        old.reverse();
        new.reverse();
        Ok((old, new))
    }
}

async fn get_blocks(
    config: &Configuration,
    chain_index: &ChainIndex,
    from_height: u32,
    to_height: u32,
) -> Result<Vec<BlockAndEvents>, String> {
    let mut blocks: Vec<BlockAndEvents> = vec![];
    for height in from_height..=to_height {
        let hashes = blockflow_api::get_blockflow_hashes(
            config,
            chain_index.from as i32,
            chain_index.to as i32,
            height as i32,
        )
        .await
        .map_err(|e| format!("failed to request block hashes due to {:?}", e))?;

        let block_hash = hashes.headers[0].as_str();
        let block = blockflow_api::get_blockflow_blocks_with_events_block_hash(config, block_hash)
            .await
            .map_err(|e| format!("failed to request block due to {:?}", e))?;

        blocks.push(block);
    }
    return Ok(blocks);
}

#[cfg(test)]
mod tests {
    use crate::handler::{ChainIndex, Handler};

    #[tokio::test]
    async fn test_subscribe_block() {
        let mut builder = env_logger::Builder::from_default_env();
        if std::env::var("RUST_LOG").is_err() {
            builder.filter_level(log::LevelFilter::Debug);
        }
        builder.is_test(true).init();
        let mut handler = Handler::from("127.0.0.1", 12973, ChainIndex { from: 0, to: 0 });
        handler
            .start(
                3622717,
                "00000000000020e1ac749d5bf7a68dce6925fdd323e9527a3c5259fd29721b60",
            )
            .await
            .unwrap();
    }
}
