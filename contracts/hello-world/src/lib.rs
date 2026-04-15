#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// STUDENT GRADE
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grade {
    pub id: u64,
    pub student_name: String,
    pub course: String,      
    pub grade: u32,          
    pub created_at: u64,
    pub updated_at: u64,
}

const GRADE_DATA: Symbol = symbol_short!("GRD_DATA");

#[contract]
pub struct GradeLogContract;

#[contractimpl]
impl GradeLogContract {
    /// CREATE - Tambah data nilai mahasiswa
    pub fn add_grade(env: Env, student_name: String, course: String, grade: u32) -> u64 {
        if grade > 100 {
            panic!("grade harus dari range 0-100");
        }

        let mut grades: Vec<Grade> = env
            .storage()
            .instance()
            .get(&GRADE_DATA)
            .unwrap_or(Vec::new(&env));

        let id = env.prng().gen::<u64>();
        let now = env.ledger().timestamp();

        let record = Grade {
            id,
            student_name,
            course,
            grade,
            created_at: now,
            updated_at: now,
        };

        grades.push_back(record);
        env.storage().instance().set(&GRADE_DATA, &grades);

        id
    }

    /// READ - Ambil semua data nilai
    pub fn get_all_grades(env: Env) -> Vec<Grade> {
        env.storage()
            .instance()
            .get(&GRADE_DATA)
            .unwrap_or(Vec::new(&env))
    }

    /// READ - Ambil data nilai berdasarkan ID
    pub fn get_grade_by_id(env: Env, id: u64) -> Option<Grade> {
        let grades: Vec<Grade> = env
            .storage()
            .instance()
            .get(&GRADE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..grades.len() {
            let g = grades.get(i).unwrap();
            if g.id == id {
                return Some(g);
            }
        }

        None
    }

    /// READ - Filter berdasarkan mata kuliah
    pub fn get_grades_by_course(env: Env, course: String) -> Vec<Grade> {
        let grades: Vec<Grade> = env
            .storage()
            .instance()
            .get(&GRADE_DATA)
            .unwrap_or(Vec::new(&env));
        let mut filtered = Vec::new(&env);

        for i in 0..grades.len() {
            let g = grades.get(i).unwrap();
            if g.course == course {
                filtered.push_back(g);
            }
        }

        filtered
    }

    /// UPDATE - Ubah nilai berdasarkan ID
    pub fn update_grade(env: Env, id: u64, new_grade: u32) -> bool {
        if new_grade > 100 {
            panic!("grade harus dari range 0-100");
        }

        let mut grades: Vec<Grade> = env
            .storage()
            .instance()
            .get(&GRADE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..grades.len() {
            let mut g = grades.get(i).unwrap();
            if g.id == id {
                g.grade = new_grade;
                g.updated_at = env.ledger().timestamp();
                grades.set(i, g);
                env.storage().instance().set(&GRADE_DATA, &grades);
                return true;
            }
        }

        false
    }

    /// DELETE - Hapus data berdasarkan ID
    pub fn remove_grade(env: Env, id: u64) -> bool {
        let mut grades: Vec<Grade> = env
            .storage()
            .instance()
            .get(&GRADE_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..grades.len() {
            if grades.get(i).unwrap().id == id {
                grades.remove(i);
                env.storage().instance().set(&GRADE_DATA, &grades);
                return true;
            }
        }

        false
    }
}