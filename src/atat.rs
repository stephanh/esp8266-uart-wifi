use atat::atat_derive::{ATATCmd, ATATResp};
use atat::{ATATCmd, ATATResp, Error};

use heapless::{consts, String};

#[derive(Clone, Debug, ATATCmd)]
#[at_cmd("+GMR", GMR)]
pub struct GetGMR;

#[derive(Clone, Debug, ATATResp)]
pub struct GMR {
    #[at_arg(position = 0)]
    pub id: String<consts::U64>,
}

#[derive(Clone, Debug, ATATResp)]
pub struct NoResponse;

#[derive(Clone, Debug, ATATCmd)]
#[at_cmd("", NoResponse)]
pub struct AT;

#[derive(Clone, Debug, ATATCmd)]
#[at_cmd("+CWMODE_CUR?", CWMODE_CUR)]
pub struct GetCWMODECUR;

//#[derive(ATATCmd)]
//#[at_cmd("+CWMODE_CUR?", CWMODE)]
//pub struct SetCWMODECUR;

#[derive(Debug, ATATResp)]
pub struct CWMODE_CUR {
    #[at_arg(position = 0)]
    pub mode: u128,
}

#[derive(Debug)]
pub struct At;

impl ATATCmd for At {
    type CommandLen = heapless::consts::U16;
    type Response = CWMODE_CUR;

    fn as_str(&self) -> String<Self::CommandLen> {
        String::from("AT+CWMODE_CUR?\r\n")
    }

    fn parse(&self, resp: &str) -> Result<Self::Response, atat::Error> {
        println!("Parsing: {}", resp);
        let e = serde_at::from_str::<CWMODE_CUR>(resp);
        println!("{:?}", e);

        e.map_err(|e| atat::Error::InvalidResponse)
        //Ok(EmptyResponse)
    }
}

#[derive(Debug)]
pub struct EmptyResponse;

impl ATATResp for EmptyResponse {}
