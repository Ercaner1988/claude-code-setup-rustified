//! Hafiza motorunun sicak yollari: gomme (embedding) matematigi, blob
//! donusumleri, icerik parcalama ve markdown/FTS5 metin isleme.

use claude_code_setup::memory_engine::{
    bytes_to_f32_vec, chunk_content, cosine_similarity, escape_fts5_query, extract_wikilinks,
    f32_vec_to_bytes, mean_pool_embeddings,
};

fn main() {
    divan::main();
}

/// MultilingualE5Small gomme boyutu.
const EMBEDDING_DIM: usize = 384;

/// Deterministik, bagimliliksiz sozde-rastgele uretici (xorshift64*).
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    fn next_f32(&mut self) -> f32 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        let bits = self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 40;
        (bits as f32 / (1u32 << 24) as f32) * 2.0 - 1.0
    }
}

fn random_embedding(seed: u64, dim: usize) -> Vec<f32> {
    let mut rng = Rng::new(seed);
    (0..dim).map(|_| rng.next_f32()).collect()
}

/// Gercekci bir bilgi notu: markdown basliklari, wikilink'ler ve Turkce metin.
fn knowledge_note(paragraphs: usize) -> String {
    let mut out = String::from("# Bilgi Notu\n\n");
    for i in 0..paragraphs {
        out.push_str(&format!("## Bolum {i}\n\n"));
        out.push_str(
            "Claude Code ortam denetimi ve MCP yonetimi icin notlar. \
             Semantik arama gomme vektorleri uzerinde kosinus benzerligi kullanir. \
             Ayrintilar icin [[mcp-yonetimi]] ve [[hafiza-motoru.md]] notlarina bakin.\n\n",
        );
        out.push_str(
            "- FTS5 anahtar kelime aramasi\n- Graf kenarlari ile iliskili notlar\n\
             - Guvenlik denetimi ve gizli anahtar taramasi\n\n",
        );
    }
    out
}

mod similarity {
    use super::*;

    /// Tek bir not ile sorgu vektorunun karsilastirilmasi.
    #[divan::bench]
    fn cosine_single(bencher: divan::Bencher) {
        let query = random_embedding(1, EMBEDDING_DIM);
        let note = random_embedding(2, EMBEDDING_DIM);
        bencher.bench(|| cosine_similarity(divan::black_box(&query), divan::black_box(&note)));
    }

    /// Dogrusal tarama: sorguyu tum notlarla karsilastirip en iyileri siralar
    /// (search_semantic_vec'in sicak dongusu).
    #[divan::bench(args = [64, 512, 4096])]
    fn cosine_linear_scan(bencher: divan::Bencher, note_count: usize) {
        let query = random_embedding(7, EMBEDDING_DIM);
        let notes: Vec<Vec<f32>> = (0..note_count)
            .map(|i| random_embedding(i as u64 + 100, EMBEDDING_DIM))
            .collect();

        bencher.bench(|| {
            let mut scored: Vec<f64> = divan::black_box(&notes)
                .iter()
                .map(|note| cosine_similarity(&query, note) as f64)
                .collect();
            scored.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
            scored.truncate(10);
            scored
        });
    }

    /// Parca gommelerinin ortalamasi (uzun notlar icin mean-pool).
    #[divan::bench(args = [4, 32, 256])]
    fn mean_pool(bencher: divan::Bencher, chunk_count: usize) {
        let embeddings: Vec<Vec<f32>> = (0..chunk_count)
            .map(|i| random_embedding(i as u64 + 5, EMBEDDING_DIM))
            .collect();
        bencher.bench(|| mean_pool_embeddings(divan::black_box(&embeddings)));
    }
}

mod blob_codec {
    use super::*;

    /// f32 vektorunu SQLite BLOB'una yaz.
    #[divan::bench]
    fn encode(bencher: divan::Bencher) {
        let embedding = random_embedding(11, EMBEDDING_DIM);
        bencher.bench(|| f32_vec_to_bytes(divan::black_box(&embedding)));
    }

    /// SQLite BLOB'undan f32 vektorunu oku.
    #[divan::bench]
    fn decode(bencher: divan::Bencher) {
        let bytes = f32_vec_to_bytes(&random_embedding(13, EMBEDDING_DIM));
        bencher.bench(|| bytes_to_f32_vec(divan::black_box(&bytes)));
    }

    /// 1000 notluk bir indeksin tamaminin cozulmesi.
    #[divan::bench]
    fn decode_index(bencher: divan::Bencher) {
        let blobs: Vec<Vec<u8>> = (0..1000)
            .map(|i| f32_vec_to_bytes(&random_embedding(i as u64 + 21, EMBEDDING_DIM)))
            .collect();
        bencher.bench(|| {
            divan::black_box(&blobs)
                .iter()
                .map(|blob| bytes_to_f32_vec(blob))
                .collect::<Vec<_>>()
        });
    }
}

mod text {
    use super::*;

    /// Notu satir sinirinda ~512 karakterlik pencerelere bol.
    #[divan::bench(args = [8, 64, 512])]
    fn chunk(bencher: divan::Bencher, paragraphs: usize) {
        let note = knowledge_note(paragraphs);
        bencher.bench(|| chunk_content(divan::black_box(&note), 512));
    }

    /// Wikilink cikarimi (graf kenarlarini besler).
    #[divan::bench(args = [8, 64, 512])]
    fn wikilinks(bencher: divan::Bencher, paragraphs: usize) {
        let note = knowledge_note(paragraphs);
        bencher.bench(|| extract_wikilinks(divan::black_box(&note)));
    }

    /// FTS5 sorgu kacirma.
    #[divan::bench]
    fn fts5_escape(bencher: divan::Bencher) {
        let query = "mcp sunucu yapilandirmasi semantik arama gomme vektoru guvenlik denetimi";
        bencher.bench(|| escape_fts5_query(divan::black_box(query)));
    }
}
