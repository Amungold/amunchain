use ed25519_dalek::Signature;

#[derive(Debug, Clone, PartialEq)]
pub struct ConstitutionalSignature {
    pub(crate) sig: Signature,
    pub verifying_key_hex: String,
    pub algorithm: String,
    pub digest_algorithm: String,
    pub domain: String,
    pub schema_version: u32,
}

impl ConstitutionalSignature {
    pub fn new(sig: Signature, verifying_key_hex: String) -> Self {
        Self {
            sig,
            verifying_key_hex,
            algorithm: "Ed25519".into(),
            digest_algorithm: "BLAKE3".into(),
            domain: "AMUN_CONSTITUTIONAL_V1".into(),
            schema_version: 1,
        }
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.sig.to_bytes())
    }

    pub fn signature_bytes(&self) -> [u8; 64] {
        self.sig.to_bytes()
    }
}

// Manual Serialize / Deserialize using hex string for sig field
impl serde::Serialize for ConstitutionalSignature {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut st = s.serialize_struct("ConstitutionalSignature", 6)?;
        st.serialize_field("sig", &hex::encode(self.sig.to_bytes()))?;
        st.serialize_field("verifying_key_hex", &self.verifying_key_hex)?;
        st.serialize_field("algorithm", &self.algorithm)?;
        st.serialize_field("digest_algorithm", &self.digest_algorithm)?;
        st.serialize_field("domain", &self.domain)?;
        st.serialize_field("schema_version", &self.schema_version)?;
        st.end()
    }
}

impl<'de> serde::Deserialize<'de> for ConstitutionalSignature {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Helper {
            sig: String,
            verifying_key_hex: String,
            algorithm: String,
            digest_algorithm: String,
            domain: String,
            schema_version: u32,
        }
        let h = Helper::deserialize(d)?;
        let bytes = hex::decode(&h.sig).map_err(serde::de::Error::custom)?;
        let arr: [u8; 64] = bytes
            .try_into()
            .map_err(|_| serde::de::Error::custom("sig must be 64 bytes"))?;
        let sig = Signature::from_bytes(&arr);
        Ok(Self {
            sig,
            verifying_key_hex: h.verifying_key_hex,
            algorithm: h.algorithm,
            digest_algorithm: h.digest_algorithm,
            domain: h.domain,
            schema_version: h.schema_version,
        })
    }
}

impl ConstitutionalSignature {
    pub fn dalek_signature(&self) -> &ed25519_dalek::Signature {
        &self.sig
    }
}
