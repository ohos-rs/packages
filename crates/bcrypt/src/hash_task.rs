use bcrypt::Version;
use napi_derive_ohos::napi;
use napi_ohos::{
    bindgen_prelude::{Either, Uint8Array},
    Env, Error, Result, Status, Task,
};

pub struct HashTask {
    buf: Either<Uint8Array, String>,
    cost: u32,
    salt: [u8; 16],
    version: Option<Version>,
}

impl HashTask {
    #[inline]
    pub fn new(
        buf: Either<Uint8Array, String>,
        cost: u32,
        salt: [u8; 16],
        version: Option<Version>,
    ) -> HashTask {
        HashTask {
            buf,
            cost,
            salt,
            version,
        }
    }

    #[inline]
    pub fn hash(buf: &[u8], salt: [u8; 16], cost: u32, version: Option<Version>) -> Result<String> {
        bcrypt::hash_with_salt(buf, cost, salt)
            .map(|hash_part| match version {
                Some(v) => hash_part.format_for_version(v).to_string(),
                None => hash_part.to_string(),
            })
            .map_err(|err| Error::new(Status::GenericFailure, format!("{err}")))
    }
}

#[napi]
impl Task for HashTask {
    type Output = String;
    type JsValue = String;

    fn compute(&mut self) -> Result<Self::Output> {
        Self::hash(
            self.buf.as_ref(),
            self.salt,
            self.cost,
            self.version.clone(),
        )
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}
