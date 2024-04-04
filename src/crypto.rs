//!
//! Cryptography Related Keys for OpenThread
//!

use core::ops::BitOr;

/// Key types usable in OpenThread
pub enum OTKeyType {
    Raw,
    Aes,
    Hmac,
    Ecdsa,
}

/// Key algorithms usable in OpenThread
pub enum OTKeyAlgorithm {
    Vendor,
    AesEcb,
    HmacSha256,
    Ecsda,
}

/// OpenThread Key usage flags
pub enum OTKeyUsage {
    // Key usage is empty
    None = 0,
    // Key can be exported
    Export = 1 << 0,
    // Encryption (vendor defined)
    Encrypt = 1 << 1,
    // AES ECB.
    Decrypt = 1 << 2,
    // Sign Hash
    SignHash = 1 << 3,
    // Verify Hash
    VerifyHash = 1 << 4
}

impl BitOr for OTKeyUsage {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        (self as u8) | (rhs as u8)
    }
}

/// OpenThread Key Storage Types
pub enum OTKeyStorage {
    Volatile,
    Persistent,
}

/// Key Reference
pub type OTCryptoKeyRef = u32;


/// Key Material required for OpenThread Crypto Operations
pub enum OTCryptoKey<'a> {
    Key(Option<&'a [u8]>),
    Ref(OTCryptoKeyRef),
}

// TODO: OTCryptoContext

// Length of SHA256 hash (in bytes)
pub const OT_CRYPTO_SHA256_HASH_SIZE: usize = 32;

/// SHA256 Hash
pub type OTSha256Hash = [u8; OT_CRYPTO_SHA256_HASH_SIZE];

// Max Buffer size (in bytes) for representing the EDCSA key-pair in DER format.
pub const OT_CRYPTO_ECDSA_MAX_DER_SIZE: usize = 125;

/// ECDSA key pair (public nad private keys)
/// 
/// The key pair is stored using Distinguished Encoding Rules (DEF) format (per RFC 5915)
pub struct OTCryptoEcdsaKeyPair {
    pub der_bytes: [u8; OT_CRYPTO_ECDSA_MAX_DER_SIZE],
    pub der_length: usize,
}

/// Buffer size (in bytes) for representing the EDCSA public key
pub const OT_CRYPTO_ECDSA_PUBLIC_KEY_SIZE: usize = 64;

/// Represnets a ECDSA public key.
/// 
/// The public key is stored as a byte sequence representation of an uncompressed curve point (RFC 6605 - sec 4)
pub type OTCryptoEcdsaPublicKey = [u8; OT_CRYPTO_ECDSA_PUBLIC_KEY_SIZE];

/// Buffer size (in bytes) for representing the EDCSA signature
pub const OT_CRYPTO_ECDSA_SIGNATURE_SIZE: usize = 64;

/// Represents aan ECDSA signature
/// 
/// The signature is encoded as the concatenated binary representation of two MPIs 'r' and 's' which are calculated
/// during signing (RCF 6605 - section 4).
pub type OTCryptoEcdsaSignature = [u8; OT_CRYPTO_ECDSA_SIGNATURE_SIZE];

/// MAX PBKDF2 SALT length: salt prefix (6) + extended panid (8) + network name (16)
pub const OT_CRYPTO_PBDKF2_MAX_SALT_SIZE: usize = 30;
