use sui_types::base_types::{ObjectID, TransactionDigest};
use sui_types::committee::Committee;
use sui_types::effects::{TransactionEffects, TransactionEffectsAPI, TransactionEvents};
use sui_types::full_checkpoint_content::CheckpointData;
use sui_types::messages_checkpoint::CheckpointSequenceNumber;
use sui_types::object::Object;

use anyhow::{anyhow,Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub checkpoint: CheckpointData,
    pub committee: Committee,
    pub tid: TransactionDigest,
}

impl Proof {
    /// Creates a new `Proof` instance from the provided checkpoint, committee, and transaction digest.
    ///
    /// # Examples
    ///
    /// ```
    /// let proof = Proof::new(checkpoint_data, committee, transaction_digest).unwrap();
    /// ```
    pub fn new(
        checkpoint: CheckpointData,
        committee: Committee,
        tid: TransactionDigest,
    ) -> Result<Self> {
        Ok(Self {
            checkpoint,
            committee,
            tid,
        })
    }

    /// Verifies the proof by validating the checkpoint summary and confirming the presence and correctness of the specified transaction.
    ///
    /// Checks that the checkpoint summary is valid for the given committee and contents, locates the transaction with the matching digest, and ensures the events digest matches the effects. Returns the transaction's effects and its optional events if verification succeeds.
    ///
    /// # Errors
    ///
    /// Returns an error if the checkpoint summary is invalid, the transaction is not found, or the events digest does not match.
    ///
    /// # Examples
    ///
    /// ```
    /// let proof = Proof::new(checkpoint, committee, tid).unwrap();
    /// let (effects, events) = proof.verify().unwrap();
    /// ```
    pub fn verify(&self) -> Result<(TransactionEffects, Option<TransactionEvents>)> {
        let checkpoint = &self.checkpoint;
        let committee = &self.committee;
        let tid = self.tid;

        let summary = &checkpoint.checkpoint_summary;

        // Verify the checkpoint summary using the committee
        summary.verify_with_contents(committee, Some(&checkpoint.checkpoint_contents))?;

        // Check the validity of the transaction
        let contents = &checkpoint.checkpoint_contents;
        let (matching_tx, _) = checkpoint
            .transactions
            .iter()
            .zip(contents.iter())
            // Note that we get the digest of the effects to ensure this is
            // indeed the correct effects that are authenticated in the contents.
            .find(|(tx, digest)| {
                tx.effects.execution_digests() == **digest && digest.transaction == tid
            })
            .ok_or(anyhow!("Transaction not found in checkpoint contents"))?;

        // Check the events are all correct.
        let events_digest = matching_tx.events.as_ref().map(|events| events.digest());
        anyhow::ensure!(
            events_digest.as_ref() == matching_tx.effects.events_digest(),
            "Events digest does not match"
        );

        // Since we do not check objects we do not return them
        Ok((matching_tx.effects.clone(), matching_tx.events.clone()))
    }

}