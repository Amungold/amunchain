use std::slice;
use amun_crypto::Ed25519Signer;

const ERR_OK: i32 = 0;
const ERR_NULL_PTR: i32 = -1;
const ERR_INVALID_LENGTH: i32 = -2;
const ERR_SIGN_FAILED: i32 = -3;
const ERR_VERIFY_FAILED: i32 = -4;

#[no_mangle]
pub extern "C" fn amun_generate_keypair(
    public_key_out: *mut u8,
    secret_key_out: *mut u8,
) -> i32 {
    if public_key_out.is_null() || secret_key_out.is_null() {
        return ERR_NULL_PTR;
    }
    let signer = Ed25519Signer::generate();
    unsafe {
        slice::from_raw_parts_mut(public_key_out, 32)
            .copy_from_slice(&signer.public_bytes());
        slice::from_raw_parts_mut(secret_key_out, 32)
            .copy_from_slice(&signer.to_bytes());
    }
    ERR_OK
}

#[no_mangle]
pub extern "C" fn amun_sign(
    secret_key: *const u8,
    message: *const u8,
    message_len: u32,
    signature_out: *mut u8,
) -> i32 {
    if secret_key.is_null() || message.is_null() || signature_out.is_null() {
        return ERR_NULL_PTR;
    }
    if message_len > 65536 {
        return ERR_INVALID_LENGTH;
    }

    let seed = unsafe {
        let sk = slice::from_raw_parts(secret_key, 32);
        if sk.len() != 32 {
            return ERR_INVALID_LENGTH;
        }
        let mut seed = [0u8; 32];
        seed.copy_from_slice(sk);
        seed
    };

    let msg = unsafe { slice::from_raw_parts(message, message_len as usize) };
    let signer = Ed25519Signer::from_seed(&seed);

    match signer.sign(msg, b"AMUN_FFI", 1) {
        Ok(sig) => {
            unsafe {
                slice::from_raw_parts_mut(signature_out, 64).copy_from_slice(&sig);
            }
            ERR_OK
        }
        Err(_) => ERR_SIGN_FAILED,
    }
}

#[no_mangle]
pub extern "C" fn amun_verify(
    public_key: *const u8,
    message: *const u8,
    message_len: u32,
    signature: *const u8,
) -> i32 {
    if public_key.is_null() || message.is_null() || signature.is_null() {
        return ERR_NULL_PTR;
    }
    if message_len > 65536 {
        return ERR_INVALID_LENGTH;
    }

    let pk = unsafe {
        let pk_slice = slice::from_raw_parts(public_key, 32);
        if pk_slice.len() != 32 {
            return ERR_INVALID_LENGTH;
        }
        let mut pk = [0u8; 32];
        pk.copy_from_slice(pk_slice);
        pk
    };

    let msg = unsafe { slice::from_raw_parts(message, message_len as usize) };

    let sig = unsafe {
        let sig_slice = slice::from_raw_parts(signature, 64);
        if sig_slice.len() != 64 {
            return ERR_INVALID_LENGTH;
        }
        let mut sig = [0u8; 64];
        sig.copy_from_slice(sig_slice);
        sig
    };

    match Ed25519Signer::verify(&pk, msg, &sig, b"AMUN_FFI", 1) {
        Ok(()) => ERR_OK,
        Err(_) => ERR_VERIFY_FAILED,
    }
}
