
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

// ─── Kyber (KEM) Imports ──────────────────────────────────────────────────────
use pqcrypto_kyber::kyber512::{
    keypair as kyber_keypair,
    encapsulate as kyber_encapsulate_impl,
    decapsulate as kyber_decapsulate_impl,
    PublicKey as KyberPublicKey,
    SecretKey as KyberSecretKey,
    Ciphertext as KyberCiphertext,
};
use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _, Ciphertext as _, SharedSecret as _};

// ─── Falcon (Signature) Imports ───────────────────────────────────────────────
use pqcrypto_falcon::falcon512::{
    keypair as falcon_keypair,
    sign as falcon_sign_impl,
    open as falcon_open_impl,
    PublicKey as FalconPublicKey,
    SecretKey as FalconSecretKey,
    SignedMessage as FalconSignedMessage,
};
use pqcrypto_traits::sign::{PublicKey as _, SecretKey as _, SignedMessage as _};

// ─── Kyber Exposed Functions ──────────────────────────────────────────────────
#[pyfunction]
fn kyber_keygen() -> PyResult<(Vec<u8>, Vec<u8>)> {
    let (pk, sk) = kyber_keypair();
    Ok((pk.as_bytes().to_vec(), sk.as_bytes().to_vec()))
}

#[pyfunction]
fn kyber_encapsulate(pk_bytes: Vec<u8>) -> PyResult<(Vec<u8>, Vec<u8>)> {
    let pk = KyberPublicKey::from_bytes(&pk_bytes)
        .map_err(|_| PyValueError::new_err("Invalid Kyber public key"))?;

    // ORDER IS (ss, ct) in pqcrypto-kyber >= 0.4
    let (ss, ct) = kyber_encapsulate_impl(&pk);

    // Return (ciphertext, shared-secret) to Python
    Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
}

#[pyfunction]
fn kyber_decapsulate(sk_bytes: Vec<u8>, ct_bytes: Vec<u8>) -> PyResult<Vec<u8>> {
    let sk = KyberSecretKey::from_bytes(&sk_bytes)
        .map_err(|_| PyValueError::new_err("Invalid Kyber secret key"))?;
    let ct = KyberCiphertext::from_bytes(&ct_bytes)
        .map_err(|_| PyValueError::new_err("Invalid Kyber ciphertext"))?;
    let ss = kyber_decapsulate_impl(&ct, &sk);
    Ok(ss.as_bytes().to_vec())
}

// ─── Falcon Exposed Functions ─────────────────────────────────────────────────
#[pyfunction]
fn falcon_keygen() -> PyResult<(Vec<u8>, Vec<u8>)> {
    let (pk, sk) = falcon_keypair();
    Ok((pk.as_bytes().to_vec(), sk.as_bytes().to_vec()))
}

#[pyfunction]
fn falcon_sign(sk_bytes: Vec<u8>, message: Vec<u8>) -> PyResult<Vec<u8>> {
    let sk = FalconSecretKey::from_bytes(&sk_bytes)
        .map_err(|_| PyValueError::new_err("Invalid Falcon secret key"))?;
    let sm = falcon_sign_impl(&message, &sk);
    Ok(sm.as_bytes().to_vec())
}

#[pyfunction]
fn falcon_verify(pk_bytes: Vec<u8>, sm_bytes: Vec<u8>, message: Vec<u8>) -> PyResult<bool> {
    let pk = FalconPublicKey::from_bytes(&pk_bytes)
        .map_err(|_| PyValueError::new_err("Invalid Falcon public key"))?;
    let sm = FalconSignedMessage::from_bytes(&sm_bytes)
        .map_err(|_| PyValueError::new_err("Invalid Falcon signed message"))?;
    match falcon_open_impl(&sm, &pk) {
        Ok(opened) => Ok(opened == message),
        Err(_) => Ok(false),
    }
}

// ─── PyO3 Module Registration ─────────────────────────────────────────────────
#[pymodule]
fn pqcrypto_bindings(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(kyber_keygen, m)?)?;
    m.add_function(wrap_pyfunction!(kyber_encapsulate, m)?)?;
    m.add_function(wrap_pyfunction!(kyber_decapsulate, m)?)?;
    m.add_function(wrap_pyfunction!(falcon_keygen, m)?)?;
    m.add_function(wrap_pyfunction!(falcon_sign, m)?)?;
    m.add_function(wrap_pyfunction!(falcon_verify, m)?)?;
    Ok(())
}
