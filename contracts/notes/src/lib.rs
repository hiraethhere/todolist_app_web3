#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// 1. Definisikan struktur data To-Do (Note)
#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub content: String,
}

// 2. Kunci penyimpanan untuk di blockchain
const NOTE_DATA: Symbol = symbol_short!("NOTE_DATA");

#[contract]
pub struct NotesContract;

#[contractimpl]
impl NotesContract {
    // Fungsi untuk membaca semua to-do list
    pub fn get_notes(env: Env) -> Vec<Note> {
        env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env))
    }

    // Fungsi untuk menambah tugas baru
    pub fn create_note(env: Env, title: String, content: String) -> String {
        let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
        
        let note = Note {
            id: env.prng().gen::<u64>(), // Generate ID unik acak
            title,
            content,
        };
        
        notes.push_back(note);
        env.storage().instance().set(&NOTE_DATA, &notes);
        
        String::from_str(&env, "Tugas berhasil ditambahkan")
    }

    // Fungsi untuk menghapus tugas yang sudah selesai
    pub fn delete_note(env: Env, id: u64) -> String {
        let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

        for i in 0..notes.len() {
            if notes.get(i).unwrap().id == id {
                notes.remove(i);
                env.storage().instance().set(&NOTE_DATA, &notes);
                return String::from_str(&env, "Tugas berhasil dihapus");
            }
        }

        String::from_str(&env, "Tugas tidak ditemukan")
    }
}