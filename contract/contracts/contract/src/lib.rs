#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec, Map};

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u32,
    pub name: Symbol,
    pub vote_count: u32,
}

#[contracttype]
pub enum DataKey {
    Proposal(u32),
    ProposalCount,
    HasVoted(Address),
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {

    // Initialize proposal counter
    pub fn initialize(env: Env) {
        env.storage().instance().set(&DataKey::ProposalCount, &0u32);
    }

    // Create a new proposal
    pub fn create_proposal(env: Env, name: Symbol) -> u32 {
        let mut count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0);

        count += 1;

        let proposal = Proposal {
            id: count,
            name,
            vote_count: 0,
        };

        env.storage().instance().set(&DataKey::Proposal(count), &proposal);
        env.storage().instance().set(&DataKey::ProposalCount, &count);

        count
    }

    // Vote for a proposal
    pub fn vote(env: Env, voter: Address, proposal_id: u32) {
        voter.require_auth();

        // Ensure voter hasn't voted before
        if env
            .storage()
            .instance()
            .has(&DataKey::HasVoted(voter.clone()))
        {
            panic!("Already voted");
        }

        let mut proposal: Proposal = env
            .storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .expect("Proposal not found");

        proposal.vote_count += 1;

        env.storage().instance().set(&DataKey::Proposal(proposal_id), &proposal);
        env.storage().instance().set(&DataKey::HasVoted(voter), &true);
    }

    // Get proposal details
    pub fn get_proposal(env: Env, proposal_id: u32) -> Proposal {
        env.storage()
            .instance()
            .get(&DataKey::Proposal(proposal_id))
            .expect("Proposal not found")
    }

    // Get total proposals
    pub fn get_proposal_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::ProposalCount)
            .unwrap_or(0)
    }
}