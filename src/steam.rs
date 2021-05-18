#[derive(Debug, Copy, Clone)]
pub enum SteamInstance {
    // we allow 3 simultaneous user account instances right now, 1= desktop, 2 = console, 4 = web,
    // 0 = all
    AllInstances = 0,
    DesktopInstance = 1,
    ConsoleInstance = 2,
    WebInstance = 4,
}

#[derive(Debug, Copy, Clone)]
pub enum SteamUniverse {
    Unspecified = 0,
    Public = 1,
    Beta = 2,
    Internal = 3,
    Dev = 4,
    RC = 5,
}

#[derive(Debug, Copy, Clone)]
pub enum SteamAccountType {
    Invalid = 0,
    Individual = 1,     // single user account
    Multiseat = 2,      // multiseat (e.g. cybercafe) account
    GameServer = 3,     // game server account
    AnonGameServer = 4, // anonymous game server account
    Pending = 5,        // pending
    ContentServer = 6,  // content server
    Clan = 7,
    Chat = 8,
    ConsoleUser = 9, // Fake SteamID for local PSN account on PS3 or Live account on 360, etc.
    AnonUser = 10,
    Max = 16, // Max of 16 items in this field
}

#[derive(Debug, Copy, Clone)]
pub struct SteamId {
    pub universe: SteamUniverse,
    pub instance: SteamInstance,
    pub account_type: SteamAccountType,
    pub xuid: u32,
}

impl Default for SteamId {
    fn default() -> Self {
        SteamId {
            universe: SteamUniverse::Public,
            instance: SteamInstance::DesktopInstance,
            account_type: SteamAccountType::Individual,
            xuid: 0,
        }
    }
}

pub struct SteamIdBuilder {
    steam_id: SteamId,
}

impl SteamIdBuilder {
    pub fn from_xuid(xuid: u32) -> Self {
        SteamIdBuilder {
            steam_id: SteamId {
                xuid,
                ..Default::default()
            },
        }
    }

    pub fn with_universe(&mut self, universe: SteamUniverse) -> &mut Self {
        self.steam_id.universe = universe;
        self
    }

    pub fn with_account_type(&mut self, account_type: SteamAccountType) -> &mut Self {
        self.steam_id.account_type = account_type;
        self
    }

    pub fn with_instance(&mut self, instance: SteamInstance) -> &mut Self {
        self.steam_id.instance = instance;
        self
    }

    pub fn build(&self) -> SteamId {
        self.steam_id
    }
}

impl From<SteamId> for u64 {
    /// Turns the Steam ID into its 64bit integer representation
    /// Source: https://developer.valvesoftware.com/wiki/SteamID#As_Represented_in_Computer_Programs
    fn from(steamid: SteamId) -> Self {
        steamid.xuid as u64
            | (steamid.instance as u64) << 32u64
            | (steamid.account_type as u64) << 52u64
            | (steamid.universe as u64) << 56u64
    }
}

impl From<SteamIdBuilder> for SteamId {
    fn from(builder: SteamIdBuilder) -> Self {
        builder.steam_id
    }
}
