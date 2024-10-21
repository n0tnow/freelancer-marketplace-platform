
#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::ToString;
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec, Map, token};

mod job;
mod user;
mod types;

use job::{Job, JobStatus};
use user::{User, UserRole};
use types::{JobId, PaymentId, CampaignId};

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone)]
#[contracttype]
pub struct Transaction {
    pub id: u32,
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub message: Symbol,
    pub timestamp: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct RegularPayment {
    pub id: PaymentId,
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub interval: u64,
    pub next_payment: u64,
    pub message: Symbol,
}

#[derive(Clone)]
#[contracttype]
pub struct Campaign {
    pub id: CampaignId,
    pub creator: Address,
    pub title: Symbol,
    pub description: Symbol,
    pub goal: i128,
    pub raised: i128,
    pub end_time: u64,
    pub token: Address,
    pub status: CampaignStatus,
    pub backers: Vec<Address>,
    pub tiers: Vec<FundingTier>,
}

#[derive(Clone, PartialEq)]
#[contracttype]
pub enum CampaignStatus {
    Active,
    Successful,
    Failed,
    Closed,
}

#[derive(Clone)]
#[contracttype]
pub struct FundingTier {
    pub amount: i128,
    pub reward: Symbol,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Users,
    Jobs,
    UserIdCounter,
    JobIdCounter,
    Transactions,
    TransactionCounter,
    RegularPayments,
    RegularPaymentCounter,
    Campaigns,
    CampaignIdCounter,
}

#[contract]
pub struct FreelancerMarketplace;

#[contractimpl]
impl FreelancerMarketplace {
    pub fn initialize(env: Env) {
        let users: Map<Address, User> = Map::new(&env);
        let jobs: Map<JobId, Job> = Map::new(&env);
        let transactions: Vec<Transaction> = Vec::new(&env);
        let regular_payments: Vec<RegularPayment> = env.storage().instance().get(&DataKey::RegularPayments).unwrap_or_else(|| Vec::new(&env));
        let campaigns: Map<CampaignId, Campaign> = Map::new(&env);
        env.storage().instance().set(&DataKey::Users, &users);
        env.storage().instance().set(&DataKey::Jobs, &jobs);
        env.storage().instance().set(&DataKey::UserIdCounter, &0u32);
        env.storage().instance().set(&DataKey::JobIdCounter, &0u32);
        env.storage().instance().set(&DataKey::Transactions, &transactions);
        env.storage().instance().set(&DataKey::TransactionCounter, &0u32);
        env.storage().instance().set(&DataKey::RegularPayments, &regular_payments);
        env.storage().instance().set(&DataKey::RegularPaymentCounter, &0u32);
        env.storage().instance().set(&DataKey::Campaigns, &campaigns);
        env.storage().instance().set(&DataKey::CampaignIdCounter, &0u32);
    }

    pub fn register_user(env: Env, address: Address, role: UserRole) -> Address {
        let mut users: Map<Address, User> = env.storage().instance().get(&DataKey::Users).unwrap_or_else(|| Map::new(&env));
        let user = User::new(&env, address.clone(), role);

        users.set(address.clone(), user);
        env.storage().instance().set(&DataKey::Users, &users);

        address
    }

    pub fn create_job(env: Env, employer: Address, title: Symbol, description: Symbol, budget: i128, token: Address) -> JobId {
        let mut jobs: Map<JobId, Job> = env.storage().instance().get(&DataKey::Jobs).unwrap_or_else(|| Map::new(&env));
        let mut job_id_counter: u32 = env.storage().instance().get(&DataKey::JobIdCounter).unwrap_or(0);

        job_id_counter += 1;
        let job_id = Symbol::new(&env, &job_id_counter.to_string());
        let job = Job::new(&env, job_id.clone(), employer.clone(), title, description, budget, token.clone());

        jobs.set(job_id.clone(), job);
        env.storage().instance().set(&DataKey::Jobs, &jobs);
        env.storage().instance().set(&DataKey::JobIdCounter, &job_id_counter);

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&employer, &env.current_contract_address(), &budget);

        Self::record_transaction(env.clone(), employer, env.current_contract_address(), budget, Symbol::new(&env, "Job creation"));

        job_id
    }

    pub fn submit_proposal(env: Env, freelancer: Address, job_id: JobId, price: i128) {
        let mut jobs: Map<JobId, Job> = env.storage().instance().get(&DataKey::Jobs).unwrap();
        let mut job = jobs.get(job_id.clone()).unwrap();

        if job.status != JobStatus::Open {
            panic!("Job is not open for proposals");
        }

        job.add_proposal(freelancer, price);
        jobs.set(job_id.clone(), job);
        env.storage().instance().set(&DataKey::Jobs, &jobs);
    }

    pub fn accept_proposal(env: Env, employer: Address, job_id: JobId, freelancer: Address) {
        let mut jobs: Map<JobId, Job> = env.storage().instance().get(&DataKey::Jobs).unwrap();
        let mut job = jobs.get(job_id.clone()).unwrap();

        if job.employer != employer {
            panic!("Only the employer can accept proposals");
        }

        if job.status != JobStatus::Open {
            panic!("Job is not open for accepting proposals");
        }

        job.accept_proposal(freelancer);
        jobs.set(job_id.clone(), job);
        env.storage().instance().set(&DataKey::Jobs, &jobs);
    }

    pub fn complete_job(env: Env, freelancer: Address, job_id: JobId) {
        let mut jobs: Map<JobId, Job> = env.storage().instance().get(&DataKey::Jobs).unwrap();
        let mut job = jobs.get(job_id.clone()).unwrap();

        if job.accepted_freelancer != Some(freelancer) {
            panic!("Only the accepted freelancer can complete the job");
        }

        if job.status != JobStatus::InProgress {
            panic!("Job is not in progress");
        }

        job.complete();
        jobs.set(job_id.clone(), job);
        env.storage().instance().set(&DataKey::Jobs, &jobs);
    }

    pub fn approve_job(env: Env, employer: Address, job_id: JobId) {
        let mut jobs: Map<JobId, Job> = env.storage().instance().get(&DataKey::Jobs).unwrap();
        let job = jobs.get(job_id.clone()).unwrap();

        if job.employer != employer {
            panic!("Only the employer can approve the job");
        }

        if job.status != JobStatus::Completed {
            panic!("Job is not completed yet");
        }

        let mut updated_job = job.clone();
        updated_job.approve();
        jobs.set(job_id.clone(), updated_job);
        env.storage().instance().set(&DataKey::Jobs, &jobs);

        let token_client = token::Client::new(&env, &job.token);
        if let Some(freelancer) = job.accepted_freelancer {
            token_client.transfer(&env.current_contract_address(), &freelancer, &job.budget);
            Self::record_transaction(env.clone(), env.current_contract_address(), freelancer, job.budget, Symbol::new(&env, "Job payment"));
        }
    }

    pub fn get_job(env: Env, job_id: JobId) -> Job {
        let jobs: Map<JobId, Job> = env.storage().instance().get(&DataKey::Jobs).unwrap();
        jobs.get(job_id).unwrap()
    }

    pub fn get_user(env: Env, address: Address) -> User {
        let users: Map<Address, User> = env.storage().instance().get(&DataKey::Users).unwrap();
        users.get(address).unwrap()
    }

    pub fn get_jobs_by_status(env: Env, status: JobStatus) -> Vec<Job> {
        let jobs: Map<JobId, Job> = env.storage().instance().get(&DataKey::Jobs).unwrap();
        let mut filtered_jobs = Vec::new(&env);
        for job in jobs.values() {
            if job.status == status {
                filtered_jobs.push_back(job);
            }
        }
        filtered_jobs
    }

    pub fn record_transaction(env: Env, from: Address, to: Address, amount: i128, message: Symbol) {
        let mut transactions: Vec<Transaction> = env.storage().instance().get(&DataKey::Transactions).unwrap_or_else(|| Vec::new(&env));
        let mut transaction_counter: u32 = env.storage().instance().get(&DataKey::TransactionCounter).unwrap_or(0);

        transaction_counter += 1;
        let transaction = Transaction {
            id: transaction_counter,
            from,
            to,
            amount,
            message,
            timestamp: env.ledger().timestamp(),
        };

        transactions.push_back(transaction);
        env.storage().instance().set(&DataKey::Transactions, &transactions);
        env.storage().instance().set(&DataKey::TransactionCounter, &transaction_counter);
    }

    pub fn get_transaction_history(env: Env, address: Address) -> Vec<Transaction> {
        let transactions: Vec<Transaction> = env.storage().instance().get(&DataKey::Transactions).unwrap_or_else(|| Vec::new(&env));
        let mut vec = Vec::new(&env);

        for t in transactions.iter() {
            if t.from == address || t.to == address {
                vec.push_back(t.clone());
            }
        }

        vec
    }

    pub fn multi_transfer(env: Env, from: Address, recipients: Vec<(Address, i128)>, token: Address, message: Symbol) {
        let token_client = token::Client::new(&env, &token);

        for (to, amount) in recipients.iter() {
            token_client.transfer(&from, &to, &amount);
            Self::record_transaction(env.clone(), from.clone(), to.clone(), amount, message.clone());
        }
    }

    pub fn create_regular_payment(env: Env, from: Address, to: Address, amount: i128, interval: u64, message: Symbol) -> PaymentId {
        let mut regular_payments: Vec<RegularPayment> = env.storage().instance().get(&DataKey::RegularPayments).unwrap_or_else(|| Vec::new(&env));
        let mut payment_id_counter: u32 = env.storage().instance().get(&DataKey::RegularPaymentCounter).unwrap_or(0);

        payment_id_counter += 1;
        let payment_id = Symbol::new(&env, &payment_id_counter.to_string());
        let next_payment = env.ledger().timestamp() + interval;

        let payment = RegularPayment {
            id: payment_id.clone(),
            from,
            to,
            amount,
            interval,
            next_payment,
            message,
        };

        regular_payments.push_back(payment);
        env.storage().instance().set(&DataKey::RegularPayments, &regular_payments);
        env.storage().instance().set(&DataKey::RegularPaymentCounter, &payment_id_counter);

        payment_id
    }

    pub fn execute_regular_payments(env: Env, token: Address) {
        let regular_payments: Vec<RegularPayment> = env.storage().instance().get(&DataKey::RegularPayments).unwrap_or_else(|| Vec::new(&env));
        let current_time = env.ledger().timestamp();
        let token_client = token::Client::new(&env, &token);

        let mut updated_payments = Vec::new(&env);
        for i in 0..regular_payments.len() {
            let mut payment = regular_payments.get(i).unwrap();
            if current_time >= payment.next_payment {
                token_client.transfer(&payment.from, &payment.to, &payment.amount);
                Self::record_transaction(env.clone(), payment.from.clone(), payment.to.clone(), payment.amount, payment.message.clone());
                payment.next_payment += payment.interval;
            }
            updated_payments.push_back(payment);
        }

        env.storage().instance().set(&DataKey::RegularPayments, &updated_payments);
    }

    pub fn get_balance(env: Env, address: Address, token: Address) -> i128 {
        let token_client = token::Client::new(&env, &token);
        token_client.balance(&address)
    }

    pub fn create_campaign(env: Env, creator: Address, title: Symbol, description: Symbol, goal: i128, duration: u64, token: Address, tiers: Vec<FundingTier>) -> CampaignId {
        let mut campaigns: Map<CampaignId, Campaign> = env.storage().instance().get(&DataKey::Campaigns).unwrap_or_else(|| Map::new(&env));
        let mut campaign_id_counter: u32 = env.storage().instance().get(&DataKey::CampaignIdCounter).unwrap_or(0);

        campaign_id_counter += 1;
        let campaign_id = Symbol::new(&env, &campaign_id_counter.to_string());
        let end_time = env.ledger().timestamp() + duration;

        let campaign = Campaign {
            id: campaign_id.clone(),
            creator,
            title,
            description,
            goal,
            raised: 0,
            end_time,
            token,
            status: CampaignStatus::Active,
            backers: Vec::new(&env),
            tiers,
        };

        campaigns.set(campaign_id.clone(), campaign);
        env.storage().instance().set(&DataKey::Campaigns, &campaigns);
        env.storage().instance().set(&DataKey::CampaignIdCounter, &campaign_id_counter);

        campaign_id
    }

    pub fn fund_campaign(env: Env, backer: Address, campaign_id: CampaignId, amount: i128) {
        let mut campaigns: Map<CampaignId, Campaign> = env.storage().instance().get(&DataKey::Campaigns).unwrap();
        let mut campaign = campaigns.get(campaign_id.clone()).unwrap();

        if campaign.status != CampaignStatus::Active {
            panic!("Campaign is not active");
        }

        if env.ledger().timestamp() > campaign.end_time {
            panic!("Campaign has ended");
        }

        let token_client = token::Client::new(&env, &campaign.token);
        token_client.transfer(&backer, &env.current_contract_address(), &amount);

        campaign.raised += amount;
        campaign.backers.push_back(backer.clone());

        if campaign.raised >= campaign.goal {
            campaign.status = CampaignStatus::Successful;
        }

        campaigns.set(campaign_id.clone(), campaign);
        env.storage().instance().set(&DataKey::Campaigns, &campaigns);

        Self::record_transaction(env.clone(), backer, env.current_contract_address(), amount, Symbol::new(&env, "Campaign funding"));
    }

    pub fn close_campaign(env: Env, campaign_id: CampaignId) {
        let mut campaigns: Map<CampaignId, Campaign> = env.storage().instance().get(&DataKey::Campaigns).unwrap();
        let mut campaign = campaigns.get(campaign_id.clone()).unwrap();

        if env.ledger().timestamp() <= campaign.end_time && campaign.status == CampaignStatus::Active {
            panic!("Campaign is still active");
        }

        if campaign.status == CampaignStatus::Active {
            campaign.status = if campaign.raised >= campaign.goal {
                CampaignStatus::Successful
            } else {
                CampaignStatus::Failed
            };
        }

        let token_client = token::Client::new(&env, &campaign.token);

        if campaign.status == CampaignStatus::Successful {
            token_client.transfer(&env.current_contract_address(), &campaign.creator, &campaign.raised);
            Self::record_transaction(env.clone(), env.current_contract_address(), campaign.creator.clone(), campaign.raised, Symbol::new(&env, "Campaign payout"));
        } else if campaign.status == CampaignStatus::Failed {
            for backer in campaign.backers.iter() {
                let backer_contribution = Self::get_backer_contribution(&env, &campaign_id, &backer);
                token_client.transfer(&env.current_contract_address(), &backer, &backer_contribution);
                Self::record_transaction(env.clone(), env.current_contract_address(), backer.clone(), backer_contribution, Symbol::new(&env, "Campaign refund"));
            }
        }

        campaign.status = CampaignStatus::Closed;
        campaigns.set(campaign_id.clone(), campaign);
        env.storage().instance().set(&DataKey::Campaigns, &campaigns);
    }

    fn get_backer_contribution(env: &Env, _campaign_id: &CampaignId, backer: &Address) -> i128 {
        let transactions: Vec<Transaction> = env.storage().instance().get(&DataKey::Transactions).unwrap_or_else(|| Vec::new(env));
        transactions.iter()
            .filter(|t| t.to == env.current_contract_address() && t.from == *backer && t.message == Symbol::new(env, "Campaign funding"))
            .map(|t| t.amount)
            .sum()
    }

    pub fn get_campaign(env: Env, campaign_id: CampaignId) -> Campaign {
        let campaigns: Map<CampaignId, Campaign> = env.storage().instance().get(&DataKey::Campaigns).unwrap();
        campaigns.get(campaign_id).unwrap()
    }

    pub fn get_campaigns_by_status(env: Env, status: CampaignStatus) -> Vec<Campaign> {
        let campaigns: Map<CampaignId, Campaign> = env.storage().instance().get(&DataKey::Campaigns).unwrap();
        let mut filtered_campaigns = Vec::new(&env);
        for campaign in campaigns.values() {
            if campaign.status == status {
                filtered_campaigns.push_back(campaign);
            }
        }
        filtered_campaigns
    }
}
