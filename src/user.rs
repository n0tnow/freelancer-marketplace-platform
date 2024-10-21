use soroban_sdk::{contracttype, Address, Env, Vec};
use crate::types::JobId;

#[derive(Clone, Debug)]
#[contracttype]
pub struct User {
    pub address: Address,
    pub role: UserRole,
    pub jobs: Vec<JobId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum UserRole {
    Employer,
    Freelancer,
}

impl User {
    pub fn new(env: &Env, address: Address, role: UserRole) -> Self {
        Self {
            address,
            role,
            jobs: Vec::new(env),
        }
    }

    pub fn add_job(&mut self, job_id: JobId) {
        self.jobs.push_back(job_id);
    }
}