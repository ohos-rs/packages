use base64::engine::Engine;
use getrandom::getrandom;
use napi_derive_ohos::napi;
use napi_ohos::{Env, Error, Result, Status, Task};

use crate::Version;

#[inline]
pub(crate) fn gen_salt() -> bcrypt::BcryptResult<[u8; 16]> {
    let mut s = [0u8; 16];
    getrandom(&mut s)
        .map(|_| s)
        .map_err(bcrypt::BcryptError::from)?;
    Ok(s)
}

#[inline]
pub(crate) fn format_salt(rounds: u32, version: &Version, salt: &[u8; 16]) -> String {
    let mut base64_string = String::new();
    let engine = base64::engine::general_purpose::GeneralPurpose::new(
        &base64::alphabet::BCRYPT,
        base64::engine::general_purpose::PAD,
    );
    engine.encode_string(salt, &mut base64_string);
    format!("${version}${rounds:0>2}${base64_string}")
}

pub struct SaltTask {
    pub(crate) round: u32,
    pub(crate) version: Version,
}

#[napi]
impl Task for SaltTask {
    type Output = String;
    type JsValue = String;

    fn compute(&mut self) -> Result<Self::Output> {
        let random = gen_salt().map_err(|err| {
            Error::new(
                Status::GenericFailure,
                format!("Generate salt failed {err}"),
            )
        })?;
        Ok(format_salt(self.round, &self.version, &random))
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}
