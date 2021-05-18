mod csgo;
mod num;
mod steam;

#[cfg(test)]
mod tests {
    use crate::csgo::FriendCode;
    use anyhow::Result;
    use std::convert::{TryFrom, TryInto};

    // GabeN's CS:GO friend code
    const FRIEND_CODE: &str = "SUCVS-FADA";

    #[test]
    fn test_decode() {
        let result = FriendCode::try_from(FRIEND_CODE);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().xuid, 22202);
    }

    #[test]
    fn test_encode() {
        let result = FriendCode::from(22202);
        let code: Result<String> = result.try_into();
        assert!(code.is_ok());
        assert_eq!(code.unwrap(), FRIEND_CODE);
    }
}
