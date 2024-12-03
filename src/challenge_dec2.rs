use std::{
    net::{Ipv4Addr, Ipv6Addr},
    ops::BitXor,
};

use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EgregiousEncryptionDestinationPacket<IPVersion> {
    from: IPVersion,
    key: IPVersion,
}

#[derive(Deserialize)]
pub struct EgregiousEncryptionKeyPacket<IPVersion> {
    from: IPVersion,
    to: IPVersion,
}

pub async fn egregious_encryption_dest(
    packet: Query<EgregiousEncryptionDestinationPacket<Ipv4Addr>>,
) -> impl IntoResponse {
    let calculated_octets: [u8; 4] = packet
        .from
        .octets()
        .iter()
        .zip(packet.key.octets())
        .map(|(from_oct, key_oct)| from_oct.wrapping_add(key_oct))
        .collect::<Vec<_>>()
        .as_slice()
        .try_into()
        .unwrap();
    let calculated_ip = Ipv4Addr::from(calculated_octets);

    calculated_ip.to_string()
}

pub async fn egregious_encryption_key(
    packet: Query<EgregiousEncryptionKeyPacket<Ipv4Addr>>,
) -> impl IntoResponse {
    let calculated_octets: [u8; 4] = packet
        .to
        .octets()
        .iter()
        .zip(packet.from.octets())
        .map(|(to_oct, from_oct)| to_oct.wrapping_sub(from_oct))
        .collect::<Vec<_>>()
        .as_slice()
        .try_into()
        .unwrap();
    let calculated_ip = Ipv4Addr::from(calculated_octets);

    calculated_ip.to_string()
}

pub async fn egregious_encryption_dest_v6(
    packet: Query<EgregiousEncryptionDestinationPacket<Ipv6Addr>>,
) -> impl IntoResponse {
    let calculated_octets: [u8; 16] = packet
        .from
        .octets()
        .iter()
        .zip(packet.key.octets())
        .map(|(from_oct, key_oct)| from_oct.bitxor(key_oct))
        .collect::<Vec<_>>()
        .as_slice()
        .try_into()
        .unwrap();
    let calculated_ip = Ipv6Addr::from(calculated_octets);

    calculated_ip.to_string()
}

pub async fn egregious_encryption_key_v6(
    packet: Query<EgregiousEncryptionKeyPacket<Ipv6Addr>>,
) -> impl IntoResponse {
    let calculated_octets: [u8; 16] = packet
        .to
        .octets()
        .iter()
        .zip(packet.from.octets())
        .map(|(to_oct, from_oct)| to_oct.bitxor(from_oct))
        .collect::<Vec<_>>()
        .as_slice()
        .try_into()
        .unwrap();
    let calculated_ip = Ipv6Addr::from(calculated_octets);

    calculated_ip.to_string()
}
