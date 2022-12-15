use curl::easy::Easy;
use std::io::Read;
use udf::prelude::*;

struct MyUdf {}

#[register]
impl BasicUdf for MyUdf {
    type Returns<'a> = Option<i64>;

    fn init(
        _cfg: &UdfCfg<Init>,
        _args: &ArgList<Init>
    ) -> Result<Self, String> {
        udf_log!(Note: "init called");
        Ok(Self {})
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        udf_log!(Note: "process called!");

        let mut data = r#"{"jsonrpc": "2.0", "method": "get_build_info", "params": {}, "id": 1}"#.as_bytes();
        let mut easy = Easy::new();

        easy.url("http://localhost:10180/rpc").unwrap();
        easy.post(true).unwrap();
        easy.post_field_size(data.len() as u64).unwrap();

        let mut transfer = easy.transfer();
        transfer.read_function(|buf| {
            Ok(data.read(buf).unwrap_or(0))
        }).unwrap();
        transfer.perform().unwrap();

        Ok(None)
    }
}
