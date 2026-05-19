use crate::certificate::ReplayCertificate;

/// Verify a replay certificate. Returns Ok if the certificate
/// is self-consistent and passes hash verification.
pub fn verify_certificate(cert: &ReplayCertificate) -> Result<(), &'static str> {
    if !cert.verify() {
        return Err("certificate hash mismatch");
    }

    if cert.event_count < cert.transaction_count {
        return Err("event count less than transaction count");
    }

    if cert.start_position.epoch > cert.end_position.epoch {
        return Err("start after end");
    }

    if cert.start_position.epoch == cert.end_position.epoch
        && cert.start_position.sequence > cert.end_position.sequence
    {
        return Err("start sequence after end sequence");
    }

    Ok(())
}
