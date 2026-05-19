use amun_kernel_types::PublicHash32;
use amun_ntr::charity::CharityDistributor;
use crate::types::SdkResult;

pub struct CharityApi {
    pub distributor: CharityDistributor,
}

impl CharityApi {
    pub fn new() -> Self { Self { distributor: CharityDistributor::new() } }

    pub fn donate(&mut self, recipient: PublicHash32, amount: u64) -> SdkResult<()> {
        match self.distributor.distribute(recipient, amount) {
            Ok(()) => SdkResult::ok(()),
            Err(_) => SdkResult::err("Donation failed"),
        }
    }

    pub fn remaining_funds(&self) -> SdkResult<u64> {
        SdkResult::ok(self.distributor.remaining)
    }

    pub fn total_distributed(&self) -> SdkResult<u64> {
        SdkResult::ok(self.distributor.total_distributed)
    }
}
