use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{LanguageServer, LspService, Server};

struct Backend;

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec!["0".to_string(), "1".to_string()]),
                    ..Default::default()
                }),
                ..ServerCapabilities::default()
            }
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        let all_combinations: Vec<String> =
            (0..(2i32.pow(8))).map(|i| format!("{i:0>8b}")).collect();

        let items = all_combinations
            .into_iter()
            .map(|item| {
                let translated = item
                    .chars()
                    .map(|c| match c {
                        '0' => "LVESRBN",
                        '1' => "HVESRBN",
                        _ => unreachable!(),
                    })
                    .collect::<Vec<&'static str>>()
                    .join(" ");

                CompletionItem {
                    label: item.clone(),
                    detail: Some(
                        format!("{translated}\n\n- HVESRBN (High Voltage Electrical Signal Represented as a Binary Number) aka 1\n- LVESRBN (Low Voltage Electrical Signal Represented as a Binary Number) aka 0")
                    ),
                    kind: Some(CompletionItemKind::VALUE),
                    ..Default::default()
                }
            })
            .collect();

        Ok(Some(CompletionResponse::Array(items)))
    }
}

pub async fn run() {
    tracing_subscriber::fmt().init();

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = LspService::new(|_| Backend);

    Server::new(stdin, stdout, socket).serve(service).await;
}
