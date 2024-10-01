//! Contains traits for working with payload attributes and their providers.

use alloc::boxed::Box;
use alloy_eips::BlockNumHash;
use async_trait::async_trait;
use core::fmt::Debug;
use op_alloy_protocol::L2BlockInfo;
use op_alloy_rpc_types_engine::{OptimismAttributesWithParent, OptimismPayloadAttributes};

use crate::{
    batch::SingleBatch,
    errors::PipelineResult,
    traits::{OriginAdvancer, OriginProvider, PreviousStage, ResettableStage},
};

/// [NextAttributes] defines the interface for pulling attributes from
/// the top level `AttributesQueue` stage of the pipeline.
#[async_trait]
pub trait NextAttributes {
    /// Returns the next [OptimismAttributesWithParent] from the current batch.
    async fn next_attributes(
        &mut self,
        parent: L2BlockInfo,
    ) -> PipelineResult<OptimismAttributesWithParent>;
}

/// [AttributesQueuePrior] is a trait abstraction that generalizes the [BatchQueue] stage.
///
/// [BatchQueue]: crate::stages::BatchQueue
#[async_trait]
pub trait AttributesQueuePrior:
    PreviousStage + ResettableStage + OriginProvider + OriginAdvancer + Debug
{
    /// Returns the next valid batch upon the given safe head.
    async fn next_batch(&mut self, parent: L2BlockInfo) -> PipelineResult<SingleBatch>;

    /// Returns whether the current batch is the last in its span.
    fn is_last_in_span(&self) -> bool;
}

/// The [AttributesQueueBuilder] is responsible for preparing [OptimismPayloadAttributes]
/// that can be used to construct an L2 Block containing only deposits.
#[async_trait]
pub trait AttributesQueueBuilder: Debug {
    /// Prepares a template [OptimismPayloadAttributes] that is ready to be used to build an L2
    /// block. The block will contain deposits only, on top of the given L2 parent, with the L1
    /// origin set to the given epoch.
    /// By default, the [OptimismPayloadAttributes] template will have `no_tx_pool` set to true,
    /// and no sequencer transactions. The caller has to modify the template to add transactions.
    /// This can be done by either setting the `no_tx_pool` to false as sequencer, or by appending
    /// batch transactions as the verifier.
    async fn prepare_payload_attributes(
        &mut self,
        l2_parent: L2BlockInfo,
        epoch: BlockNumHash,
    ) -> PipelineResult<OptimismPayloadAttributes>;
}
