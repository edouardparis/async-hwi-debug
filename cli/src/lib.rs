pub mod command {
    use async_hwi::{
        bitbox::{api::runtime, BitBox02, PairingBitbox02WithLocalCache},
        coldcard,
        jade::{self, Jade},
        ledger::{HidApi, Ledger, LedgerSimulator, TransportHID},
        specter::{Specter, SpecterSimulator},
        HWI,
    };
    use bitcoin::{hashes::hex::FromHex, Network};
    use std::error::Error;

    pub struct Wallet<'a> {
        pub name: Option<&'a String>,
        pub policy: Option<&'a String>,
        pub hmac: Option<&'a String>,
    }

    pub async fn list(
        network: Network,
        wallet: Option<Wallet<'_>>,
    ) -> Result<Vec<Box<dyn HWI + Send>>, Box<dyn Error>> {
        let mut hws = Vec::new();

        let api = Box::new(HidApi::new().unwrap());

        for detected in Ledger::<TransportHID>::enumerate(&api) {
            if let Ok(mut device) = Ledger::<TransportHID>::connect(&api, detected) {
                if let Some(ref wallet) = wallet {
                    let hmac = if let Some(s) = wallet.hmac {
                        let mut h = [b'\0'; 32];
                        h.copy_from_slice(&Vec::from_hex(s)?);
                        Some(h)
                    } else {
                        None
                    };
                    device = device.with_wallet(
                        wallet
                            .name
                            .ok_or::<Box<dyn Error>>("ledger requires a wallet name".into())?,
                        wallet
                            .policy
                            .ok_or::<Box<dyn Error>>("ledger requires a wallet policy".into())?,
                        hmac,
                    )?;
                }
                hws.push(device.into());
            }
        }

        Ok(hws)
    }
}
