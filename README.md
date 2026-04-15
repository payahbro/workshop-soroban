# Grade Log Smart Contract (Soroban + Stellar)

Smart contract ini digunakan untuk mencatat **nilai mahasiswa** di blockchain Stellar menggunakan **Soroban**.  
Kontrak menyediakan fitur **CRUD** (Create, Read, Update, Delete) untuk data nilai.

## ✨ Fitur

- **Create**: Tambah data nilai mahasiswa (`add_grade`)
- **Read**:
  - Ambil semua nilai (`get_all_grades`)
  - Ambil nilai berdasarkan ID (`get_grade_by_id`)
  - Filter nilai berdasarkan mata kuliah (`get_grades_by_course`)
- **Update**: Ubah nilai berdasarkan ID (`update_grade`)
- **Delete**: Hapus nilai berdasarkan ID (`remove_grade`)

---

## 🧱 Struktur Data

```rust
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grade {
    pub id: u64,
    pub student_name: String,
    pub course: String,
    pub grade: u32,      // 0 - 100
    pub created_at: u64, // timestamp ledger
    pub updated_at: u64, // timestamp ledger
}
```

Data disimpan di storage instance dengan key:

- `GRD_DATA`

---

## ⚙️ Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Soroban SDK](https://soroban.stellar.org/docs)
- [Stellar](https://stellar.org/) (Testnet)

---

## 📦 Prerequisites

Pastikan sudah install:

1. Rust + cargo
2. Soroban CLI
3. Stellar account untuk testnet (untuk deploy dan invoke)

Contoh cek versi:

```bash
rustc --version
cargo --version
soroban --version
```

---

## 🚀 Build Contract

```bash
cargo build --target wasm32v1-none --release
```

Output wasm biasanya ada di:

`target/wasm32v1-none/release/<nama_contract>.wasm`

---

## 🌐 Deploy ke Stellar Testnet

> Ganti placeholder sesuai project kamu.

### 1) Set network testnet
```bash
soroban network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

### 2) Generate identity
```bash
soroban keys generate alice --network testnet
```

### 3) Fund account (friendbot)
```bash
soroban keys fund alice --network testnet
```

### 4) Deploy contract
```bash
soroban contract deploy \
  --wasm target/wasm32v1-none/release/<nama_contract>.wasm \
  --source alice \
  --network testnet
```

Simpan **contract ID** yang dihasilkan.

---

## 🧪 Contoh Invoke Function

> Ganti `<CONTRACT_ID>` dengan ID contract hasil deploy.

### Create - add_grade
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- add_grade \
  --student_name "Budi" \
  --course "Blockchain" \
  --grade 90
```

### Read - get_all_grades
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- get_all_grades
```

### Read - get_grade_by_id
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- get_grade_by_id \
  --id 12345
```

### Read - get_grades_by_course
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- get_grades_by_course \
  --course "Blockchain"
```

### Update - update_grade
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- update_grade \
  --id 12345 \
  --new_grade 95
```

### Delete - remove_grade
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- remove_grade \
  --id 12345
```

---

## 🛡️ Validasi

Kontrak memiliki validasi:
- `grade` harus di rentang **0 - 100**  
- jika `grade > 100`, kontrak akan `panic`

---

## 📚 Daftar Method

| Method | Deskripsi | Return |
|---|---|---|
| `add_grade(student_name, course, grade)` | Menambah data nilai baru | `u64` (id) |
| `get_all_grades()` | Mengambil semua data nilai | `Vec<Grade>` |
| `get_grade_by_id(id)` | Mengambil 1 data nilai berdasarkan id | `Option<Grade>` |
| `get_grades_by_course(course)` | Filter data berdasarkan mata kuliah | `Vec<Grade>` |
| `update_grade(id, new_grade)` | Update nilai berdasarkan id | `bool` |
| `remove_grade(id)` | Hapus data berdasarkan id | `bool` |

---