//! Guvenlik denetiminin sicak yollari: gizli anahtar deseni taramasi ve
//! dal adi normalizasyonu.

use claude_code_setup::branch_manager::{is_protected_branch, sanitize_description};
use claude_code_setup::security::find_secrets;

fn main() {
    divan::main();
}

/// Gizli anahtar icermeyen, gercekci bir MCP yapilandirmasi.
fn clean_config(entries: usize) -> String {
    let mut out = String::from("{\n  \"mcpServers\": {\n");
    for i in 0..entries {
        out.push_str(&format!(
            "    \"sunucu-{i}\": {{ \"command\": \"npx\", \
             \"args\": [\"-y\", \"@modelcontextprotocol/server-filesystem\", \"/home/kullanici\"], \
             \"env\": {{ \"LOG_LEVEL\": \"info\" }} }},\n"
        ));
    }
    out.push_str("  }\n}\n");
    out
}

/// Icine bilinen desenlerden (gercek olmayan) gizli anahtarlar serpistirilmis
/// yapilandirma — src/security.rs testlerindeki ayni sahte degerler.
fn config_with_secrets(entries: usize) -> String {
    let mut out = clean_config(entries);
    out.push_str("  token = \"ghp_abcdefghij1234567890abcd\"\n");
    out.push_str("  other = \"sk-proj-abcdef1234567890\"\n");
    out.push_str("  slack = \"xoxb-1234567890-abcdefghij\"\n");
    out.push_str("  aws = \"AKIAIOSFODNN7EXAMPLE\"\n");
    out
}

#[divan::bench(args = [8, 64, 256])]
fn scan_clean_config(bencher: divan::Bencher, entries: usize) {
    let text = clean_config(entries);
    bencher.bench(|| find_secrets(divan::black_box(&text)));
}

#[divan::bench(args = [8, 64, 256])]
fn scan_config_with_secrets(bencher: divan::Bencher, entries: usize) {
    let text = config_with_secrets(entries);
    bencher.bench(|| find_secrets(divan::black_box(&text)));
}

#[divan::bench]
fn sanitize_branch_description(bencher: divan::Bencher) {
    let description = "Fix Login Bug! (Turkce karakterler: sifre dogrulama & oturum yenileme)";
    bencher.bench(|| sanitize_description(divan::black_box(description)));
}

#[divan::bench]
fn protected_branch_check(bencher: divan::Bencher) {
    bencher.bench(|| is_protected_branch(divan::black_box("feature/olcum")));
}
