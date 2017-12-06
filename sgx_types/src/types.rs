// Copyright (c) 2017 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use error::*;
use marker::ContiguousMemory;

//
// sgx_attributes.h
//

pub type sgx_misc_select_t = ::uint32_t;

// Enclave Flags Bit Masks
pub const SGX_FLAGS_INITTED: ::uint64_t         = 0x0000000000000001;    //If set, then the enclave is initialized
pub const SGX_FLAGS_DEBUG: ::uint64_t           = 0x0000000000000002;    //If set, then the enclave is debug
pub const SGX_FLAGS_MODE64BIT: ::uint64_t       = 0x0000000000000004;    //If set, then the enclave is 64 bit
pub const SGX_FLAGS_PROVISION_KEY: ::uint64_t   = 0x0000000000000010;    //If set, then the enclave has access to provision key
pub const SGX_FLAGS_EINITTOKEN_KEY: ::uint64_t  = 0x0000000000000020;    //If set, then the enclave has access to EINITTOKEN key
pub const SGX_FLAGS_RESERVED: ::uint64_t        = (!(SGX_FLAGS_INITTED
                                                | SGX_FLAGS_DEBUG
                                                | SGX_FLAGS_MODE64BIT
                                                | SGX_FLAGS_PROVISION_KEY
                                                | SGX_FLAGS_EINITTOKEN_KEY));

// XSAVE Feature Request Mask
pub const SGX_XFRM_LEGACY: ::uint64_t           = 0x0000000000000003;  //Legacy XFRM
pub const SGX_XFRM_AVX: ::uint64_t              = 0x0000000000000006;  // AVX
pub const SGX_XFRM_AVX512: ::uint64_t           = 0x00000000000000E6;  // AVX-512 - not supported
pub const SGX_XFRM_MPX: ::uint64_t              = 0x0000000000000018;  // MPX - not supported

pub const SGX_XFRM_RESERVED: ::uint64_t         = (!(SGX_XFRM_LEGACY | SGX_XFRM_AVX));

impl_struct! {
    pub struct sgx_attributes_t {
        pub flags: ::uint64_t,
        pub xfrm: ::uint64_t,
    }

    pub struct sgx_misc_attribute_t {
        pub secs_attr: sgx_attributes_t,
        pub misc_select: sgx_misc_select_t,
    }
}

//
// sgx_dh.h
//

pub const SGX_DH_MAC_SIZE: ::size_t           = 16;
pub const SGX_DH_SESSION_DATA_SIZE: ::size_t  = 200;

impl_struct! {

    #[repr(packed)]
    pub struct sgx_dh_msg1_t {
        pub g_a: sgx_ec256_public_t,
        pub target: sgx_target_info_t,
    }
}

impl_copy_clone! {

    #[repr(packed)]
    pub struct sgx_dh_msg2_t {
        pub g_b: sgx_ec256_public_t,
        pub report: sgx_report_t,
        pub cmac: [::uint8_t; SGX_DH_MAC_SIZE],
    }

    #[repr(packed)]
    pub struct sgx_dh_msg3_body_t {
        pub report: sgx_report_t,
        pub additional_prop_length: ::uint32_t,
        pub additional_prop: [::uint8_t; 0],
    }

    #[repr(packed)]
    pub struct sgx_dh_msg3_t {
        pub cmac: [::uint8_t; SGX_DH_MAC_SIZE],
        pub msg3_body: sgx_dh_msg3_body_t,
    }

    #[repr(packed)]
    pub struct sgx_dh_session_enclave_identity_t {
        pub cpu_svn: sgx_cpu_svn_t,
        pub misc_select: ::sgx_misc_select_t,
        pub reserved_1: [::uint8_t; 28],
        pub attributes: sgx_attributes_t,
        pub mr_enclave: sgx_measurement_t,
        pub reserved_2: [::uint8_t; 32],
        pub mr_signer: sgx_measurement_t,
        pub reserved_3: [::uint8_t; 96],
        pub isv_prod_id: ::sgx_prod_id_t,
        pub isv_svn: ::sgx_isv_svn_t,
    }

    #[repr(packed)]
    pub struct sgx_dh_session_t {
        pub sgx_dh_session: [::uint8_t; SGX_DH_SESSION_DATA_SIZE],
    }
}

impl_struct_default! {
    sgx_dh_msg2_t, 512;
    sgx_dh_msg3_body_t, 436;
    sgx_dh_msg3_t, 452;
    sgx_dh_session_enclave_identity_t, 260;
    sgx_dh_session_t, 200;
}

impl_struct_ContiguousMemory! {
    sgx_dh_msg2_t;
    sgx_dh_msg3_body_t;
    sgx_dh_msg3_t;
    sgx_dh_session_enclave_identity_t;
    sgx_dh_session_t;
}

impl_enum! {

    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum sgx_dh_session_role_t {
        SGX_DH_SESSION_INITIATOR = 0,
        SGX_DH_SESSION_RESPONDER = 1,
    }
}

//
// sgx_ecp_types.h
//


pub const SGX_FEBITSIZE: ::uint32_t = 256;

impl_struct!{

    #[repr(packed)]
    pub struct ecc_param_t {
        pub eccP: [::uint32_t; SGX_NISTP_ECP256_KEY_SIZE],      /* EC prime field */
        pub eccA: [::uint32_t; SGX_NISTP_ECP256_KEY_SIZE],      /* EC curve coefficient A */
        pub eccB: [::uint32_t; SGX_NISTP_ECP256_KEY_SIZE],      /* EC curve coefficient B */
        pub eccG: [[::uint32_t; SGX_NISTP_ECP256_KEY_SIZE]; 2], /* ECC base point */
        pub eccR: [::uint32_t; SGX_NISTP_ECP256_KEY_SIZE],      /* ECC base point order */
    }
}

pub type sgx_ec_key_128bit_t = [::uint8_t; SGX_CMAC_KEY_SIZE];

//
// sgx_eid.h
//


pub type sgx_enclave_id_t = ::uint64_t;

//
// sgx_key.h
//


// Key Name
pub const SGX_KEYSELECT_LICENSE: ::uint16_t          = 0x0000;
pub const SGX_KEYSELECT_PROVISION: ::uint16_t        = 0x0001;
pub const SGX_KEYSELECT_PROVISION_SEAL: ::uint16_t   = 0x0002;
pub const SGX_KEYSELECT_REPORT: ::uint16_t           = 0x0003;
pub const SGX_KEYSELECT_SEAL: ::uint16_t             = 0x0004;

// Key Policy
pub const SGX_KEYPOLICY_MRENCLAVE: ::uint16_t        = 0x0001;      /* Derive key using the enclave's ENCLAVE measurement register */
pub const SGX_KEYPOLICY_MRSIGNER: ::uint16_t         = 0x0002;      /* Derive key using the enclave's SINGER measurement register */

pub const SGX_KEYID_SIZE: ::size_t                    = 32;
pub const SGX_CPUSVN_SIZE: ::size_t                   = 16;
pub const SGX_KEY_REQUEST_RESERVED2_BYTES: ::size_t   = 436;

pub type sgx_key_128bit_t = [::uint8_t; 16];
pub type sgx_isv_svn_t = ::uint16_t;

impl_struct! {

    pub struct sgx_cpu_svn_t {
        pub svn: [::uint8_t; SGX_CPUSVN_SIZE],
    }

    pub struct sgx_key_id_t {
        pub id: [::uint8_t; SGX_KEYID_SIZE],
    }
}

impl_copy_clone! {

    pub struct sgx_key_request_t {
        pub key_name: ::uint16_t,
        pub key_policy: ::uint16_t,
        pub isv_svn: sgx_isv_svn_t,
        pub reserved1: ::uint16_t,
        pub cpu_svn: sgx_cpu_svn_t,
        pub attribute_mask: sgx_attributes_t,
        pub key_id: sgx_key_id_t,
        pub misc_mask: sgx_misc_select_t,
        pub reserved2: [::uint8_t; SGX_KEY_REQUEST_RESERVED2_BYTES],
    }
}

impl_struct_default! {
    sgx_key_request_t, 512;
}

impl_struct_ContiguousMemory! {
    sgx_key_request_t;
}

//
// sgx_key_exchange.h
//


pub type sgx_ra_context_t = ::uint32_t;
pub type sgx_ra_key_128_t = sgx_key_128bit_t;

impl_enum! {

    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum sgx_ra_key_type_t {
        SGX_RA_KEY_SK = 1,
        SGX_RA_KEY_MK = 2,
        SGX_RA_KEY_VK = 3,
    }
}

impl_struct! {

    pub struct sgx_ra_msg1_t {
        pub g_a: sgx_ec256_public_t,
        pub gid: sgx_epid_group_id_t,
    }

    pub struct sgx_ra_msg2_t {
        pub g_b: sgx_ec256_public_t,
        pub spid: sgx_spid_t,
        pub quote_type: ::uint16_t,
        pub kdf_id: ::uint16_t,
        pub sign_gb_ga: sgx_ec256_signature_t,
        pub mac: sgx_mac_t,
        pub sig_rl_size: ::uint32_t,
        pub sig_rl: [::uint8_t; 0],
    }
}

impl_copy_clone! {

    pub struct sgx_ra_msg3_t {
        pub mac: sgx_mac_t,
        pub g_a: sgx_ec256_public_t,
        pub ps_sec_prop: sgx_ps_sec_prop_desc_t,
        pub quote: [::uint8_t; 0],
    }
}

impl_struct_default! {
    sgx_ra_msg3_t, 336;
}

impl_struct_ContiguousMemory! {
    sgx_ra_msg3_t;
}

//
// sgx_quote.h
//


pub type sgx_epid_group_id_t = [::uint8_t; 4];
pub const SGX_PLATFORM_INFO_SIZE: ::size_t = 101;

impl_struct! {

    #[repr(packed)]
    pub struct sgx_spid_t {
        pub id: [::uint8_t ; 16],
    }

    #[repr(packed)]
    pub struct sgx_basename_t {
        pub name: [::uint8_t ; 32],
    }

    #[repr(packed)]
    pub struct sgx_quote_nonce_t {
        pub rand: [::uint8_t ; 16],
    }

    #[repr(packed)]
    pub struct sgx_update_info_bit_t {
        pub ucodeUpdate: ::int32_t,
        pub csmeFwUpdate: ::int32_t,
        pub pswUpdate: ::int32_t,
    }
}

impl_enum! {

    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum sgx_quote_sign_type_t {
        SGX_UNLINKABLE_SIGNATURE    = 0,
        SGX_LINKABLE_SIGNATURE      = 1,
    }
}

impl_copy_clone! {

    #[repr(packed)]
    pub struct sgx_quote_t {
        pub version: ::uint16_t,                    /* 0   */
        pub sign_type: ::uint16_t,                  /* 2   */
        pub epid_group_id: sgx_epid_group_id_t,     /* 4   */
        pub qe_svn: sgx_isv_svn_t,                  /* 8   */
        pub pce_svn: sgx_isv_svn_t,                 /* 10  */
        pub xeid: ::uint32_t,                       /* 12  */
        pub basename: sgx_basename_t,               /* 16  */
        pub report_body: sgx_report_body_t,         /* 48  */
        pub signature_len: ::uint32_t,              /* 432 */
        pub signature: [::uint8_t; 0],              /* 436 */
    }

    #[repr(packed)]
    pub struct sgx_platform_info_t {
        pub platform_info: [::uint8_t; SGX_PLATFORM_INFO_SIZE],
    }
}

impl_struct_default! {
    sgx_quote_t, 436;
    sgx_platform_info_t, 101;
}

impl_struct_ContiguousMemory! {
    sgx_quote_t;
    sgx_platform_info_t;
}

//
// sgx_report.h
//


pub const SGX_HASH_SIZE: ::size_t   = 32;
pub const SGX_MAC_SIZE: ::size_t    = 16;

pub const SGX_REPORT_DATA_SIZE: ::size_t   = 64;

impl_struct! {

    pub struct sgx_measurement_t {
        pub m: [::uint8_t; SGX_HASH_SIZE],
    }
}

pub type sgx_mac_t = [::uint8_t; SGX_MAC_SIZE];

impl_copy_clone! {

    pub struct sgx_report_data_t {
        pub d: [::uint8_t; SGX_REPORT_DATA_SIZE],
    }
}

impl_struct_default! {
    sgx_report_data_t, 64;
}

impl_struct_ContiguousMemory! {
    sgx_report_data_t;
}

pub type sgx_prod_id_t = ::uint16_t;

pub const SGX_TARGET_INFO_RESERVED1_BYTES: ::size_t = 4;
pub const SGX_TARGET_INFO_RESERVED2_BYTES: ::size_t = 456;

impl_copy_clone! {

    pub struct sgx_target_info_t {
        pub mr_enclave: sgx_measurement_t,
        pub attributes: sgx_attributes_t,
        pub reserved1: [::uint8_t; SGX_TARGET_INFO_RESERVED1_BYTES],
        pub misc_select: sgx_misc_select_t,
        pub reserved2: [::uint8_t; SGX_TARGET_INFO_RESERVED2_BYTES],
    }

    pub struct sgx_report_body_t {
        pub cpu_svn: sgx_cpu_svn_t,
        pub misc_select: sgx_misc_select_t,
        pub reserved1: [::uint8_t; 28],
        pub attributes: sgx_attributes_t,
        pub mr_enclave: sgx_measurement_t,
        pub reserved2: [::uint8_t; 32],
        pub mr_signer: sgx_measurement_t,
        pub reserved3: [::uint8_t; 96],
        pub isv_prod_id: sgx_prod_id_t,
        pub isv_svn: sgx_isv_svn_t,
        pub reserved4: [::uint8_t; 60],
        pub report_data: sgx_report_data_t,
    }

    pub struct sgx_report_t {
        pub body: sgx_report_body_t,
        pub key_id: sgx_key_id_t,
        pub mac: sgx_mac_t,
    }
}

impl_struct_default! {
    sgx_target_info_t, 512;
    sgx_report_body_t, 384;
    sgx_report_t, 432;
}

impl_struct_ContiguousMemory! {
    sgx_target_info_t;
    sgx_report_body_t;
    sgx_report_t;
}

//
// sgx_spinlock.h
//

// typedef volatile uint32_t sgx_spinlock_t;
pub type sgx_spinlock_t = ::uint32_t;

pub const SGX_SPINLOCK_INITIALIZER: ::uint32_t    = 0;

//
// sgx_tae_service.h
//

pub type sgx_time_t = ::uint64_t;

pub type sgx_time_source_nonce_t = [::uint8_t; 32];

pub const SGX_MC_UUID_COUNTER_ID_SIZE: ::size_t    = 3;
pub const SGX_MC_UUID_NONCE_SIZE: ::size_t         = 13;

impl_struct! {

    #[repr(packed)]
    pub struct sgx_mc_uuid_t {
        pub counter_id: [::uint8_t; SGX_MC_UUID_COUNTER_ID_SIZE],
        pub nonce: [::uint8_t; SGX_MC_UUID_NONCE_SIZE],
    }
}

impl_copy_clone! {

    #[repr(packed)]
    pub struct sgx_ps_sec_prop_desc_t {
        pub sgx_ps_sec_prop_desc: [::uint8_t; 256],
    }

    pub struct sgx_ps_sec_prop_desc_ex_t {
        pub ps_sec_prop_desc: sgx_ps_sec_prop_desc_t,
        pub pse_mrsigner: sgx_measurement_t,
        pub pse_prod_id: sgx_prod_id_t,
        pub pse_isv_svn: sgx_isv_svn_t,
    }
}

impl_struct_default! {
    sgx_ps_sec_prop_desc_t, 256;
    sgx_ps_sec_prop_desc_ex_t, 292;
}

impl_struct_ContiguousMemory! {
    sgx_ps_sec_prop_desc_t;
    sgx_ps_sec_prop_desc_ex_t;
}

pub const SGX_MC_POLICY_SIGNER: ::uint16_t   = 0x01;
pub const SGX_MC_POLICY_ENCLAVE: ::uint16_t  = 0x02;

//
// sgx_tcrypto.h
//


pub const SGX_SHA256_HASH_SIZE: ::size_t       = 32;
pub const SGX_ECP256_KEY_SIZE: ::size_t        = 32;
pub const SGX_NISTP_ECP256_KEY_SIZE: ::size_t  = (SGX_ECP256_KEY_SIZE / 4);
pub const SGX_AESGCM_IV_SIZE: ::size_t         = 12;
pub const SGX_AESGCM_KEY_SIZE: ::size_t        = 16;
pub const SGX_AESGCM_MAC_SIZE: ::size_t        = 16;
pub const SGX_CMAC_KEY_SIZE: ::size_t          = 16;
pub const SGX_CMAC_MAC_SIZE: ::size_t          = 16;
pub const SGX_AESCTR_KEY_SIZE: ::size_t        = 16;
pub const SGX_RSA3072_KEY_SIZE: ::size_t       = 384;
pub const SGX_RSA3072_PRI_EXP_SIZE: ::size_t   = 384;
pub const SGX_RSA3072_PUB_EXP_SIZE: ::size_t   = 4;

impl_struct! {

    pub struct sgx_ec256_dh_shared_t {
        pub s: [::uint8_t; SGX_ECP256_KEY_SIZE],
    }

    pub struct sgx_ec256_dh_shared512_t {
        pub x: [::uint8_t; SGX_ECP256_KEY_SIZE],
        pub y: [::uint8_t; SGX_ECP256_KEY_SIZE],
    }

    pub struct sgx_ec256_private_t {
        pub r: [::uint8_t; SGX_ECP256_KEY_SIZE],
    }

    pub struct sgx_ec256_public_t {
        pub gx: [::uint8_t; SGX_ECP256_KEY_SIZE],
        pub gy: [::uint8_t; SGX_ECP256_KEY_SIZE],
    }

    pub struct sgx_ec256_signature_t {
        pub x: [::uint32_t; SGX_NISTP_ECP256_KEY_SIZE],
        pub y: [::uint32_t; SGX_NISTP_ECP256_KEY_SIZE],
    }
}

impl_copy_clone! {

    pub struct sgx_rsa3072_public_key_t {
        pub modulus: [::uint8_t; SGX_RSA3072_KEY_SIZE],
        pub exponent: [::uint8_t; SGX_RSA3072_PUB_EXP_SIZE],
    }

    pub struct sgx_rsa3072_private_key_t {
        pub modulus: [::uint8_t; SGX_RSA3072_KEY_SIZE],
        pub exponent: [::uint8_t; SGX_RSA3072_PRI_EXP_SIZE],
    }

    pub struct sgx_rsa3072_signature_t {
        pub signature: [::uint8_t; SGX_RSA3072_KEY_SIZE],
    }
}

impl_struct_default! {
    sgx_rsa3072_public_key_t, 388;
    sgx_rsa3072_private_key_t, 768;
    sgx_rsa3072_signature_t, 384;
}

impl_struct_ContiguousMemory! {
    sgx_rsa3072_public_key_t;
    sgx_rsa3072_private_key_t;
    sgx_rsa3072_signature_t;
}

//pub type sgx_rsa3072_signature_t    = [::uint8_t; SGX_RSA3072_KEY_SIZE];

pub type sgx_sha_state_handle_t     = * mut ::c_void;
pub type sgx_cmac_state_handle_t    = * mut ::c_void;
pub type sgx_ecc_state_handle_t     = * mut ::c_void;

pub type sgx_sha256_hash_t = [::uint8_t; SGX_SHA256_HASH_SIZE];

pub type sgx_aes_gcm_128bit_key_t   = [::uint8_t; SGX_AESGCM_KEY_SIZE];
pub type sgx_aes_gcm_128bit_tag_t   = [::uint8_t; SGX_AESGCM_MAC_SIZE];
pub type sgx_cmac_128bit_key_t      = [::uint8_t; SGX_CMAC_KEY_SIZE];
pub type sgx_cmac_128bit_tag_t      = [::uint8_t; SGX_CMAC_MAC_SIZE];
pub type sgx_aes_ctr_128bit_key_t   = [::uint8_t; SGX_AESCTR_KEY_SIZE];

impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum sgx_generic_ecresult_t {
        SGX_EC_VALID                = 0x00000000,   /* validation pass successfully     */

        SGX_EC_COMPOSITE_BASE       = 0x00000001,   /* field based on composite         */
        SGX_EC_COMPLICATED_BASE     = 0x00000002,   /* number of non-zero terms in the polynomial (> PRIME_ARR_MAX) */
        SGX_EC_IS_ZERO_DISCRIMINANT = 0x00000003,   /* zero discriminant */
        SGX_EC_COMPOSITE_ORDER      = 0x00000004,   /* composite order of base point    */
        SGX_EC_INVALID_ORDER        = 0x00000005,   /* invalid base point order         */
        SGX_EC_IS_WEAK_MOV          = 0x00000006,   /* weak Meneze-Okamoto-Vanstone  reduction attack */
        SGX_EC_IS_WEAK_SSA          = 0x00000007,   /* weak Semaev-Smart,Satoh-Araki reduction attack */
        SGX_EC_IS_SUPER_SINGULAR    = 0x00000008,   /* supersingular curve */

        SGX_EC_INVALID_PRIVATE_KEY  = 0x00000009,   /* !(0 < Private < order) */
        SGX_EC_INVALID_PUBLIC_KEY   = 0x0000000a,   /* (order*PublicKey != Infinity)    */
        SGX_EC_INVALID_KEY_PAIR     = 0x0000000b,   /* (Private*BasePoint != PublicKey) */

        SGX_EC_POINT_OUT_OF_GROUP   = 0x0000000c,   /* out of group (order*P != Infinity)  */
        SGX_EC_POINT_IS_AT_INFINITY = 0x0000000d,   /* point (P=(Px,Py)) at Infinity  */
        SGX_EC_POINT_IS_NOT_VALID   = 0x0000000e,   /* point (P=(Px,Py)) out-of EC    */

        SGX_EC_POINT_IS_EQUAL       = 0x0000000f,   /* compared points are equal     */
        SGX_EC_POINT_IS_NOT_EQUAL   = 0x00000010,   /* compared points are different  */

        SGX_EC_INVALID_SIGNATURE    = 0x00000011,   /* invalid signature */
    }
}

impl_enum! {
    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum sgx_rsa_result_t {
        SGX_RSA_VALID               = 0,   /* validation pass successfully */
        SGX_RSA_INVALID_SIGNATURE   = 1,   /* invalid signature */
    }
}

//
// sgx_thread.h
//


pub type sgx_thread_t = ::uintptr_t;

cfg_if! {
    if #[cfg(target_arch = "x86")] {
        pub const SE_WORDSIZE: ::size_t = 4;
    } else {
        pub const SE_WORDSIZE: ::size_t = 8;
    }
}

//pub const THREAD_SELF_ADDR:         ::size_t = 0;
//pub const THREAD_LAST_SP_ADDR:      ::size_t = (SE_WORDSIZE * 1);
//pub const THREAD_STACK_BASE_ADDR:   ::size_t = (SE_WORDSIZE * 2);
//pub const THREAD_STACK_LIMIT_ADDR:  ::size_t = (SE_WORDSIZE * 3);
//pub const THREAD_STACK_SSA_GPR:     ::size_t = (SE_WORDSIZE * 4);

#[repr(C)]
pub struct sgx_thread_queue_t {
    pub m_first: sgx_thread_t,
    pub m_last: sgx_thread_t,
}

#[repr(C)]
pub struct sgx_thread_mutex_t {
    pub m_refcount: ::size_t,
    pub m_control: ::uint32_t,
    pub m_lock: ::uint32_t,
    pub m_owner: sgx_thread_t,
    pub m_queue: sgx_thread_queue_t,
}

pub const SGX_THREAD_T_NULL: sgx_thread_t   = 0 ;

pub const SGX_THREAD_MUTEX_NONRECURSIVE: ::uint32_t = 0x01;
pub const SGX_THREAD_MUTEX_RECURSIVE: ::uint32_t    = 0x02;

pub const SGX_THREAD_NONRECURSIVE_MUTEX_INITIALIZER: sgx_thread_mutex_t = sgx_thread_mutex_t {
    m_refcount: 0,
    m_control: SGX_THREAD_MUTEX_NONRECURSIVE,
    m_lock: 0,
    m_owner: SGX_THREAD_T_NULL,
    m_queue: sgx_thread_queue_t {
        m_first: SGX_THREAD_T_NULL,
        m_last: SGX_THREAD_T_NULL
        }
    };

pub const SGX_THREAD_RECURSIVE_MUTEX_INITIALIZER: sgx_thread_mutex_t = sgx_thread_mutex_t {
    m_refcount: 0,
    m_control: SGX_THREAD_MUTEX_RECURSIVE,
    m_lock: 0,
    m_owner: SGX_THREAD_T_NULL,
    m_queue: sgx_thread_queue_t {
        m_first: SGX_THREAD_T_NULL,
        m_last: SGX_THREAD_T_NULL
        }
    };

pub const SGX_THREAD_MUTEX_INITIALIZER: sgx_thread_mutex_t = SGX_THREAD_NONRECURSIVE_MUTEX_INITIALIZER;

impl_struct! {

    pub struct sgx_thread_mutexattr_t {
        pub m_dummy: ::c_uchar,
    }

    pub struct sgx_thread_condattr_t {
        pub m_dummy: ::c_uchar,
    }
}

#[repr(C)]
pub struct sgx_thread_cond_t {
    pub m_lock: ::uint32_t,
    pub m_queue: sgx_thread_queue_t,
}

pub const SGX_THREAD_COND_INITIALIZER: sgx_thread_cond_t = sgx_thread_cond_t {
    m_lock: 0,
    m_queue: sgx_thread_queue_t {
        m_first: SGX_THREAD_T_NULL,
        m_last: SGX_THREAD_T_NULL
    }
};

//
// sgx_tkey_exchange.h
//


pub type sgx_ra_derive_secret_keys_t = extern "C" fn(p_shared_key: * const sgx_ec256_dh_shared_t,
                                                     kdf_id: ::uint16_t,
                                                     p_smk_key: * mut sgx_ec_key_128bit_t,
                                                     p_sk_key: * mut sgx_ec_key_128bit_t,
                                                     p_mk_key: * mut sgx_ec_key_128bit_t,
                                                     p_vk_key: * mut sgx_ec_key_128bit_t) -> sgx_status_t;

//
// sgx_trts_exception.h
//

pub const EXCEPTION_CONTINUE_SEARCH: ::uint32_t      = 0;
pub const EXCEPTION_CONTINUE_EXECUTION: ::uint32_t   = 0xFFFFFFFF;

impl_enum! {

    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum sgx_exception_vector_t {
        SGX_EXCEPTION_VECTOR_DE = 0,  /* DIV and DIV instructions */
        SGX_EXCEPTION_VECTOR_DB = 1,  /* For Intel use only */
        SGX_EXCEPTION_VECTOR_BP = 3,  /* INT 3 instruction */
        SGX_EXCEPTION_VECTOR_BR = 5,  /* BOUND instruction */
        SGX_EXCEPTION_VECTOR_UD = 6,  /* UD2 instruction or reserved opcode */
        SGX_EXCEPTION_VECTOR_MF = 16, /* x87 FPU floating-point or WAIT/FWAIT instruction */
        SGX_EXCEPTION_VECTOR_AC = 17, /* Any data reference in memory */
        SGX_EXCEPTION_VECTOR_XM = 19, /* SSE/SSE2/SSE3 floating-point instruction */
    }
}

impl_enum!{

    #[repr(u32)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum sgx_exception_type_t {
        SGX_EXCEPTION_HARDWARE = 3,
        SGX_EXCEPTION_SOFTWARE = 6,
    }
}


cfg_if! {
    if #[cfg(target_arch = "x86")] {
        impl_struct! {

            pub struct sgx_cpu_context_t {
                pub eax: ::uint32_t,
                pub ecx: ::uint32_t,
                pub edx: ::uint32_t,
                pub ebx: ::uint32_t,
                pub esp: ::uint32_t,
                pub ebp: ::uint32_t,
                pub esi: ::uint32_t,
                pub edi: ::uint32_t,
                pub eflags: ::uint32_t,
                pub eip: ::uint32_t,
            }
        }
    } else {
        impl_struct! {

            pub struct sgx_cpu_context_t {
                pub rax: ::uint64_t,
                pub rcx: ::uint64_t,
                pub rdx: ::uint64_t,
                pub rbx: ::uint64_t,
                pub rsp: ::uint64_t,
                pub rbp: ::uint64_t,
                pub rsi: ::uint64_t,
                pub rdi: ::uint64_t,
                pub r8: ::uint64_t,
                pub r9: ::uint64_t,
                pub r10: ::uint64_t,
                pub r11: ::uint64_t,
                pub r12: ::uint64_t,
                pub r13: ::uint64_t,
                pub r14: ::uint64_t,
                pub r15: ::uint64_t,
                pub rflags: ::uint64_t,
                pub rip: ::uint64_t,
            }
        }
    }
}

impl_struct! {

    pub struct sgx_exception_info_t {
        pub cpu_context: sgx_cpu_context_t,
        pub exception_vector: sgx_exception_vector_t,
        pub exception_type: sgx_exception_type_t,
    }
}

pub type sgx_exception_handler_t = extern "C" fn(info: * mut sgx_exception_info_t) -> ::uint32_t;

//
// sgx_tseal.h
//

pub const SGX_SEAL_TAG_SIZE: ::size_t  = SGX_AESGCM_MAC_SIZE;
pub const SGX_SEAL_IV_SIZE: ::size_t   = 12;

impl_struct! {

    pub struct sgx_aes_gcm_data_t {
        pub payload_size: ::uint32_t,
        pub reserved: [::uint8_t; 12],
        pub payload_tag: [::uint8_t; SGX_SEAL_TAG_SIZE],
        pub payload: [::uint8_t; 0],
    }

    pub struct sgx_sealed_data_t {
        pub key_request: sgx_key_request_t,
        pub plain_text_offset: ::uint32_t,
        pub reserved: [::uint8_t; 12],
        pub aes_data: sgx_aes_gcm_data_t,
    }
}

//
// sgx_uae_service.h
//


pub const PS_CAP_TRUSTED_TIME: ::size_t        = 0x1;
pub const PS_CAP_MONOTONIC_COUNTER: ::size_t   = 0x2;

impl_struct! {

    pub struct sgx_ps_cap_t {
        pub ps_cap0: ::uint32_t,
        pub ps_cap1: ::uint32_t,
    }
}

//
// sgx_ukey_exchange.h
//


pub type sgx_ecall_get_ga_trusted_t = fn(eid: sgx_enclave_id_t,
                                         retval: * mut sgx_status_t,
                                         context: sgx_ra_context_t,
                                         g_a: * mut sgx_ec256_public_t) -> sgx_status_t;

pub type sgx_ecall_proc_msg2_trusted_t = fn(eid: sgx_enclave_id_t,
                                            retval: * mut sgx_status_t,
                                            context: sgx_ra_context_t,
                                            p_msg2: * const sgx_ra_msg2_t,
                                            p_qe_target: * const sgx_target_info_t,
                                            p_report: * mut sgx_report_t,
                                            nonce: * mut sgx_quote_nonce_t) -> sgx_status_t;

pub type sgx_ecall_get_msg3_trusted_t = fn(eid: sgx_enclave_id_t,
                                           retval: * mut sgx_status_t,
                                           context: sgx_ra_context_t,
                                           quote_size: ::uint32_t,
                                           qe_report: * mut sgx_report_t,
                                           p_msg3: * mut sgx_ra_msg3_t,
                                           msg3_size: ::uint32_t) -> sgx_status_t;

//
// sgx_urts.h
//


pub type sgx_launch_token_t = [::uint8_t; 1024];



//
// trts.pic.h
//
pub const ENCLAVE_INIT_NOT_STARTED: u32 = 0;
pub const ENCLAVE_INIT_IN_PROGRESS: u32 = 1;
pub const ENCLAVE_INIT_DONE: u32        = 2;
pub const ENCLAVE_CRASHED: u32          = 3;

//
// sgx_cpuid.h
//
pub type sgx_cpuinfo_t = [::int32_t; 4];

//
//
//
//
//cfg_if! {
//    if #[cfg(any(not(feature = "NDEBUG"), feature = "EDEBUG"))] {
//        pub const SGX_DEBUG_FLAG: ::int32_t   = 1;
//    } else {
//        pub const SGX_DEBUG_FLAG: ::int32_t   = 0;
//    }
//}


//
// sgx_tprotected_fs.h
//

pub type SGX_FILE = * mut ::c_void;

pub const EOF: i32 = -1;

pub const SEEK_SET: i32 = 0;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;

pub const FILENAME_MAX: u32  = 260;
pub const FOPEN_MAX: u32     = 20;