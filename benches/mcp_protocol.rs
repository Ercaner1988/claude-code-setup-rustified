//! MCP sunucusunun JSON-RPC sicak yolu: istek ayristirma, yonlendirme ve
//! yanit serilestirme (handle_request) ile arac -> CLI arguman cevrimi.

use claude_code_setup::mcp_server::{handle_request, tool_to_cli_args, JsonRpcRequest};
use serde_json::json;

fn main() {
    divan::main();
}

const INITIALIZE: &str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
const TOOLS_LIST: &str = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#;
const RESOURCES_LIST: &str = r#"{"jsonrpc":"2.0","id":3,"method":"resources/list","params":{}}"#;
const UNKNOWN_METHOD: &str = r#"{"jsonrpc":"2.0","id":4,"method":"prompts/list","params":{}}"#;

fn parse(line: &str) -> JsonRpcRequest {
    serde_json::from_str(line).expect("valid json-rpc request")
}

mod dispatch {
    use super::*;

    /// Sunucu yeteneklerini dondurur (her oturumun ilk cagrisi).
    #[divan::bench]
    fn initialize(bencher: divan::Bencher) {
        let request = parse(INITIALIZE);
        bencher.bench(|| handle_request(divan::black_box(&request)));
    }

    /// En buyuk yanit: tum araclarin JSON Schema'lari serilestirilir.
    #[divan::bench]
    fn tools_list(bencher: divan::Bencher) {
        let request = parse(TOOLS_LIST);
        bencher.bench(|| handle_request(divan::black_box(&request)));
    }

    #[divan::bench]
    fn resources_list(bencher: divan::Bencher) {
        let request = parse(RESOURCES_LIST);
        bencher.bench(|| handle_request(divan::black_box(&request)));
    }

    /// Bilinmeyen metot -> JSON-RPC hata yolu.
    #[divan::bench]
    fn unknown_method(bencher: divan::Bencher) {
        let request = parse(UNKNOWN_METHOD);
        bencher.bench(|| handle_request(divan::black_box(&request)));
    }
}

mod parsing {
    use super::*;

    /// stdin satirindan JsonRpcRequest ayristirma.
    #[divan::bench]
    fn parse_request(bencher: divan::Bencher) {
        bencher.bench(|| parse(divan::black_box(TOOLS_LIST)));
    }
}

mod tool_args {
    use super::*;

    #[divan::bench]
    fn mcp_add(bencher: divan::Bencher) {
        let args = json!({
            "target": "claude-code",
            "name": "filesystem",
            "command": "npx",
            "args": ["-y", "@modelcontextprotocol/server-filesystem", "/home/kullanici"]
        });
        bencher.bench(|| tool_to_cli_args(divan::black_box("mcp_add"), divan::black_box(&args)));
    }

    #[divan::bench]
    fn memory_note(bencher: divan::Bencher) {
        let args = json!({
            "text": "Olcum notu\n\nHafiza motoru FTS5 ve gomme vektorlerini birlikte kullanir."
        });
        bencher
            .bench(|| tool_to_cli_args(divan::black_box("memory_note"), divan::black_box(&args)));
    }
}
