// Copyright 2017 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement.  This, along with the Licenses can be
// found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

//! This module contains common FFI types and structures used by both authenticator and apps.

#![allow(unsafe_code)]

use errors::CoreError;
use ffi_utils::ReprC;
use routing::XOR_NAME_LEN;
use rust_sodium::crypto::{box_, secretbox, sign};

/// Array containing public key bytes
pub type AsymPublicKey = [u8; box_::PUBLICKEYBYTES];
/// Array containing private key bytes
pub type AsymSecretKey = [u8; box_::SECRETKEYBYTES];
/// Array containing nonce bytes
pub type AsymNonce = [u8; box_::NONCEBYTES];

/// Array containing private key bytes
pub type SymSecretKey = [u8; secretbox::KEYBYTES];
/// Array containing nonce bytes
pub type SymNonce = [u8; secretbox::NONCEBYTES];

/// Array containing sign public key bytes
pub type SignPublicKey = [u8; sign::PUBLICKEYBYTES];
/// Array containing sign private key bytes
pub type SignSecretKey = [u8; sign::SECRETKEYBYTES];

/// Array containing `XorName` bytes
pub type XorNameArray = [u8; XOR_NAME_LEN];

/// Represents the FFI-safe account info
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AccountInfo {
    /// Number of used mutations
    pub mutations_done: u64,
    /// Number of available mutations
    pub mutations_available: u64,
}

impl ReprC for AccountInfo {
    type C = *const AccountInfo;
    type Error = CoreError;

    unsafe fn clone_from_repr_c(repr_c: Self::C) -> Result<Self, Self::Error> {
        Ok(*repr_c)
    }
}

/// FFI wrapper for `MDataInfo`
#[repr(C)]
#[derive(Clone)]
pub struct MDataInfo {
    /// Name of the mutable data.
    pub name: XorNameArray,
    /// Type tag of the mutable data.
    pub type_tag: u64,

    /// Flag indicating whether the encryption info (`enc_key` and `enc_nonce`)
    /// is set.
    pub has_enc_info: bool,
    /// Encryption key. Meaningful only if `has_enc_info` is `true`.
    pub enc_key: SymSecretKey,
    /// Encryption nonce. Meaningful only if `has_enc_info` is `true`.
    pub enc_nonce: SymNonce,

    /// Flag indicating whether the new encryption info is set.
    pub has_new_enc_info: bool,
    /// New encryption key (used for two-phase reencryption). Meaningful only if
    /// `has_new_enc_info` is `true`.
    pub new_enc_key: SymSecretKey,
    /// New encryption nonce (used for two-phase reencryption). Meaningful only if
    /// `has_new_enc_info` is `true`.
    pub new_enc_nonce: SymNonce,
}
