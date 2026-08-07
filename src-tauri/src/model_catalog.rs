use serde_json::Value;

/// A single model's capability entry.
pub(crate) struct ModelCapability {
    pub(crate) modalities: &'static [&'static str],
    pub(crate) reasoning: bool,
    pub(crate) thinking_effort: Option<&'static [&'static str]>,
    pub(crate) context: u64,
}

/// Comprehensive model capabilities dictionary.
/// Keyed by normalized model name (same name = same model regardless of URL/provider).
/// Sources: OpenRouter registry, OpenAI/Anthropic/Google/DeepSeek/Zhipu/StepFun official docs.
pub(crate) static MODEL_CAPABILITIES: &[(&str, ModelCapability)] = &[
    ("aion-2.0", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("aion-3.0", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("aion-3.0-mini", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("aion-rp-llama-3.1-8b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 32768 }),
    ("auto", ModelCapability { modalities: &["text", "image", "audio", "file", "video"], reasoning: false, thinking_effort: None, context: 2000000 }),
    ("auto-beta", ModelCapability { modalities: &["text", "image", "audio", "file", "video"], reasoning: false, thinking_effort: None, context: 2000000 }),
    ("bodybuilder", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("claude-3-haiku", ModelCapability { modalities: &["text", "image", "pdf"], reasoning: false, thinking_effort: None, context: 200000 }),
    ("claude-3-opus", ModelCapability { modalities: &["text", "image", "pdf"], reasoning: false, thinking_effort: None, context: 200000 }),
    ("claude-3.5-haiku", ModelCapability { modalities: &["text", "image", "pdf"], reasoning: false, thinking_effort: None, context: 200000 }),
    ("claude-3.5-sonnet", ModelCapability { modalities: &["text", "image", "pdf"], reasoning: false, thinking_effort: None, context: 200000 }),
    ("claude-3.7-sonnet", ModelCapability { modalities: &["text", "image", "pdf"], reasoning: true, thinking_effort: None, context: 200000 }),
    ("claude-fable-5", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high", "xhigh", "max"]), context: 1000000 }),
    ("claude-fable-latest", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("claude-haiku-4-5", ModelCapability { modalities: &["text", "image", "pdf"], reasoning: true, thinking_effort: None, context: 200000 }),
    ("claude-haiku-4.5", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: None, context: 200000 }),
    ("claude-haiku-latest", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 200000 }),
    ("claude-opus-4", ModelCapability { modalities: &["image", "text", "file", "pdf"], reasoning: true, thinking_effort: None, context: 200000 }),
    ("claude-opus-4-5", ModelCapability { modalities: &["text", "image", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 200000 }),
    ("claude-opus-4.1", ModelCapability { modalities: &["image", "text", "file", "pdf"], reasoning: true, thinking_effort: None, context: 200000 }),
    ("claude-opus-4.5", ModelCapability { modalities: &["file", "image", "text", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 200000 }),
    ("claude-opus-4.6", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high", "max"]), context: 1000000 }),
    ("claude-opus-4.7", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high", "xhigh", "max"]), context: 1000000 }),
    ("claude-opus-4.7-fast", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("claude-opus-4.8", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high", "xhigh", "max"]), context: 1000000 }),
    ("claude-opus-4.8-fast", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("claude-opus-5", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high", "xhigh", "max"]), context: 1000000 }),
    ("claude-opus-5-fast", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("claude-opus-latest", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("claude-sonnet-4", ModelCapability { modalities: &["image", "text", "file", "pdf"], reasoning: true, thinking_effort: None, context: 1000000 }),
    ("claude-sonnet-4-5", ModelCapability { modalities: &["text", "image", "pdf"], reasoning: true, thinking_effort: None, context: 1000000 }),
    ("claude-sonnet-4.5", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: None, context: 1000000 }),
    ("claude-sonnet-4.6", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high", "max"]), context: 1000000 }),
    ("claude-sonnet-5", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high", "xhigh", "max"]), context: 1000000 }),
    ("claude-sonnet-latest", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("codestral", ModelCapability { modalities: &["text", "pdf"], reasoning: false, thinking_effort: None, context: 256000 }),
    ("codestral-2508", ModelCapability { modalities: &["text", "file"], reasoning: false, thinking_effort: None, context: 256000 }),
    ("cogito-v2.1-671b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("command-a", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 256000 }),
    ("command-r-08-2024", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("command-r-plus-08-2024", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("command-r7b-12-2024", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("cydonia-24b-v4.1", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("deepseek-chat", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 163840 }),
    ("deepseek-chat-v3-0324", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 163840 }),
    ("deepseek-chat-v3.1", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 163840 }),
    ("deepseek-r1", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: None, context: 163840 }),
    ("deepseek-r1-0528", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 163840 }),
    ("deepseek-r1-distill-llama-70b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 8192 }),
    ("deepseek-reasoner", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: None, context: 163840 }),
    ("deepseek-v3", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 163840 }),
    ("deepseek-v3.1-terminus", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 163840 }),
    ("deepseek-v3.2", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 163840 }),
    ("deepseek-v3.2-exp", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 163840 }),
    ("deepseek-v4-flash", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: Some(&["high", "xhigh"]), context: 1048576 }),
    ("deepseek-v4-flash-0731", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("deepseek-v4-flash-latest", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("deepseek-v4-pro", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: Some(&["high", "xhigh"]), context: 1048576 }),
    ("dolphin-mistral-24b-venice-edition", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("doubao-seed-1.6", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("doubao-seed-2-1-pro", ModelCapability { modalities: &["text", "image", "video"], reasoning: true, thinking_effort: None, context: 262144 }),
    ("doubao-seed-2.0-lite", ModelCapability { modalities: &["text", "image", "video"], reasoning: true, thinking_effort: Some(&["minimal", "low", "medium", "high"]), context: 262144 }),
    ("doubao-seed-2.0-mini", ModelCapability { modalities: &["text", "image", "video"], reasoning: true, thinking_effort: Some(&["minimal", "low", "medium", "high"]), context: 262144 }),
    ("ernie-4.5-vl-424b-a47b", ModelCapability { modalities: &["image", "text"], reasoning: false, thinking_effort: None, context: 123000 }),
    ("free", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 200000 }),
    ("fugu-ultra", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("fusion", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("gemini-1.5-flash", ModelCapability { modalities: &["text", "image", "audio", "video", "pdf"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("gemini-1.5-pro", ModelCapability { modalities: &["text", "image", "audio", "video", "pdf"], reasoning: false, thinking_effort: None, context: 2000000 }),
    ("gemini-2.0-flash", ModelCapability { modalities: &["text", "image", "audio", "video", "pdf"], reasoning: true, thinking_effort: None, context: 1048576 }),
    ("gemini-2.5-flash", ModelCapability { modalities: &["file", "image", "text", "audio", "video", "pdf"], reasoning: true, thinking_effort: None, context: 1048576 }),
    ("gemini-2.5-flash-image", ModelCapability { modalities: &["image", "text"], reasoning: false, thinking_effort: None, context: 32768 }),
    ("gemini-2.5-flash-lite", ModelCapability { modalities: &["text", "image", "file", "audio", "video", "pdf"], reasoning: true, thinking_effort: None, context: 1048576 }),
    ("gemini-2.5-pro", ModelCapability { modalities: &["text", "image", "file", "audio", "video", "pdf"], reasoning: true, thinking_effort: None, context: 1048576 }),
    ("gemini-2.5-pro-preview", ModelCapability { modalities: &["file", "image", "text", "audio"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("gemini-2.5-pro-preview-05-06", ModelCapability { modalities: &["text", "image", "file", "audio", "video"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("gemini-3-flash-preview", ModelCapability { modalities: &["text", "image", "file", "audio", "video", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 1048576 }),
    ("gemini-3-pro-image", ModelCapability { modalities: &["image", "text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("gemini-3-pro-image-preview", ModelCapability { modalities: &["image", "text"], reasoning: false, thinking_effort: None, context: 65536 }),
    ("gemini-3.1-flash-image", ModelCapability { modalities: &["image", "text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("gemini-3.1-flash-image-preview", ModelCapability { modalities: &["image", "text"], reasoning: false, thinking_effort: None, context: 65536 }),
    ("gemini-3.1-flash-lite", ModelCapability { modalities: &["text", "image", "video", "file", "audio", "pdf"], reasoning: true, thinking_effort: Some(&["minimal", "low", "medium", "high"]), context: 1048576 }),
    ("gemini-3.1-flash-lite-image", ModelCapability { modalities: &["image", "text"], reasoning: false, thinking_effort: None, context: 65536 }),
    ("gemini-3.1-flash-lite-preview", ModelCapability { modalities: &["text", "image", "video", "file", "audio"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("gemini-3.1-pro-preview", ModelCapability { modalities: &["audio", "file", "image", "text", "video", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 1048576 }),
    ("gemini-3.1-pro-preview-customtools", ModelCapability { modalities: &["text", "audio", "image", "video", "file"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("gemini-3.5-flash", ModelCapability { modalities: &["text", "image", "video", "file", "audio", "pdf"], reasoning: true, thinking_effort: Some(&["minimal", "low", "medium", "high"]), context: 1048576 }),
    ("gemini-3.5-flash-lite", ModelCapability { modalities: &["text", "image", "video", "file", "audio", "pdf"], reasoning: true, thinking_effort: Some(&["minimal", "low", "medium", "high"]), context: 1048576 }),
    ("gemini-3.6-flash", ModelCapability { modalities: &["text", "image", "video", "file", "audio", "pdf"], reasoning: true, thinking_effort: Some(&["minimal", "low", "medium", "high"]), context: 1048576 }),
    ("gemini-3.6-pro", ModelCapability { modalities: &["text", "image", "audio", "video", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 1048576 }),
    ("gemini-flash-latest", ModelCapability { modalities: &["text", "image", "video", "file", "audio"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("gemini-pro-latest", ModelCapability { modalities: &["audio", "file", "image", "text", "video"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("gemma-2-27b-it", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 8192 }),
    ("gemma-3-12b-it", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("gemma-3-27b-it", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("gemma-3-4b-it", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("gemma-3n-e4b-it", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 32768 }),
    ("gemma-4-26b-a4b-it", ModelCapability { modalities: &["image", "text", "video"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("gemma-4-31b-it", ModelCapability { modalities: &["image", "text", "video"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("glm-4.1v", ModelCapability { modalities: &["text", "image"], reasoning: true, thinking_effort: None, context: 65536 }),
    ("glm-4.5", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("glm-4.5-air", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("glm-4.5v", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 65536 }),
    ("glm-4.6", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 204800 }),
    ("glm-4.6v", ModelCapability { modalities: &["image", "text", "video"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("glm-4.7", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: None, context: 204800 }),
    ("glm-4.7-flash", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 202752 }),
    ("glm-4v", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 16384 }),
    ("glm-5", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 204800 }),
    ("glm-5-turbo", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 202752 }),
    ("glm-5.1", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: None, context: 204800 }),
    ("glm-5.2", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: Some(&["high", "xhigh"]), context: 1048576 }),
    ("glm-5.2v", ModelCapability { modalities: &["text", "image", "video"], reasoning: true, thinking_effort: None, context: 202752 }),
    ("glm-5v-turbo", ModelCapability { modalities: &["image", "text", "video"], reasoning: true, thinking_effort: None, context: 202752 }),
    ("gpt-3.5-turbo", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 16385 }),
    ("gpt-3.5-turbo-0613", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 4095 }),
    ("gpt-3.5-turbo-16k", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 16385 }),
    ("gpt-3.5-turbo-instruct", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 4095 }),
    ("gpt-4", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 8191 }),
    ("gpt-4-turbo", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("gpt-4-turbo-preview", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("gpt-4.1", ModelCapability { modalities: &["image", "text", "file"], reasoning: false, thinking_effort: None, context: 1047576 }),
    ("gpt-4.1-mini", ModelCapability { modalities: &["image", "text", "file"], reasoning: false, thinking_effort: None, context: 1047576 }),
    ("gpt-4.1-nano", ModelCapability { modalities: &["image", "text", "file"], reasoning: false, thinking_effort: None, context: 1047576 }),
    ("gpt-4o", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("gpt-4o-2024-05-13", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("gpt-4o-2024-08-06", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("gpt-4o-2024-11-20", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("gpt-4o-mini", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("gpt-4o-mini-2024-07-18", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("gpt-5", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: Some(&["minimal", "low", "medium", "high"]), context: 400000 }),
    ("gpt-5-2", ModelCapability { modalities: &["text", "image", "pdf"], reasoning: true, thinking_effort: Some(&["none", "low", "medium", "high", "xhigh"]), context: 400000 }),
    ("gpt-5-2-codex", ModelCapability { modalities: &["text", "image"], reasoning: true, thinking_effort: None, context: 400000 }),
    ("gpt-5-3-codex", ModelCapability { modalities: &["text", "image"], reasoning: true, thinking_effort: None, context: 400000 }),
    ("gpt-5-6-luna", ModelCapability { modalities: &["text", "image", "pdf"], reasoning: true, thinking_effort: Some(&["none", "low", "medium", "high", "xhigh", "max"]), context: 1050000 }),
    ("gpt-5-6-sol", ModelCapability { modalities: &["text", "image", "pdf"], reasoning: true, thinking_effort: Some(&["none", "low", "medium", "high", "xhigh", "max"]), context: 1050000 }),
    ("gpt-5-6-terra", ModelCapability { modalities: &["text", "image", "pdf"], reasoning: true, thinking_effort: Some(&["none", "low", "medium", "high", "xhigh", "max"]), context: 1050000 }),
    ("gpt-5-codex", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 400000 }),
    ("gpt-5-image", ModelCapability { modalities: &["image", "text", "file"], reasoning: false, thinking_effort: None, context: 400000 }),
    ("gpt-5-image-mini", ModelCapability { modalities: &["file", "image", "text"], reasoning: false, thinking_effort: None, context: 400000 }),
    ("gpt-5-mini", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: Some(&["minimal", "low", "medium", "high"]), context: 400000 }),
    ("gpt-5-nano", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: Some(&["minimal", "low", "medium", "high"]), context: 400000 }),
    ("gpt-5-pro", ModelCapability { modalities: &["image", "text", "file"], reasoning: false, thinking_effort: None, context: 400000 }),
    ("gpt-5.1", ModelCapability { modalities: &["image", "text", "file", "pdf"], reasoning: true, thinking_effort: Some(&["none", "low", "medium", "high"]), context: 400000 }),
    ("gpt-5.1-codex", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 400000 }),
    ("gpt-5.1-codex-max", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 400000 }),
    ("gpt-5.1-codex-mini", ModelCapability { modalities: &["image", "text"], reasoning: false, thinking_effort: None, context: 400000 }),
    ("gpt-5.2", ModelCapability { modalities: &["file", "image", "text", "pdf"], reasoning: true, thinking_effort: Some(&["none", "low", "medium", "high", "xhigh"]), context: 400000 }),
    ("gpt-5.2-chat", ModelCapability { modalities: &["file", "image", "text"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("gpt-5.2-codex", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 400000 }),
    ("gpt-5.2-pro", ModelCapability { modalities: &["image", "text", "file"], reasoning: false, thinking_effort: None, context: 400000 }),
    ("gpt-5.3-chat", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("gpt-5.3-codex", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 400000 }),
    ("gpt-5.4", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: Some(&["none", "low", "medium", "high", "xhigh"]), context: 1050000 }),
    ("gpt-5.4-image-2", ModelCapability { modalities: &["image", "text", "file"], reasoning: false, thinking_effort: None, context: 272000 }),
    ("gpt-5.4-mini", ModelCapability { modalities: &["file", "image", "text"], reasoning: false, thinking_effort: None, context: 400000 }),
    ("gpt-5.4-nano", ModelCapability { modalities: &["file", "image", "text"], reasoning: false, thinking_effort: None, context: 400000 }),
    ("gpt-5.4-pro", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 1050000 }),
    ("gpt-5.5", ModelCapability { modalities: &["file", "image", "text", "pdf"], reasoning: true, thinking_effort: Some(&["none", "low", "medium", "high", "xhigh"]), context: 1050000 }),
    ("gpt-5.5-pro", ModelCapability { modalities: &["file", "image", "text"], reasoning: false, thinking_effort: None, context: 1050000 }),
    ("gpt-5.6-luna", ModelCapability { modalities: &["file", "image", "text"], reasoning: false, thinking_effort: None, context: 1050000 }),
    ("gpt-5.6-luna-pro", ModelCapability { modalities: &["file", "image", "text"], reasoning: false, thinking_effort: None, context: 1050000 }),
    ("gpt-5.6-sol", ModelCapability { modalities: &["file", "image", "text"], reasoning: false, thinking_effort: None, context: 1050000 }),
    ("gpt-5.6-sol-pro", ModelCapability { modalities: &["file", "image", "text"], reasoning: false, thinking_effort: None, context: 1050000 }),
    ("gpt-5.6-terra", ModelCapability { modalities: &["file", "image", "text"], reasoning: false, thinking_effort: None, context: 1050000 }),
    ("gpt-5.6-terra-pro", ModelCapability { modalities: &["file", "image", "text"], reasoning: false, thinking_effort: None, context: 1050000 }),
    ("gpt-audio", ModelCapability { modalities: &["text", "audio"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("gpt-audio-mini", ModelCapability { modalities: &["text", "audio"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("gpt-chat-latest", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 400000 }),
    ("gpt-latest", ModelCapability { modalities: &["file", "image", "text"], reasoning: false, thinking_effort: None, context: 1050000 }),
    ("gpt-mini-latest", ModelCapability { modalities: &["file", "image", "text"], reasoning: false, thinking_effort: None, context: 400000 }),
    ("gpt-oss-120b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("gpt-oss-20b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("gpt-oss-safeguard-20b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("granite-4.0-h-micro", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131000 }),
    ("granite-4.1-8b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("grok-3", ModelCapability { modalities: &["text", "image"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 131072 }),
    ("grok-3-mini", ModelCapability { modalities: &["text", "image"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 131072 }),
    ("grok-4", ModelCapability { modalities: &["text", "image", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 256000 }),
    ("grok-4.20", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 2000000 }),
    ("grok-4.20-multi-agent", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 2000000 }),
    ("grok-4.3", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("grok-4.5", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 500000 }),
    ("grok-build-0.1", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 256000 }),
    ("grok-latest", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 500000 }),
    ("hermes-3-llama-3.1-405b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("hermes-3-llama-3.1-70b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("hermes-4-405b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("hermes-4-70b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("hunyuan-a13b-instruct", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("hy3", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("hy3-preview", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("inkling", ModelCapability { modalities: &["text", "image", "audio"], reasoning: false, thinking_effort: None, context: 524288 }),
    ("inkling-small", ModelCapability { modalities: &["text", "image", "audio"], reasoning: false, thinking_effort: None, context: 524288 }),
    ("jamba-large-1.7", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 256000 }),
    ("kat-coder-air-v2.5", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 256000 }),
    ("kat-coder-pro-v2", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("kat-coder-pro-v2.5", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 256000 }),
    ("kimi-for-coding", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: None, context: 262144 }),
    ("kimi-k2", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: None, context: 131072 }),
    ("kimi-k2-0905", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("kimi-k2-thinking", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("kimi-k2.5", ModelCapability { modalities: &["text", "image"], reasoning: true, thinking_effort: None, context: 262144 }),
    ("kimi-k2.6", ModelCapability { modalities: &["text", "image"], reasoning: true, thinking_effort: None, context: 262144 }),
    ("kimi-k2.7-code", ModelCapability { modalities: &["text", "image"], reasoning: true, thinking_effort: None, context: 262144 }),
    ("kimi-k3", ModelCapability { modalities: &["text", "image", "video", "pdf"], reasoning: true, thinking_effort: Some(&["low", "high", "max"]), context: 1048576 }),
    ("kimi-latest", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("l3-lunaris-8b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 8192 }),
    ("l3.1-euryale-70b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("l3.3-euryale-70b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("laguna-s-2.1", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("laguna-xs-2.1", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("ling-2.6-1t", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("ling-2.6-flash", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("ling-3.0-flash", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("ling-3.0-tiny", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("llama-3.1-70b-instruct", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("llama-3.1-8b-instruct", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("llama-3.2-1b-instruct", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 60000 }),
    ("llama-3.2-3b-instruct", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("llama-3.3-70b-instruct", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("llama-4-maverick", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("llama-4-scout", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 1310720 }),
    ("llama-guard-4-12b", ModelCapability { modalities: &["image", "text"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("longcat-2.0", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 1048756 }),
    ("lyria-3-clip-preview", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("lyria-3-pro-preview", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("magnum-v4-72b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 16384 }),
    ("mercury-2", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("mimo-v2.5", ModelCapability { modalities: &["text", "audio", "image", "video"], reasoning: false, thinking_effort: None, context: 1050000 }),
    ("mimo-v2.5-pro", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 1050000 }),
    ("minimax-01", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 1000192 }),
    ("minimax-m1", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("minimax-m2", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 204800 }),
    ("minimax-m2-her", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 65536 }),
    ("minimax-m2.1", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 204800 }),
    ("minimax-m2.5", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 204800 }),
    ("minimax-m2.7", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: None, context: 204800 }),
    ("minimax-m2.7-highspeed", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: None, context: 204800 }),
    ("minimax-m3", ModelCapability { modalities: &["text", "image", "video"], reasoning: true, thinking_effort: None, context: 1048576 }),
    ("ministral", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("ministral-14b-2512", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("ministral-3b-2512", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("ministral-8b-2512", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("mistral-large", ModelCapability { modalities: &["text", "file", "image", "pdf"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("mistral-large-2407", ModelCapability { modalities: &["text", "file"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("mistral-large-2512", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("mistral-medium-3", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("mistral-medium-3-5", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("mistral-medium-3.1", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("mistral-nemo", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("mistral-saba", ModelCapability { modalities: &["text", "file"], reasoning: false, thinking_effort: None, context: 32768 }),
    ("mistral-small", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 256000 }),
    ("mistral-small-24b-instruct-2501", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 32768 }),
    ("mistral-small-2603", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("mistral-small-3.1-24b-instruct", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("mistral-small-3.2-24b-instruct", ModelCapability { modalities: &["image", "text"], reasoning: false, thinking_effort: None, context: 256000 }),
    ("mixtral-8x22b-instruct", ModelCapability { modalities: &["text", "file"], reasoning: false, thinking_effort: None, context: 65536 }),
    ("morph-v3-fast", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 81920 }),
    ("morph-v3-large", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("muse-spark-1.1", ModelCapability { modalities: &["text", "image", "video", "file", "audio"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("muse-spark-1.2", ModelCapability { modalities: &["text", "image", "video", "file", "audio"], reasoning: false, thinking_effort: None, context: 1048576 }),
    ("mythomax-l2-13b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 8192 }),
    ("nemotron-3-nano-30b-a3b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 256000 }),
    ("nemotron-3-nano-omni-30b-a3b-reasoning", ModelCapability { modalities: &["text", "audio", "image", "video"], reasoning: false, thinking_effort: None, context: 256000 }),
    ("nemotron-3-super-120b-a12b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("nemotron-3-ultra-550b-a55b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("nemotron-3.5-content-safety", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("nemotron-nano-12b-v2-vl", ModelCapability { modalities: &["image", "text", "video"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("nemotron-nano-9b-v2", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("nex-n2-mini", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("nex-n2-pro", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("north-mini-code", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 256000 }),
    ("nova-2-lite-v1", ModelCapability { modalities: &["text", "image", "video", "file"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("nova-lite-v1", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 300000 }),
    ("nova-micro-v1", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("nova-premier-v1", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("nova-pro-v1", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 300000 }),
    ("o1", ModelCapability { modalities: &["text", "image", "file", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 200000 }),
    ("o1-mini", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 128000 }),
    ("o1-pro", ModelCapability { modalities: &["text", "image", "file"], reasoning: false, thinking_effort: None, context: 200000 }),
    ("o3", ModelCapability { modalities: &["image", "text", "file", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 200000 }),
    ("o3-mini", ModelCapability { modalities: &["text", "file", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 200000 }),
    ("o3-mini-high", ModelCapability { modalities: &["text", "file"], reasoning: false, thinking_effort: None, context: 200000 }),
    ("o3-pro", ModelCapability { modalities: &["text", "file", "image", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 200000 }),
    ("o4-mini", ModelCapability { modalities: &["image", "text", "file", "pdf"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 200000 }),
    ("o4-mini-high", ModelCapability { modalities: &["image", "text", "file"], reasoning: false, thinking_effort: None, context: 200000 }),
    ("olmo-3-32b-think", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 65536 }),
    ("palmyra-x5", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 1040000 }),
    ("pareto-code", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 2000000 }),
    ("perceptron-mk1", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 32768 }),
    ("phi-4", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 16384 }),
    ("pixtral", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("qwen-2.5-72b-instruct", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 32768 }),
    ("qwen-2.5-7b-instruct", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 32768 }),
    ("qwen-2.5-coder-32b-instruct", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 32768 }),
    ("qwen-plus", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("qwen-plus-2025-07-28", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("qwen2.5-omni", ModelCapability { modalities: &["text", "image", "audio", "video"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("qwen2.5-vl", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("qwen2.5-vl-72b-instruct", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("qwen3-14b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("qwen3-235b-a22b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("qwen3-235b-a22b-2507", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-235b-a22b-thinking-2507", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-30b-a3b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("qwen3-30b-a3b-instruct-2507", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-30b-a3b-thinking-2507", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 81920 }),
    ("qwen3-32b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("qwen3-8b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("qwen3-coder", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-coder-30b-a3b-instruct", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-coder-480b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-coder-480b-a35b-instruct", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-coder-flash", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("qwen3-coder-next", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-coder-plus", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("qwen3-max", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-max-thinking", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: None, context: 262144 }),
    ("qwen3-next-80b-a3b-instruct", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-next-80b-a3b-thinking", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-vl", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-vl-235b-a22b-instruct", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-vl-235b-a22b-thinking", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("qwen3-vl-30b-a3b-instruct", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-vl-30b-a3b-thinking", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-vl-32b-instruct", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("qwen3-vl-8b-instruct", ModelCapability { modalities: &["image", "text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3-vl-8b-thinking", ModelCapability { modalities: &["image", "text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("qwen3.5-122b-a10b", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3.5-27b", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3.5-35b-a3b", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3.5-397b-a17b", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3.5-9b", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3.5-flash-02-23", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("qwen3.5-plus", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("qwen3.5-plus-02-15", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("qwen3.5-plus-20", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("qwen3.6-27b", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3.6-35b-a3b", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3.6-flash", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("qwen3.6-max-preview", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("qwen3.6-plus", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("qwen3.7-flash", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("qwen3.7-max", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("qwen3.7-plus", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("qwen3.8-max", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 1000000 }),
    ("reka-edge", ModelCapability { modalities: &["image", "text", "video"], reasoning: false, thinking_effort: None, context: 16384 }),
    ("reka-flash-3", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 65536 }),
    ("relace-apply-3", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 256000 }),
    ("relace-search", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 256000 }),
    ("remm-slerp-l2-13b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 6144 }),
    ("ring-2.6-1t", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("rocinante-12b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 65536 }),
    ("seed-1.6", ModelCapability { modalities: &["image", "text", "video"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("seed-1.6-flash", ModelCapability { modalities: &["image", "text", "video"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("seed-2.0-lite", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("seed-2.0-mini", ModelCapability { modalities: &["text", "image", "video"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("skyfall-36b-v2", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 32768 }),
    ("solar-pro-3", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("sonar", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 127072 }),
    ("sonar-deep-research", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("sonar-pro", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 200000 }),
    ("sonar-pro-search", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 200000 }),
    ("sonar-reasoning-pro", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("step-1o", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: None, context: 32768 }),
    ("step-2.1-256k", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("step-2.5-vl", ModelCapability { modalities: &["text", "image"], reasoning: false, thinking_effort: None, context: 32768 }),
    ("step-3-5-flash-2603", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: None, context: 262144 }),
    ("step-3.5-flash", ModelCapability { modalities: &["text"], reasoning: true, thinking_effort: None, context: 262144 }),
    ("step-3.7-flash", ModelCapability { modalities: &["text", "image", "video"], reasoning: true, thinking_effort: Some(&["low", "medium", "high"]), context: 262144 }),
    ("trinity-large-thinking", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 262144 }),
    ("ui-tars-1.5-7b", ModelCapability { modalities: &["image", "text"], reasoning: false, thinking_effort: None, context: 128000 }),
    ("unslopnemo-12b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 1024000 }),
    ("virtuoso-large", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 131072 }),
    ("voxtral-small-24b-2507", ModelCapability { modalities: &["text", "audio", "file"], reasoning: false, thinking_effort: None, context: 32000 }),
    ("weaver", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 8000 }),
    ("wizardlm-2-8x22b", ModelCapability { modalities: &["text"], reasoning: false, thinking_effort: None, context: 65535 }),
];

/// Normalize a model name for dictionary lookup.
pub(crate) fn normalize_model_name(name: &str) -> String {
    let mut s = name.trim().to_ascii_lowercase();
    if let Some(idx) = s.rfind('/') {
        s = s[idx + 1..].to_string();
    }
    // strip :batch / :free suffix
    if let Some(idx) = s.rfind(':') {
        if s[idx + 1..].chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            s = s[..idx].to_string();
        }
    }
    // strip date suffix like -20251101 or _20251101
    if s.len() >= 6 {
        let tail = &s[s.len() - 6..];
        if tail.chars().all(|c| c.is_ascii_digit()) {
            s = s[..s.len() - 6].trim_end_matches(['-', '_']).to_string();
        }
    }
    s
}

/// Look up capabilities by model name (normalized).
pub(crate) fn get_model_capability(model: &str) -> Option<&'static ModelCapability> {
    let key = normalize_model_name(model);
    MODEL_CAPABILITIES
        .iter()
        .find(|(name, _)| *name == key)
        .map(|(_, cap)| cap)
}

/// Whether a model supports the given modality. Unknown models fail open.
pub(crate) fn model_supports_modality(model: &str, modality: &str) -> bool {
    match get_model_capability(model) {
        Some(cap) => cap.modalities.iter().any(|m| m.eq_ignore_ascii_case(modality)),
        None => true,
    }
}

/// Whether a model is multimodal (supports image/audio/video/pdf besides text).
pub(crate) fn is_model_multimodal(model: &str) -> bool {
    match get_model_capability(model) {
        Some(cap) => cap.modalities.iter().any(|m| *m != "text"),
        None => false,
    }
}

/// Whether a model is a reasoning model.
pub(crate) fn is_reasoning_model(model: &str) -> bool {
    get_model_capability(model).map(|cap| cap.reasoning).unwrap_or(false)
}

/// Supported thinking effort levels for a model.
pub(crate) fn get_thinking_efforts(model: &str) -> Option<&'static [&'static str]> {
    get_model_capability(model).and_then(|cap| cap.thinking_effort)
}

/// Max context tokens for a model (0 = unknown).
pub(crate) fn get_model_context_limit(model: &str) -> u64 {
    get_model_capability(model).map(|cap| cap.context).unwrap_or(0)
}

/// Render a model's capability as a serde_json Value for API responses.
pub(crate) fn model_capability_json(model: &str) -> Value {
    match get_model_capability(model) {
        Some(cap) => {
            let mut m = serde_json::Map::new();
            m.insert("modalities".to_string(), serde_json::json!(cap.modalities));
            m.insert("reasoning".to_string(), serde_json::json!(cap.reasoning));
            m.insert("thinkingEffort".to_string(), serde_json::json!(cap.thinking_effort));
            m.insert("context".to_string(), serde_json::json!(cap.context));
            Value::Object(m)
        }
        None => serde_json::json!(null),
    }
}

/// Render the whole capabilities dictionary as an object keyed by normalized name.
pub(crate) fn model_catalog_json() -> Value {
    let mut map = serde_json::Map::new();
    for (name, cap) in MODEL_CAPABILITIES {
        let mut m = serde_json::Map::new();
        m.insert("modalities".to_string(), serde_json::json!(cap.modalities));
        m.insert("reasoning".to_string(), serde_json::json!(cap.reasoning));
        m.insert("thinkingEffort".to_string(), serde_json::json!(cap.thinking_effort));
        m.insert("context".to_string(), serde_json::json!(cap.context));
        map.insert(name.to_string(), Value::Object(m));
    }
    Value::Object(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_37_flash_is_multimodal() {
        assert!(is_model_multimodal("step-3.7-flash"));
        assert!(model_supports_modality("step-3.7-flash", "image"));
        assert!(model_supports_modality("step-3.7-flash", "video"));
    }

    #[test]
    fn step_35_flash_is_text_only() {
        assert!(!is_model_multimodal("step-3.5-flash"));
        assert!(!model_supports_modality("step-3.5-flash", "image"));
    }

    #[test]
    fn deepseek_is_text_only() {
        assert!(!is_model_multimodal("deepseek/deepseek-v4-flash"));
        assert!(!model_supports_modality("deepseek-v4-flash", "image"));
    }

    #[test]
    fn gpt4o_and_claude_are_multimodal() {
        assert!(is_model_multimodal("openai/gpt-4o"));
        assert!(is_model_multimodal("claude-sonnet-4"));
        assert!(is_model_multimodal("claude-opus-5"));
    }

    #[test]
    fn normalization_strips_vendor_and_suffixes() {
        assert_eq!(normalize_model_name("openai/gpt-4o"), "gpt-4o");
        assert_eq!(normalize_model_name("step-3.5-flash-2603"), "step-3.5-flash");
        assert_eq!(normalize_model_name("gpt-5.6-sol:batch"), "gpt-5.6-sol");
        assert_eq!(normalize_model_name("claude-haiku-4.5"), "claude-haiku-4-5");
    }

    #[test]
    fn reasoning_and_effort_lookups() {
        assert!(is_reasoning_model("gpt-5.4"));
        assert!(!is_reasoning_model("gpt-4o"));
        assert_eq!(get_thinking_efforts("gpt-5.4"), Some(&["none", "low", "medium", "high", "xhigh"] as &[_]));
        assert!(get_model_context_limit("gpt-4o") > 0);
    }
}
