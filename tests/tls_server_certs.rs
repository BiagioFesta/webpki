// Copyright 2022 Joseph Birr-Pixton.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
#![cfg(all(feature = "alloc", feature = "ring"))]

use core::time::Duration;

use pki_types::{CertificateDer, SignatureVerificationAlgorithm, UnixTime};
use webpki::{extract_trust_anchor, KeyUsage};

static ALL_SIGALGS: &[&dyn SignatureVerificationAlgorithm] = &[
    webpki::ECDSA_P256_SHA256,
    webpki::ECDSA_P256_SHA384,
    webpki::ECDSA_P384_SHA256,
    webpki::ECDSA_P384_SHA384,
    webpki::ED25519,
    webpki::RSA_PKCS1_2048_8192_SHA256,
    webpki::RSA_PKCS1_2048_8192_SHA384,
    webpki::RSA_PKCS1_2048_8192_SHA512,
    webpki::RSA_PKCS1_3072_8192_SHA384,
];

fn check_cert(
    ee: &[u8],
    ca: &[u8],
    valid_names: &[&str],
    invalid_names: &[&str],
) -> Result<(), webpki::Error> {
    let ca_cert_der = CertificateDer::from(ca);
    let anchors = [extract_trust_anchor(&ca_cert_der).unwrap()];

    let ee_der = CertificateDer::from(ee);
    let time = UnixTime::since_unix_epoch(Duration::from_secs(0x1fed_f00d));
    let cert = webpki::EndEntityCert::try_from(&ee_der).unwrap();
    cert.verify_for_usage(
        ALL_SIGALGS,
        &anchors,
        &[],
        time,
        KeyUsage::server_auth(),
        None,
    )?;

    for valid in valid_names {
        let name = webpki::SubjectNameRef::try_from_ascii_str(valid).unwrap();
        assert_eq!(cert.verify_is_valid_for_subject_name(name), Ok(()));
    }

    for invalid in invalid_names {
        let name = webpki::SubjectNameRef::try_from_ascii_str(invalid).unwrap();
        assert_eq!(
            cert.verify_is_valid_for_subject_name(name),
            Err(webpki::Error::CertNotValidForName)
        );
    }

    Ok(())
}

// DO NOT EDIT BELOW: generated by tests/generate.py

#[test]
fn no_name_constraints() {
    let ee = include_bytes!("tls_server_certs/no_name_constraints.ee.der");
    let ca = include_bytes!("tls_server_certs/no_name_constraints.ca.der");
    assert_eq!(
        check_cert(ee, ca, &["dns.example.com"], &["subject.example.com"]),
        Ok(())
    );
}

#[test]
fn additional_dns_labels() {
    let ee = include_bytes!("tls_server_certs/additional_dns_labels.ee.der");
    let ca = include_bytes!("tls_server_certs/additional_dns_labels.ca.der");
    assert_eq!(
        check_cert(
            ee,
            ca,
            &["host1.example.com", "host2.example.com"],
            &["subject.example.com"]
        ),
        Ok(())
    );
}

#[test]
fn disallow_dns_san() {
    let ee = include_bytes!("tls_server_certs/disallow_dns_san.ee.der");
    let ca = include_bytes!("tls_server_certs/disallow_dns_san.ca.der");
    assert_eq!(
        check_cert(ee, ca, &[], &[]),
        Err(webpki::Error::NameConstraintViolation)
    );
}

#[test]
fn allow_subject_common_name() {
    let ee = include_bytes!("tls_server_certs/allow_subject_common_name.ee.der");
    let ca = include_bytes!("tls_server_certs/allow_subject_common_name.ca.der");
    assert_eq!(check_cert(ee, ca, &[], &["allowed.example.com"]), Ok(()));
}

#[test]
fn allow_dns_san() {
    let ee = include_bytes!("tls_server_certs/allow_dns_san.ee.der");
    let ca = include_bytes!("tls_server_certs/allow_dns_san.ca.der");
    assert_eq!(check_cert(ee, ca, &["allowed.example.com"], &[]), Ok(()));
}

#[test]
fn allow_dns_san_and_subject_common_name() {
    let ee = include_bytes!("tls_server_certs/allow_dns_san_and_subject_common_name.ee.der");
    let ca = include_bytes!("tls_server_certs/allow_dns_san_and_subject_common_name.ca.der");
    assert_eq!(
        check_cert(
            ee,
            ca,
            &["allowed-san.example.com"],
            &["allowed-cn.example.com"]
        ),
        Ok(())
    );
}

#[test]
fn disallow_dns_san_and_allow_subject_common_name() {
    let ee =
        include_bytes!("tls_server_certs/disallow_dns_san_and_allow_subject_common_name.ee.der");
    let ca =
        include_bytes!("tls_server_certs/disallow_dns_san_and_allow_subject_common_name.ca.der");
    assert_eq!(
        check_cert(ee, ca, &[], &[]),
        Err(webpki::Error::NameConstraintViolation)
    );
}

#[test]
fn we_incorrectly_ignore_name_constraints_on_name_in_subject() {
    let ee = include_bytes!(
        "tls_server_certs/we_incorrectly_ignore_name_constraints_on_name_in_subject.ee.der"
    );
    let ca = include_bytes!(
        "tls_server_certs/we_incorrectly_ignore_name_constraints_on_name_in_subject.ca.der"
    );
    assert_eq!(check_cert(ee, ca, &[], &[]), Ok(()));
}

#[test]
fn reject_constraints_on_unimplemented_names() {
    let ee = include_bytes!("tls_server_certs/reject_constraints_on_unimplemented_names.ee.der");
    let ca = include_bytes!("tls_server_certs/reject_constraints_on_unimplemented_names.ca.der");
    assert_eq!(
        check_cert(ee, ca, &[], &[]),
        Err(webpki::Error::NameConstraintViolation)
    );
}

#[test]
fn we_ignore_constraints_on_names_that_do_not_appear_in_cert() {
    let ee = include_bytes!(
        "tls_server_certs/we_ignore_constraints_on_names_that_do_not_appear_in_cert.ee.der"
    );
    let ca = include_bytes!(
        "tls_server_certs/we_ignore_constraints_on_names_that_do_not_appear_in_cert.ca.der"
    );
    assert_eq!(
        check_cert(ee, ca, &["notexample.com"], &["example.com"]),
        Ok(())
    );
}

#[test]
fn wildcard_san_accepted_if_in_subtree() {
    let ee = include_bytes!("tls_server_certs/wildcard_san_accepted_if_in_subtree.ee.der");
    let ca = include_bytes!("tls_server_certs/wildcard_san_accepted_if_in_subtree.ca.der");
    assert_eq!(
        check_cert(
            ee,
            ca,
            &["bob.example.com", "jane.example.com"],
            &["example.com", "uh.oh.example.com"]
        ),
        Ok(())
    );
}

#[test]
fn wildcard_san_rejected_if_in_excluded_subtree() {
    let ee = include_bytes!("tls_server_certs/wildcard_san_rejected_if_in_excluded_subtree.ee.der");
    let ca = include_bytes!("tls_server_certs/wildcard_san_rejected_if_in_excluded_subtree.ca.der");
    assert_eq!(
        check_cert(ee, ca, &[], &[]),
        Err(webpki::Error::NameConstraintViolation)
    );
}

#[test]
fn ip4_address_san_rejected_if_in_excluded_subtree() {
    let ee =
        include_bytes!("tls_server_certs/ip4_address_san_rejected_if_in_excluded_subtree.ee.der");
    let ca =
        include_bytes!("tls_server_certs/ip4_address_san_rejected_if_in_excluded_subtree.ca.der");
    assert_eq!(
        check_cert(ee, ca, &[], &[]),
        Err(webpki::Error::NameConstraintViolation)
    );
}

#[test]
fn ip4_address_san_allowed_if_outside_excluded_subtree() {
    let ee = include_bytes!(
        "tls_server_certs/ip4_address_san_allowed_if_outside_excluded_subtree.ee.der"
    );
    let ca = include_bytes!(
        "tls_server_certs/ip4_address_san_allowed_if_outside_excluded_subtree.ca.der"
    );
    assert_eq!(check_cert(ee, ca, &["12.34.56.78"], &[]), Ok(()));
}

#[test]
fn ip4_address_san_rejected_if_excluded_is_sparse_cidr_mask() {
    let ee = include_bytes!(
        "tls_server_certs/ip4_address_san_rejected_if_excluded_is_sparse_cidr_mask.ee.der"
    );
    let ca = include_bytes!(
        "tls_server_certs/ip4_address_san_rejected_if_excluded_is_sparse_cidr_mask.ca.der"
    );
    assert_eq!(
        check_cert(ee, ca, &[], &[]),
        Err(webpki::Error::InvalidNetworkMaskConstraint)
    );
}

#[test]
fn ip4_address_san_allowed() {
    let ee = include_bytes!("tls_server_certs/ip4_address_san_allowed.ee.der");
    let ca = include_bytes!("tls_server_certs/ip4_address_san_allowed.ca.der");
    assert_eq!(
        check_cert(
            ee,
            ca,
            &["12.34.56.78"],
            &[
                "12.34.56.77",
                "12.34.56.79",
                "0000:0000:0000:0000:0000:ffff:0c22:384e"
            ]
        ),
        Ok(())
    );
}

#[test]
fn ip6_address_san_rejected_if_in_excluded_subtree() {
    let ee =
        include_bytes!("tls_server_certs/ip6_address_san_rejected_if_in_excluded_subtree.ee.der");
    let ca =
        include_bytes!("tls_server_certs/ip6_address_san_rejected_if_in_excluded_subtree.ca.der");
    assert_eq!(
        check_cert(ee, ca, &[], &[]),
        Err(webpki::Error::NameConstraintViolation)
    );
}

#[test]
fn ip6_address_san_allowed_if_outside_excluded_subtree() {
    let ee = include_bytes!(
        "tls_server_certs/ip6_address_san_allowed_if_outside_excluded_subtree.ee.der"
    );
    let ca = include_bytes!(
        "tls_server_certs/ip6_address_san_allowed_if_outside_excluded_subtree.ca.der"
    );
    assert_eq!(
        check_cert(ee, ca, &["2001:0db9:0000:0000:0000:0000:0000:0001"], &[]),
        Ok(())
    );
}

#[test]
fn ip6_address_san_allowed() {
    let ee = include_bytes!("tls_server_certs/ip6_address_san_allowed.ee.der");
    let ca = include_bytes!("tls_server_certs/ip6_address_san_allowed.ca.der");
    assert_eq!(
        check_cert(
            ee,
            ca,
            &["2001:0db9:0000:0000:0000:0000:0000:0001"],
            &["12.34.56.78"]
        ),
        Ok(())
    );
}

#[test]
fn ip46_mixed_address_san_allowed() {
    let ee = include_bytes!("tls_server_certs/ip46_mixed_address_san_allowed.ee.der");
    let ca = include_bytes!("tls_server_certs/ip46_mixed_address_san_allowed.ca.der");
    assert_eq!(
        check_cert(
            ee,
            ca,
            &["12.34.56.78", "2001:0db9:0000:0000:0000:0000:0000:0001"],
            &[
                "12.34.56.77",
                "12.34.56.79",
                "0000:0000:0000:0000:0000:ffff:0c22:384e"
            ]
        ),
        Ok(())
    );
}

#[test]
fn permit_directory_name_not_implemented() {
    let ee = include_bytes!("tls_server_certs/permit_directory_name_not_implemented.ee.der");
    let ca = include_bytes!("tls_server_certs/permit_directory_name_not_implemented.ca.der");
    assert_eq!(
        check_cert(ee, ca, &[], &[]),
        Err(webpki::Error::NameConstraintViolation)
    );
}

#[test]
fn exclude_directory_name_not_implemented() {
    let ee = include_bytes!("tls_server_certs/exclude_directory_name_not_implemented.ee.der");
    let ca = include_bytes!("tls_server_certs/exclude_directory_name_not_implemented.ca.der");
    assert_eq!(
        check_cert(ee, ca, &[], &[]),
        Err(webpki::Error::NameConstraintViolation)
    );
}

#[test]
fn invalid_dns_name_matching() {
    let ee = include_bytes!("tls_server_certs/invalid_dns_name_matching.ee.der");
    let ca = include_bytes!("tls_server_certs/invalid_dns_name_matching.ca.der");
    assert_eq!(check_cert(ee, ca, &["dns.example.com"], &[]), Ok(()));
}
