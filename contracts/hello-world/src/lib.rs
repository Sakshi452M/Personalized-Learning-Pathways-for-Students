#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, symbol_short, log};

#[contracttype]
#[derive(Clone)]
pub struct LearningPath {
    pub student_id: u64,
    pub name: String,
    pub interest_area: String,
    pub path_assigned: String,
}

#[contracttype]
pub enum Pathbook {
    Entry(u64),
}

const STUDENT_COUNT: Symbol = symbol_short!("STU_CNT");

#[contract]
pub struct LearningPathContract;

#[contractimpl]
impl LearningPathContract {
    pub fn register_student(env: Env, name: String, interest_area: String, path: String) -> u64 {
        let mut student_count: u64 = env.storage().instance().get(&STUDENT_COUNT).unwrap_or(0);
        student_count += 1;

        let new_entry = LearningPath {
            student_id: student_count,
            name,
            interest_area,
            path_assigned: path,
        };

        env.storage().instance().set(&Pathbook::Entry(student_count), &new_entry);
        env.storage().instance().set(&STUDENT_COUNT, &student_count);
        log!(&env, "Student registered with ID: {}", student_count);

        student_count
    }

    pub fn get_learning_path(env: Env, student_id: u64) -> LearningPath {
        env.storage()
            .instance()
            .get(&Pathbook::Entry(student_id))
            .unwrap_or(LearningPath {
                student_id: 0,
                name: String::from_str(&env, "Unknown"),
                interest_area: String::from_str(&env, "Unknown"),
                path_assigned: String::from_str(&env, "None"),
            })
    }
}

