use curve25519_dalek::{
    // constants::{RISTRETTO_BASEPOINT_COMPRESSED, RISTRETTO_BASEPOINT_POINT},
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
};
use wedpr_l_utils::error::WedprError;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

const RISTRETTO_POINT_SIZE_IN_BYTES: usize = 32;
// const SCALAR_SIZE_IN_BYTE: usize = 32;

fn to_bytes32_slice(barry: &[u8]) -> Result<&[u8; 32], WedprError> {
    let pop_u8 = match barry.try_into() {
        Ok(v) => v,
        Err(_) => return Err(WedprError::FormatError),
    };
    Ok(pop_u8)
}

pub fn bytes_to_scalar(input: &[u8]) -> Result<Scalar, WedprError> {
    let get_num_u8 = to_bytes32_slice(&input)?;
    let scalar_num = Scalar::from_bits(*get_num_u8);
    Ok(scalar_num)
}

pub fn bytes_to_point(point: &[u8]) -> Result<RistrettoPoint, WedprError> {
    if point.len() != RISTRETTO_POINT_SIZE_IN_BYTES {
        return Err(WedprError::FormatError);
    }
    let point_value = match CompressedRistretto::from_slice(&point).decompress()
    {
        Some(v) => v,
        None => {
            return Err(WedprError::FormatError);
        },
    };
    Ok(point_value)
}

pub fn point_to_bytes(point: &RistrettoPoint) -> Vec<u8> {
    point.compress().to_bytes().to_vec()
}

/// Converts Scalar to a vector.
pub fn scalar_to_bytes(input: &Scalar) -> Vec<u8> {
    input.as_bytes().to_vec()
}

#[wasm_bindgen]
pub fn wasm_scalar_multi(scalar_bytes: &[u8], point_bytes: &[u8]) -> Vec<u8> {
    let scalar = match bytes_to_scalar(scalar_bytes) {
        Ok(v) => v,
        Err(_) => return vec![],
    };

    let point = match bytes_to_point(point_bytes) {
        Ok(v) => v,
        Err(_) => return vec![],
    };
    point_to_bytes(&(scalar * point))
}

#[wasm_bindgen]
pub fn wasm_point_add(point1_bytes: &[u8], point2_bytes: &[u8]) -> Vec<u8> {
    let point1 = match bytes_to_point(point1_bytes) {
        Ok(v) => v,
        Err(_) => return vec![],
    };

    let point2 = match bytes_to_point(point2_bytes) {
        Ok(v) => v,
        Err(_) => return vec![],
    };
    point_to_bytes(&(point1 + point2))
}

#[wasm_bindgen]
pub fn wasm_scalar2_multi(scalar1_bytes: &[u8], scalar2_bytes: &[u8]) -> Vec<u8> {
    let scalar1 = match bytes_to_scalar(scalar1_bytes) {
        Ok(v) => v,
        Err(_) => return vec![],
    };

    let scalar2 = match bytes_to_scalar(scalar2_bytes) {
        Ok(v) => v,
        Err(_) => return vec![],
    };
    scalar_to_bytes(&(scalar1 * scalar2))
}

#[wasm_bindgen]
pub fn wasm_scalar_add(scalar1_bytes: &[u8], scalar2_bytes: &[u8]) -> Vec<u8> {
    let scalar1 = match bytes_to_scalar(scalar1_bytes) {
        Ok(v) => v,
        Err(_) => return vec![],
    };

    let scalar2 = match bytes_to_scalar(scalar2_bytes) {
        Ok(v) => v,
        Err(_) => return vec![],
    };
    scalar_to_bytes(&(scalar1 + scalar2))
}


