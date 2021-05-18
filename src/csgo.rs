use crate::{num::SwapNibbles, steam::SteamId};
use anyhow::{anyhow, Result};
use bitintr::{Pdep, Pext};
use std::convert::{TryFrom, TryInto};

// CS:GO uses a reduced alphabet for friend codes without ambiguous characters
// like 1 and I
const ALPHABET: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789";

// This is kinda dumb, skip one bit, take four and skip one again. Compressed
// into a bitmask This is 1111011110111101111011110111101111011110u64
// but Rust has a length limit on literals
const NIBBLE_MASK: u64 = 0xF7BDEF7BDEu64;

#[derive(Debug, Copy, Clone)]
pub struct FriendCode {
    pub xuid: u32,
}

impl FriendCode {
    /// Calculate the first byte of the hash of the XUID
    pub fn calculate_noise(xuid: u32) -> Result<u32> {
        // This is b"CSGO\0\0\0\0" as an integer
        let xuid = xuid as u64 | 0x4353474F00000000u64;
        let digest = md5::compute(xuid.to_le_bytes());
        let digest = &digest.0[0..4];
        let noise = u32::from_le_bytes(digest.try_into()?) & 0xFF;
        Ok((noise as u8).reverse_bits() as u32)
    }
}

impl TryInto<String> for FriendCode {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<String, Self::Error> {
        // There's 8 bits in a friend code that could theoretically be chosen at random
        // CS:GO uses the first byte of the MD5 hash of the account id for this
        let noise = FriendCode::calculate_noise(self.xuid)?;
        // Scatter the noise so it can fit in ~NIBBLE_MASK
        let noise = (noise as u64).pdep(!NIBBLE_MASK);
        // Prepare the steam id for scattering
        let steamid = self.xuid.swap_nibbles().swap_bytes() as u64;
        // Scatter and combine with the noise
        let steamid = steamid.pdep(NIBBLE_MASK) | noise;
        let steamid = steamid.swap_bytes();
        // 5 bits encode one character of the resulting friend code
        let code = (0..13)
            .into_iter()
            .map(|i| (steamid >> (5 * i)) & 0x1F)
            .map(|i| ALPHABET.as_bytes().get(i as usize).cloned())
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| anyhow!("Bad friend code!"))?;
        let code = String::from_utf8(code)?;
        // Format it all pretty and we're done
        Ok(format!("{}-{}", &code[4..9], &code[9..13]))
    }
}

impl From<SteamId> for FriendCode {
    fn from(steamid: SteamId) -> Self {
        FriendCode { xuid: steamid.xuid }
    }
}

impl From<FriendCode> for SteamId {
    fn from(code: FriendCode) -> Self {
        // This obviously looses the ability to set universe, account type or instance
        // But this is how CS:GO does it...
        SteamId {
            xuid: code.xuid,
            ..Default::default()
        }
    }
}

impl From<u32> for FriendCode {
    fn from(xuid: u32) -> Self {
        FriendCode { xuid }
    }
}

impl TryFrom<&str> for FriendCode {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO: Validate
        let code = format!("AAAA-{}", value);
        let code = code.split('-').collect::<String>();
        let state = code
            .chars()
            // Rust pls give try_map
            .map(|c| {
                ALPHABET
                    .chars()
                    .position(|inner| inner == c)
                    .map(|result| result as u8)
                    .ok_or_else(|| anyhow!("Illegal character in friend code: {}", c))
            })
            .try_rfold(0u64, |c, a| -> Result<u64> {
                Ok(c.checked_shl(5)
                    .ok_or_else(|| anyhow!("Bad friend code!"))?
                    + (a? as u64))
            })?
            .swap_bytes();
        if state & 0xFFFFFF0000000000u64 == 0 {
            // x86 comes with an intrinsic for extracting packed bits
            // https://www.felixcloutier.com/x86/pext
            // Uses a fallback on other platforms
            let xuid = state.pext(NIBBLE_MASK);
            // Swap all nibbles in the integer
            let xuid = xuid.swap_nibbles();
            // Do a final endianness swap
            let xuid = (xuid as u32).swap_bytes();
            return Ok(FriendCode { xuid });
        }

        Err(anyhow!("Bad friend code!"))
    }
}
