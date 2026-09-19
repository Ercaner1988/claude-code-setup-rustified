//! SQLite destekli arama yollari: FTS5 anahtar kelime aramasi, semantik
//! siralama ve graf uzerinde 2 adimlik BFS. Veritabani bellekte kurulur,
//! boylece olcum disk I/O'suna bagli olmaz.

use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};

use claude_code_setup::memory_engine::{
    bytes_to_f32_vec, cosine_similarity, escape_fts5_query, f32_vec_to_bytes, init_db,
};
use rusqlite::{params, Connection};

fn main() {
    divan::main();
}

const EMBEDDING_DIM: usize = 384;
const NOTE_COUNT: usize = 500;

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn next_f32(&mut self) -> f32 {
        ((self.next_u64() >> 40) as f32 / (1u32 << 24) as f32) * 2.0 - 1.0
    }
}

const TOPICS: &[&str] = &[
    "mcp sunucu yapilandirmasi",
    "semantik arama gomme vektoru",
    "guvenlik denetimi gizli anahtar",
    "graf hafiza kenarlari",
    "claude code ortam denetimi",
];

/// NOTE_COUNT adet sentetik notu FTS5 tablosu, gomme blob'lari ve graf
/// kenarlari ile birlikte bellekteki bir veritabanina yazar.
fn seeded_db() -> Connection {
    let conn = Connection::open_in_memory().expect("in-memory sqlite");
    init_db(&conn).expect("schema");

    let mut rng = Rng::new(0x5EED);
    for i in 0..NOTE_COUNT {
        let filename = format!("not-{i:04}.md");
        let topic = TOPICS[i % TOPICS.len()];
        let title = format!("Not {i} - {topic}");
        let content = format!(
            "# {title}\n\n{topic} hakkinda ayrintili notlar. \
             Iliskili notlar: [[not-{:04}]] ve [[not-{:04}]].\n",
            (i + 1) % NOTE_COUNT,
            (i + 7) % NOTE_COUNT
        );
        let embedding: Vec<f32> = (0..EMBEDDING_DIM).map(|_| rng.next_f32()).collect();

        conn.execute(
            "INSERT INTO knowledge_notes (filename, title, content, embedding) VALUES (?1, ?2, ?3, ?4)",
            params![filename, title, content, f32_vec_to_bytes(&embedding)],
        )
        .expect("insert note");
        conn.execute(
            "INSERT INTO knowledge_fts (filename, title, content) VALUES (?1, ?2, ?3)",
            params![filename, title, content],
        )
        .expect("insert fts");

        for offset in [1usize, 7] {
            conn.execute(
                "INSERT OR IGNORE INTO note_edges (src, dst, tur, agirlik) VALUES (?1, ?2, ?3, ?4)",
                params![
                    filename,
                    format!("not-{:04}.md", (i + offset) % NOTE_COUNT),
                    "wikilink",
                    1.0_f64
                ],
            )
            .expect("insert edge");
        }
    }

    conn
}

thread_local! {
    static DB: RefCell<Option<Connection>> = const { RefCell::new(None) };
}

/// Tohumlanmis veritabanini yalnizca bir kez kurar, sonra yeniden kullanir.
fn with_db<T>(f: impl FnOnce(&Connection) -> T) -> T {
    DB.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.is_none() {
            *slot = Some(seeded_db());
        }
        f(slot.as_ref().expect("db"))
    })
}

/// FTS5 anahtar kelime aramasi (search_keyword_vec ile ayni sorgu).
#[divan::bench]
fn keyword_search(bencher: divan::Bencher) {
    with_db(|conn| {
        bencher.bench_local(|| {
            let escaped = escape_fts5_query(divan::black_box("semantik arama gomme"));
            let mut stmt = conn
                .prepare(
                    "SELECT filename, title, rank FROM knowledge_fts \
                     WHERE knowledge_fts MATCH ?1 ORDER BY rank",
                )
                .expect("prepare");
            let rows = stmt
                .query_map(params![escaped], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, f64>(2)?,
                    ))
                })
                .expect("query");
            let mut out = Vec::new();
            for row in rows {
                out.push(row.expect("row"));
                if out.len() >= 10 {
                    break;
                }
            }
            out
        });
    });
}

/// Semantik siralama: tum gommeleri oku, kosinus ile puanla, en iyi 10'u al
/// (search_semantic_vec'in model cagrisi olmadan olculen kismi).
#[divan::bench]
fn semantic_ranking(bencher: divan::Bencher) {
    let mut rng = Rng::new(0xC0FFEE);
    let query: Vec<f32> = (0..EMBEDDING_DIM).map(|_| rng.next_f32()).collect();

    with_db(|conn| {
        bencher.bench_local(|| {
            let mut stmt = conn
                .prepare("SELECT filename, title, embedding FROM knowledge_notes")
                .expect("prepare");
            let rows = stmt
                .query_map([], |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<Vec<u8>>>(2)?,
                    ))
                })
                .expect("query");

            let mut scored: Vec<(String, f64)> = Vec::new();
            for row in rows {
                let (filename, _title, blob) = row.expect("row");
                if let Some(blob) = blob {
                    let embedding = bytes_to_f32_vec(&blob);
                    scored.push((
                        filename,
                        cosine_similarity(divan::black_box(&query), &embedding) as f64,
                    ));
                }
            }
            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            scored.truncate(10);
            scored
        });
    });
}

/// Graf uzerinde 2 adimlik BFS (get_related_notes'un gezinme mantigi).
#[divan::bench]
fn graph_related_notes(bencher: divan::Bencher) {
    with_db(|conn| {
        bencher.bench_local(|| {
            let start = divan::black_box("not-0042.md").to_string();
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((start.clone(), 0usize));
            visited.insert(start);

            let mut stmt = conn
                .prepare("SELECT dst, tur, agirlik FROM note_edges WHERE src = ?1")
                .expect("prepare");

            while let Some((curr, dist)) = queue.pop_front() {
                if dist >= 2 {
                    continue;
                }
                let rows = stmt
                    .query_map(params![curr], |row| row.get::<_, String>(0))
                    .expect("query");
                for row in rows {
                    let dst = row.expect("row");
                    if visited.insert(dst.clone()) {
                        queue.push_back((dst, dist + 1));
                    }
                }
            }
            visited.len()
        });
    });
}
