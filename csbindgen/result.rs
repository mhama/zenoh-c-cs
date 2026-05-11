//
// Copyright (c) 2017, 2024 ZettaScale Technology.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh team, <zenoh@zettascale.tech>
//

// original: pub type z_result_t = i8;

// this definition is not the same as original rust code. just for csbindgen
#[repr(i8)]
pub enum z_result_t {
    Z_XXXXXX = 100, 
    Z_CHANNEL_DISCONNECTED = 1,
    Z_CHANNEL_NODATA = 2,
    Z_OK = 0,
    Z_EINVAL = -1,
    Z_EPARSE = -2,
    Z_EIO = -3,
    Z_ENETWORK = -4,
    Z_ENULL = -5,
    Z_EUNAVAILABLE = -6,
    Z_EDESERIALIZE = -7,
    Z_ESESSION_CLOSED = -8,
    Z_EUTF8 = -9,
    Z_EBUSY_MUTEX = -16,
    Z_EINVAL_MUTEX = -22,
    Z_EAGAIN_MUTEX = -11,
    Z_EPOISON_MUTEX = -22, // same as Z_EINVAL_MUTEX
    Z_EGENERIC = -128,
}


