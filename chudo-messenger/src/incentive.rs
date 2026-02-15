pub struct IncentiveCalculator;
pub struct RewardClaim;
impl IncentiveCalculator {
    pub fn new() -> Self { Self }
    pub fn calculate_reward(&self, _count: usize, _addr: &str) -> RewardClaim { RewardClaim }
}
