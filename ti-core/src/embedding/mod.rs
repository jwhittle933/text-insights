use std::sync::Arc;

use fastembed::{Embedding, EmbeddingModel, InitOptions, TextEmbedding};

#[derive(Clone)]
pub struct TextEmbedder {
    model: Arc<TextEmbedding>,
}

impl TextEmbedder {
    pub fn new() -> anyhow::Result<Self> {
        let model = Arc::new(TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::MultilingualE5Small).with_show_download_progress(true),
        )?);

        Ok(Self { model })
    }

    pub fn embed(&self, text: &str) -> anyhow::Result<Vec<Embedding>> {
        self.model.embed(vec![text], None)
    }

    /// Creates an embedding for each word in the text
    pub fn embed_text(&self, text: &str) -> anyhow::Result<Vec<Embedding>> {
        let words = text.trim().split(" ").collect::<Vec<_>>();
        self.model.embed(words, None)
    }
}
