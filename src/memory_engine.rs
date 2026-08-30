use anyhow::{bail, Context, Result};
use colored::*;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use regex::Regex;
use rusqlite::{params, Connection};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::PathBuf;

use crate::branch_manager::sanitize_description;
use crate::installer::get_home_dir;

// ─── Yardımcılar ────────────────────────────────────────────────────────────

pub fn get_db_path(home_override: Option<String>) -> Result<PathBuf> {
    let home = get_home_dir(home_override)?;
    let claude_dir = home.join(".claude");
    fs::create_dir_all(&claude_dir)?;
    Ok(claude_dir.join("memory_index.db"))
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    for (x, y) in a.iter().zip(b.iter()) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a.sqrt() * norm_b.sqrt())
    }
}

fn bytes_to_f32_vec(bytes: &[u8]) -> Vec<f32> {
    bytes
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
        .collect()
}

fn f32_vec_to_bytes(vec: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(vec.len() * 4);
    for &val in vec {
        bytes.extend_from_slice(&val.to_le_bytes());
    }
    bytes
}

/// Wikilink'leri ayrıştır: `[[hedef]]` → "hedef" veya "hedef.md"
pub fn extract_wikilinks(content: &str) -> Vec<String> {
    let re = Regex::new(r"\[\[(.*?)\]\]").unwrap();
    re.captures_iter(content)
        .map(|cap| {
            let target = cap[1].trim();
            if target.ends_with(".md") {
                target.to_string()
            } else {
                format!("{}.md", target)
            }
        })
        .collect()
}

/// İçeriği ~chunk_size karakterlik pencerelere böl (satır sınırında)
fn chunk_content(content: &str, chunk_size: usize) -> Vec<String> {
    if content.len() <= chunk_size {
        return vec![content.to_string()];
    }
    let mut chunks = Vec::new();
    let mut start = 0;
    while start < content.len() {
        let mut end = std::cmp::min(start + chunk_size, content.len());
        // UTF-8 karakter sınırına hizala — Türkçe/çok baytlı metinde panik olmasın
        while end < content.len() && !content.is_char_boundary(end) {
            end += 1;
        }
        // Satır sınırında kes
        let actual_end = if end < content.len() {
            content[start..end]
                .rfind('\n')
                .map(|pos| start + pos + 1)
                .unwrap_or(end)
        } else {
            end
        };
        chunks.push(content[start..actual_end].to_string());
        start = actual_end;
    }
    chunks
}

/// Chunk embedding'lerinin ortalamasını al (mean-pool)
fn mean_pool_embeddings(embeddings: &[Vec<f32>]) -> Vec<f32> {
    if embeddings.is_empty() {
        return Vec::new();
    }
    let dim = embeddings[0].len();
    let mut mean = vec![0.0f32; dim];
    for emb in embeddings {
        for (i, &v) in emb.iter().enumerate() {
            mean[i] += v;
        }
    }
    let n = embeddings.len() as f32;
    for val in mean.iter_mut() {
        *val /= n;
    }
    mean
}

/// FTS5 sorgusu için güvenli kaçırma: her sözcüğü çift tırnağa al
fn escape_fts5_query(query: &str) -> String {
    query
        .split_whitespace()
        .map(|word| format!("\"{}\"", word))
        .collect::<Vec<_>>()
        .join(" ")
}

// ─── Sonuç tipi ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub filename: String,
    pub title: String,
    pub score: f64,
}

fn render_results(results: &[SearchResult], mode_label: &str, query: &str) {
    println!("{} for '{}'", mode_label.cyan().bold(), query.yellow());
    println!("========================================");
    for r in results {
        println!(
            "• {} [{}] (Score: {:.4})",
            r.title.green().bold(),
            r.filename.dimmed(),
            r.score
        );
    }
    println!("========================================");
    println!("Total matched documents: {}", results.len());
}

// ─── DB Başlatma ────────────────────────────────────────────────────────────

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS knowledge_notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            filename TEXT UNIQUE NOT NULL,
            title TEXT,
            content TEXT NOT NULL,
            embedding BLOB
        )",
        [],
    )?;

    // FTS5 Sanal Tablosu
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS knowledge_fts USING fts5(
            filename UNINDEXED,
            title,
            content
        )",
        [],
    )?;

    // Graph Kenarları Tablosu
    conn.execute(
        "CREATE TABLE IF NOT EXISTS note_edges (
            src TEXT NOT NULL,
            dst TEXT NOT NULL,
            tur TEXT NOT NULL,
            agirlik REAL NOT NULL,
            PRIMARY KEY (src, dst, tur)
        )",
        [],
    )?;

    Ok(())
}

// ─── Not Ekleme ─────────────────────────────────────────────────────────────

/// Varsayılan knowledge dizini: <home>/claude_global_memory/knowledge
pub fn default_knowledge_dir(home_override: Option<String>) -> Result<PathBuf> {
    let home = get_home_dir(home_override)?;
    Ok(home.join("claude_global_memory").join("knowledge"))
}

/// Yeni bir markdown notu ekler. Dosya adı başlıktan kebab-case türetilir.
/// Var olan dosyanın üzerine ASLA yazmaz. Dizini döndürür.
pub fn add_memory_note(
    title: &str,
    body: Option<&str>,
    dir_override: Option<String>,
    home_override: Option<String>,
) -> Result<PathBuf> {
    let dir = match dir_override {
        Some(d) => PathBuf::from(d),
        None => default_knowledge_dir(home_override)?,
    };
    fs::create_dir_all(&dir)
        .with_context(|| format!("Failed to create knowledge directory {:?}", dir))?;

    let slug = sanitize_description(title);
    if slug.is_empty() {
        bail!("Title '{}' does not produce a usable filename", title);
    }
    let path = dir.join(format!("{}.md", slug));
    if path.exists() {
        bail!(
            "Note already exists: {:?}. Choose a different title or edit the file directly.",
            path
        );
    }

    let content = match body {
        Some(b) if !b.trim().is_empty() => format!("# {}\n\n{}\n", title, b),
        _ => format!("# {}\n", title),
    };
    fs::write(&path, content).with_context(|| format!("Failed to write note {:?}", path))?;

    println!(
        "{} Created note {} (run memory-index to make it searchable)",
        "✓".green().bold(),
        path.display().to_string().cyan()
    );
    Ok(path)
}

// ─── İndeksleme ─────────────────────────────────────────────────────────────

pub fn index_memory(
    home_override: Option<String>,
    edge_threshold: f32,
    sources: Vec<String>,
) -> Result<()> {
    let db_path = get_db_path(home_override.clone())?;

    // --source verilmezse eski davranis korunur: <home>/claude_global_memory/knowledge
    let knowledge_dirs: Vec<PathBuf> = if sources.is_empty() {
        vec![default_knowledge_dir(home_override)?]
    } else {
        sources.iter().map(PathBuf::from).collect()
    };

    let existing_dirs: Vec<&PathBuf> = knowledge_dirs.iter().filter(|d| d.exists()).collect();
    if existing_dirs.is_empty() {
        println!(
            "{} No source directory found, skipping index: {:?}",
            "⚠".yellow(),
            knowledge_dirs
        );
        return Ok(());
    }

    let conn = Connection::open(&db_path)
        .with_context(|| format!("Failed to open SQLite database at {:?}", db_path))?;

    init_db(&conn)?;

    conn.execute("DELETE FROM knowledge_notes", [])?;
    conn.execute("DELETE FROM knowledge_fts", [])?;
    conn.execute("DELETE FROM note_edges", [])?;

    let mut files_data = Vec::new();
    let mut seen_names: HashSet<String> = HashSet::new();
    for dir in &existing_dirs {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                // filename wikilink hedefi olarak kullaniliyor -> benzersiz olmak zorunda
                if !seen_names.insert(filename.clone()) {
                    println!(
                        "{} Duplicate filename skipped: {} (in {:?})",
                        "⚠".yellow(),
                        filename,
                        dir
                    );
                    continue;
                }
                let content = fs::read_to_string(&path)?;
                let title = content
                    .lines()
                    .find(|l| l.starts_with("# "))
                    .map(|l| l.trim_start_matches("# ").to_string())
                    .unwrap_or_else(|| filename.clone());

                files_data.push((filename, title, content));
            }
        }
    }

    if files_data.is_empty() {
        println!("{}", "No markdown files found to index.".yellow());
        return Ok(());
    }

    // Dosya adları kümesi — hayalet wikilink filtrelemesi için (B3)
    let file_set: HashSet<String> = files_data.iter().map(|(f, _, _)| f.clone()).collect();

    println!(
        "{}",
        "Generating embeddings via FastEmbed (Multilingual-E5-Small)...".blue()
    );
    let model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::MultilingualE5Small).with_show_download_progress(true),
    )?;

    // B1: Chunking + mean-pool — ~1500 karakter pencereler
    let chunk_size: usize = 1500;
    let mut all_embeddings: Vec<Vec<f32>> = Vec::with_capacity(files_data.len());

    for (_filename, title, content) in files_data.iter() {
        let full_text = format!("{}\n{}", title, content);
        let chunks = chunk_content(&full_text, chunk_size);
        let chunk_embeddings = model.embed(chunks, None)?;
        let pooled = mean_pool_embeddings(&chunk_embeddings);
        all_embeddings.push(pooled);
    }

    // DB'ye Ekle
    for (i, (filename, title, content)) in files_data.iter().enumerate() {
        let emb_bytes = f32_vec_to_bytes(&all_embeddings[i]);
        conn.execute(
            "INSERT INTO knowledge_notes (filename, title, content, embedding) VALUES (?1, ?2, ?3, ?4)",
            params![filename, title, content, emb_bytes],
        )?;

        conn.execute(
            "INSERT INTO knowledge_fts (filename, title, content) VALUES (?1, ?2, ?3)",
            params![filename, title, content],
        )?;
    }

    // Wikilink & Semantik Kenarları Oluştur
    let mut skipped_wikilinks = 0u32;
    for (i, (src_file, _, content)) in files_data.iter().enumerate() {
        // 1. Wikilinks — B3: yalnız var olan hedefe kenar yaz
        let links = extract_wikilinks(content);
        for dst_file in &links {
            if file_set.contains(dst_file) {
                conn.execute(
                    "INSERT OR REPLACE INTO note_edges (src, dst, tur, agirlik) VALUES (?1, ?2, 'wikilink', 1.0)",
                    params![src_file, dst_file],
                )?;
            } else {
                skipped_wikilinks += 1;
            }
        }

        // 2. Kosinüs Semantik Kenarlar
        // ponytail: lineer kosinüs; not > ~5k olursa ANN ekle
        for j in (i + 1)..files_data.len() {
            let sim = cosine_similarity(&all_embeddings[i], &all_embeddings[j]);
            if sim >= edge_threshold {
                let dst_file = &files_data[j].0;
                conn.execute(
                    "INSERT OR REPLACE INTO note_edges (src, dst, tur, agirlik) VALUES (?1, ?2, 'semantic', ?3)",
                    params![src_file, dst_file, sim],
                )?;
                conn.execute(
                    "INSERT OR REPLACE INTO note_edges (src, dst, tur, agirlik) VALUES (?1, ?2, 'semantic', ?3)",
                    params![dst_file, src_file, sim],
                )?;
            }
        }
    }

    if skipped_wikilinks > 0 {
        println!(
            "{} {} ghost wikilink edge(s) skipped (target note not found).",
            "⚠".yellow(),
            skipped_wikilinks
        );
    }

    println!(
        "{} Successfully indexed {} notes with embeddings and graph edges into SQLite ({:?})",
        "✓".green().bold(),
        files_data.len(),
        db_path
    );

    Ok(())
}

// ─── Arama ──────────────────────────────────────────────────────────────────

pub fn search_memory(
    query: &str,
    mode: &str,
    home_override: Option<String>,
    limit: usize,
    min_score: f64,
) -> Result<()> {
    let db_path = get_db_path(home_override.clone())?;
    if !db_path.exists() {
        println!(
            "{} Memory database not found, run memory-index first.",
            "⚠".yellow()
        );
        return Ok(());
    }

    let conn = Connection::open(&db_path)?;

    match mode {
        "keyword" => {
            let results = search_keyword_vec(&conn, query, limit, min_score)?;
            render_results(&results, "Memory Keyword Search (FTS5)", query);
        }
        "semantic" => {
            let results = search_semantic_vec(&conn, query, limit, min_score)?;
            render_results(&results, "Memory Semantic Search", query);
        }
        _ => {
            // A1: Gerçek hybrid — RRF (k=60)
            let results = search_hybrid_rrf(&conn, query, limit, min_score)?;
            render_results(
                &results,
                "Memory Hybrid Search (RRF: FTS5 + Semantic)",
                query,
            );
        }
    }
    Ok(())
}

fn search_keyword_vec(
    conn: &Connection,
    query: &str,
    limit: usize,
    min_score: f64,
) -> Result<Vec<SearchResult>> {
    // A2: FTS5 kaçırma — her sözcüğü çift tırnağa al
    let escaped = escape_fts5_query(query);

    let mut stmt = conn.prepare(
        "SELECT filename, title, rank FROM knowledge_fts WHERE knowledge_fts MATCH ?1 ORDER BY rank",
    )?;

    let rows = stmt.query_map(params![escaped], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, f64>(2)?,
        ))
    })?;

    let mut results = Vec::new();
    for row in rows {
        let (filename, title, rank) = row?;
        // FTS5 rank negatif (daha negatif = daha iyi); normalize edip pozitife çeviriyoruz
        let score = -rank;
        if score >= min_score || min_score <= 0.0 {
            results.push(SearchResult {
                filename,
                title,
                score,
            });
        }
        if results.len() >= limit {
            break;
        }
    }

    Ok(results)
}

fn search_semantic_vec(
    conn: &Connection,
    query: &str,
    limit: usize,
    min_score: f64,
) -> Result<Vec<SearchResult>> {
    let model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::MultilingualE5Small).with_show_download_progress(false),
    )?;

    let query_emb = model.embed(vec![query.to_string()], None)?[0].clone();

    let mut stmt = conn.prepare("SELECT filename, title, embedding FROM knowledge_notes")?;
    let mut results: Vec<SearchResult> = Vec::new();

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<Vec<u8>>>(2)?,
        ))
    })?;

    for row in rows {
        let (filename, title, emb_opt) = row?;
        if let Some(emb_bytes) = emb_opt {
            let emb = bytes_to_f32_vec(&emb_bytes);
            // ponytail: lineer kosinüs; not > ~5k olursa ANN ekle
            let score = cosine_similarity(&query_emb, &emb) as f64;
            if score >= min_score {
                results.push(SearchResult {
                    filename,
                    title,
                    score,
                });
            }
        }
    }

    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    results.truncate(limit);

    Ok(results)
}

/// A1: RRF (Reciprocal Rank Fusion) — k=60
/// Her kaynağın sıralama pozisyonunu 1/(k+rank) ile puanlar, birleştirir.
fn search_hybrid_rrf(
    conn: &Connection,
    query: &str,
    limit: usize,
    _min_score: f64,
) -> Result<Vec<SearchResult>> {
    let k = 60.0;

    // İki kanaldan geniş havuz çek — sonra RRF birleştirecek
    let keyword_results = search_keyword_vec(conn, query, 50, 0.0)?;
    let semantic_results = search_semantic_vec(conn, query, 50, 0.0)?;

    // RRF puanlarını birleştir
    let mut rrf_scores: HashMap<String, (f64, String)> = HashMap::new(); // filename → (rrf_score, title)

    for (rank, r) in keyword_results.iter().enumerate() {
        let rrf = 1.0 / (k + rank as f64 + 1.0);
        let entry = rrf_scores
            .entry(r.filename.clone())
            .or_insert((0.0, r.title.clone()));
        entry.0 += rrf;
    }

    for (rank, r) in semantic_results.iter().enumerate() {
        let rrf = 1.0 / (k + rank as f64 + 1.0);
        let entry = rrf_scores
            .entry(r.filename.clone())
            .or_insert((0.0, r.title.clone()));
        entry.0 += rrf;
    }

    // min_score bir KOSİNÜS eşiğidir (0..1); RRF puanları ~1/(k+rank) ölçeğindedir
    // (~0.016). Füzyon puanına kosinüs eşiği uygulamak her sonucu eler — hybrid
    // (varsayılan kip) her zaman boş dönerdi. Eşik semantik kipe aittir; kanallar
    // buraya zaten 0.0 ile (geniş havuz) çağrılıyor.
    let mut results: Vec<SearchResult> = rrf_scores
        .into_iter()
        .map(|(filename, (score, title))| SearchResult {
            filename,
            title,
            score,
        })
        .collect();

    results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    results.truncate(limit);

    Ok(results)
}

// ─── İlişkili Notlar (Graph BFS) ───────────────────────────────────────────

pub fn get_related_notes(note_filename: &str, home_override: Option<String>) -> Result<()> {
    let db_path = get_db_path(home_override)?;
    if !db_path.exists() {
        println!("{} Memory database not found.", "⚠".yellow());
        return Ok(());
    }

    let conn = Connection::open(&db_path)?;

    println!(
        "{} for '{}'",
        "Graph Related Notes".cyan().bold(),
        note_filename.yellow()
    );
    println!("========================================");

    // BFS ile 1 ve 2 adımlık komşuları bul
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((note_filename.to_string(), 0));
    visited.insert(note_filename.to_string());

    let mut stmt = conn.prepare("SELECT dst, tur, agirlik FROM note_edges WHERE src = ?1")?;

    while let Some((curr, dist)) = queue.pop_front() {
        if dist >= 2 {
            continue;
        }

        let rows = stmt.query_map(params![curr], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, f64>(2)?,
            ))
        })?;

        for row in rows {
            let (dst, tur, agirlik) = row?;
            if !visited.contains(&dst) {
                visited.insert(dst.clone());
                println!(
                    "  [Hop {}] {} (Edge: {}, Weight: {:.2})",
                    dist + 1,
                    dst.green().bold(),
                    tur.dimmed(),
                    agirlik
                );
                queue.push_back((dst, dist + 1));
            }
        }
    }

    println!("========================================");
    Ok(())
}

// ─── Testler ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_orthogonality() {
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![0.0, 1.0, 0.0];
        assert_eq!(cosine_similarity(&v1, &v2), 0.0);

        let v3 = vec![1.0, 2.0, 3.0];
        assert!((cosine_similarity(&v3, &v3) - 1.0).abs() < 1e-5);
    }

    // B5: Wikilink ayrıştırma testi
    #[test]
    fn test_extract_wikilinks() {
        let content = "See [[My Note]] and also [[other.md]] for details. No link here.";
        let links = extract_wikilinks(content);
        assert_eq!(links.len(), 2);
        assert_eq!(links[0], "My Note.md");
        assert_eq!(links[1], "other.md");
    }

    #[test]
    fn test_extract_wikilinks_empty() {
        let content = "No wikilinks here at all.";
        let links = extract_wikilinks(content);
        assert!(links.is_empty());
    }

    // A2: FTS5 kaçırma testi
    #[test]
    fn test_fts5_escape() {
        let q = "foo-bar baz";
        let escaped = escape_fts5_query(q);
        assert_eq!(escaped, "\"foo-bar\" \"baz\"");
    }

    #[test]
    fn test_fts5_escape_special_chars() {
        let q = "hello:world \"test\" *star";
        let escaped = escape_fts5_query(q);
        assert_eq!(escaped, "\"hello:world\" \"\"test\"\" \"*star\"");
    }

    // RRF sıralama testi (birim)
    #[test]
    fn test_rrf_ordering() {
        // RRF: doc hem keyword hem semantic'te ilkse en yüksek puanı almalı
        // k=60, rank 0: 1/61 ≈ 0.01639
        // İki kanalda rank 0: ~0.03279
        let k = 60.0;
        let dual_rank0 = 2.0 / (k + 1.0);
        let single_rank0 = 1.0 / (k + 1.0);
        assert!(dual_rank0 > single_rank0);
    }

    // Mean-pool testi
    #[test]
    fn test_mean_pool_embeddings() {
        let embs = vec![vec![1.0, 2.0, 3.0], vec![3.0, 4.0, 5.0]];
        let mean = mean_pool_embeddings(&embs);
        assert_eq!(mean.len(), 3);
        assert!((mean[0] - 2.0).abs() < 1e-5);
        assert!((mean[1] - 3.0).abs() < 1e-5);
        assert!((mean[2] - 4.0).abs() < 1e-5);
    }

    // Chunking testi
    #[test]
    fn test_chunk_content_short() {
        let content = "Short text";
        let chunks = chunk_content(content, 1500);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], "Short text");
    }

    #[test]
    fn test_chunk_content_long() {
        // 3000+ karakter oluştur
        let line = "This is a test line for chunking.\n";
        let content: String = line.repeat(100); // ~3300 karakter
        let chunks = chunk_content(&content, 1500);
        assert!(chunks.len() >= 2);
        // Tüm chunk'lar birleşince orijinale eşit
        let rejoined: String = chunks.concat();
        assert_eq!(rejoined, content);
    }

    #[test]
    fn test_chunk_content_utf8_no_panic() {
        // Regresyon: bayt-dilimleme Türkçe karakterin ortasında panikliyordu.
        // Satır sonu YOK ki rfind('\n') devreye girmesin, kesim tam çok baytlı
        // karakterin üstüne denk gelsin.
        let content: String = "çşğüöıÇŞĞÜÖİ".repeat(400);
        let chunks = chunk_content(&content, 1500);
        assert!(chunks.len() >= 2);
        let rejoined: String = chunks.concat();
        assert_eq!(rejoined, content, "chunk'lar orijinali kayıpsız vermeli");
    }

    #[test]
    fn test_add_memory_note_creates_file() {
        let dir = tempfile::tempdir().unwrap();
        let note_dir = dir.path().join("knowledge");
        let path = add_memory_note(
            "My Great Note!",
            Some("Body text here."),
            Some(note_dir.to_string_lossy().to_string()),
            None,
        )
        .unwrap();
        assert_eq!(path.file_name().unwrap().to_str().unwrap(), "my-great-note.md");
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.starts_with("# My Great Note!"));
        assert!(content.contains("Body text here."));
    }

    #[test]
    fn test_add_memory_note_refuses_overwrite() {
        let dir = tempfile::tempdir().unwrap();
        let note_dir = dir.path().join("knowledge");
        let dir_arg = Some(note_dir.to_string_lossy().to_string());
        add_memory_note("Dup Note", None, dir_arg.clone(), None).unwrap();
        let second = add_memory_note("Dup Note", Some("other"), dir_arg, None);
        assert!(second.is_err());
        // İlk içerik korunmuş olmalı
        let content = fs::read_to_string(note_dir.join("dup-note.md")).unwrap();
        assert_eq!(content, "# Dup Note\n");
    }

    #[test]
    fn test_add_memory_note_rejects_empty_slug() {
        let dir = tempfile::tempdir().unwrap();
        let result = add_memory_note(
            "!!!",
            None,
            Some(dir.path().to_string_lossy().to_string()),
            None,
        );
        assert!(result.is_err());
    }
}
