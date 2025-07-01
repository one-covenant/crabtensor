use sp_core::{sr25519, Pair};
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    MultiSignature as SpMultiSignature,
};
use subxt::{
    config::substrate::{AccountId32, MultiSignature},
    tx::Signer,
};
use subxt::{Config, SubstrateConfig};
use thiserror::Error;

use crate::AccountId;

pub type KeypairSignature = sr25519::Signature;

#[derive(Debug, Error)]
pub enum SigningError {
    #[error("Unsupported signature type, expected Sr25519")]
    UnsupportedSignature,
}

/// A [`Signer`] implementation for [`sp_core::sr25519::Pair`].
///
/// Based on https://github.com/paritytech/subxt/blob/master/subxt/examples/substrate_compat_signer.rs
#[derive(Clone)]
pub struct PairSigner {
    account_id: <SubstrateConfig as Config>::AccountId,
    signer: sr25519::Pair,
}

impl PairSigner {
    /// Creates a new [`Signer`] from an [`sp_core::sr25519::Pair`].
    pub fn new(signer: sr25519::Pair) -> Self {
        let account_id = <SpMultiSignature as Verify>::Signer::from(signer.public()).into_account();
        Self {
            // Convert `sp_core::AccountId32` to `subxt::config::substrate::AccountId32`.
            //
            // This is necessary because we use `subxt::config::substrate::AccountId32` and no
            // From/Into impls are provided between `sp_core::AccountId32` because `polkadot-sdk` isn't a direct
            // dependency in subxt.
            //
            // This can also be done by provided a wrapper type around `subxt::config::substrate::AccountId32` to implement
            // such conversions but that also most likely requires a custom `Config` with a separate `AccountId` type to work
            // properly without additional hacks.
            account_id: AccountId32(account_id.into()),
            signer,
        }
    }

    /// Returns the [`sp_core::sr25519::Pair`] implementation used to construct this.
    pub fn signer(&self) -> &sr25519::Pair {
        &self.signer
    }

    /// Return the account ID.
    pub fn account_id(&self) -> &AccountId32 {
        &self.account_id
    }
}

impl Signer<SubstrateConfig> for PairSigner {
    fn account_id(&self) -> <SubstrateConfig as Config>::AccountId {
        self.account_id.clone()
    }

    fn sign(&self, signer_payload: &[u8]) -> <SubstrateConfig as Config>::Signature {
        let signature = self.signer.sign(signer_payload);
        MultiSignature::Sr25519(signature.0)
    }
}

pub fn verify_signature(
    account_id: &AccountId,
    signature: &KeypairSignature,
    message: impl AsRef<[u8]>,
) -> bool {
    sr25519::Pair::verify(signature, message, &sr25519::Public::from_raw(account_id.0))
}

pub fn sign_message(
    signer: &PairSigner,
    message: impl AsRef<[u8]>,
) -> Result<KeypairSignature, SigningError> {
    match signer.sign(message.as_ref()) {
        MultiSignature::Sr25519(s) => Ok(s.into()),
        _ => Err(SigningError::UnsupportedSignature),
    }
}
