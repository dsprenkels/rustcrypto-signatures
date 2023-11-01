use std::io::prelude::*;

use signature::rand_core::RngCore;

mod nistrng;

#[derive(Debug, Clone)]
struct KAT {
    count: u64,
    seed: Vec<u8>,
    mlen: u64,
    msg: Vec<u8>,
    pk: Vec<u8>,
    sk: Vec<u8>,
    smlen: u64,
    sm: Vec<u8>,
}

fn load_kats(kat_file: &str) -> (String, Vec<KAT>) {
    let dilithium2_kat = download_kat_file("PQCsignKAT_Dilithium2.rsp");
    let kat_strs = dilithium2_kat.trim_end().split("\n\n").collect::<Vec<_>>();
    (
        kat_strs[0].to_string(),
        (kat_strs[1..])
            .into_iter()
            .map(|s| parse_kat(*s))
            .collect::<Vec<_>>(),
    )
}

fn download_kat_file(kat_file: &str) -> String {
    let url = format!(
        "https://raw.githubusercontent.com/dsprenkels/rustcrypto-signatures/kat/dilithium/kat/{}",
        kat_file
    );
    let output = std::process::Command::new("wget")
        .args(&["--quiet", "--tries=3", "--timeout=5", "-O", "-", &url])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .output()
        .expect(&format!("failed to download KAT file '{}'", kat_file));
    String::from_utf8(output.stdout).expect("KAT file is not valid UTF-8")
}

fn parse_kat(kat_str: &str) -> KAT {
    let lines = kat_str.split("\n").collect::<Vec<_>>();
    let expected_keys = ["count", "seed", "mlen", "msg", "pk", "sk", "smlen", "sm"];
    let mut str_values = [""; 8];
    for (i, line) in lines.iter().enumerate() {
        let expected_key = expected_keys[i];
        let (key, value) = line.split_once(" = ").expect(&format!(
            "invalid KAT line, cannot split around ' = ', '{}'",
            line
        ));
        assert_eq!(
            key, expected_key,
            "invalid key in KAT line: '{}' should be '{}'",
            key, expected_key
        );
        str_values[i] = value.into();
    }
    KAT {
        count: parse_int(&str_values[0]),
        seed: parse_hex(&str_values[1]),
        mlen: parse_int(&str_values[2]),
        msg: parse_hex(&str_values[3]),
        pk: parse_hex(&str_values[4]),
        sk: parse_hex(&str_values[5]),
        smlen: parse_int(&str_values[6]),
        sm: parse_hex(&str_values[7]),
    }
}

fn parse_int<T: std::str::FromStr>(s: &str) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.trim_end().parse().expect("invalid integer in KAT file")
}

fn parse_hex(s: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let mut chars = s.chars();
    while let (Some(h), Some(l)) = (chars.next(), chars.next()) {
        let byte = match (h.to_digit(16), l.to_digit(16)) {
            (Some(h), Some(l)) => ((h << 4) | l) as u8,
            _ => panic!("invalid hex string"),
        };
        result.push(byte);
    }
    result
}

#[test]
fn test_seed_kat() {
    let (header, kats) = load_kats("PQCsignKAT_Dilithium2.rsp");
    writeln!(std::io::stderr(), "KAT file header: '{}'", header).unwrap();

    let mut seeds = [[0; 48]; 100];
    let mut msgs = [[0; 3300]; 100];

    let mut entropy_input = [0; 48];
    for (idx, b) in entropy_input.iter_mut().enumerate() {
        *b = idx as u8;
    }
    let mut rb = nistrng::RandomBytes::init(&entropy_input);
    dbg!(rb.key[0], rb.v[0]);
    for idx in 0..100 {
        let mlen = 33 * (idx + 1);
        rb.fill_bytes(&mut seeds[idx]);
        rb.fill_bytes(&mut msgs[idx][..mlen]);
        assert_eq!(&seeds[idx], &kats[idx].seed[..]);
    }
}

#[test]
#[ignore = "key encoding not implemented yet"]
fn test_keypair_kat() {
    let (header, kats) = load_kats("PQCsignKAT_Dilithium2.rsp");
    writeln!(std::io::stderr(), "KAT file header: '{}'", header).unwrap();

    for kat in &kats {
        let seed = &kat.seed.clone().try_into().expect("bad seed length");
        let mut rb = nistrng::RandomBytes::init(seed);
        let sk = dilithium::dilithium2::SigningKey::random(&mut rb);
        assert_eq!(&sk.to_bytes()[..], &kat.sk[..]);
        // TODO: Check the public key as well
    }
}
