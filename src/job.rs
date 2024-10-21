use soroban_sdk::{contracttype, Address, Env, Symbol, Vec};
use crate::types::JobId;

#[derive(Clone, Debug)]
#[contracttype]
pub struct Job {
    pub id: JobId,
    pub employer: Address,
    pub title: Symbol,
    pub description: Symbol,
    pub budget: i128,
    pub token: Address,
    pub status: JobStatus,
    pub proposals: Vec<Proposal>,
    pub accepted_freelancer: Option<Address>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum JobStatus {
    Open,
    InProgress,
    Completed,
    Approved,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Proposal {
    pub freelancer: Address,
    pub price: i128,
}

impl Job {
    pub fn new(env: &Env, id: JobId, employer: Address, title: Symbol, description: Symbol, budget: i128, token: Address) -> Self {
        Self {
            id,
            employer,
            title,
            description,
            budget,
            token,
            status: JobStatus::Open,
            proposals: Vec::new(env),
            accepted_freelancer: None,
        }
    }

    pub fn add_proposal(&mut self, freelancer: Address, price: i128) {
        self.proposals.push_back(Proposal { freelancer, price });
    }

    pub fn accept_proposal(&mut self, freelancer: Address) {
        self.status = JobStatus::InProgress;
        self.accepted_freelancer = Some(freelancer);
    }

    pub fn complete(&mut self) {
        self.status = JobStatus::Completed;
    }

    pub fn approve(&mut self) {
        self.status = JobStatus::Approved;
    }
}