use std::path::PathBuf;
use sui_types::base_types::TransactionDigest;
use sui_types::committee::Committee;
use sui_types::effects::{TransactionEffects, TransactionEffectsAPI, TransactionEvents};
use sui_types::full_checkpoint_content::CheckpointData;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub checkpoint: CheckpointData,
    pub committee: Committee,
    pub tid: TransactionDigest,
}

impl Proof {
    pub fn new(
        checkpoint: CheckpointData,
        committee: Committee,
        tid: TransactionDigest,
    ) -> Self {
        Self {
            checkpoint,
            committee,
            tid,
        }
    }

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


#[test]
fn test_verify() {
    let proof_file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("proof.json");
    let proof: Proof = serde_json::from_reader(std::fs::File::open(proof_file).unwrap()).unwrap();
    proof.verify().unwrap();
}