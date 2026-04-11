"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.openrouter_chat = openrouter_chat;
exports.openrouter_embed = openrouter_embed;
const ai_1 = require("ai");
const openai_1 = require("@ai-sdk/openai");
const openrouter = (0, openai_1.createOpenAI)({
    baseURL: 'https://openrouter.ai/api/v1',
    apiKey: process.env.OPENROUTER_API_KEY
});
async function openrouter_chat(model, prompt, maxTokens, effort) {
    // Vercel AI SDK generateText doesn't strictly have a maxTokens in base config, let's just pass basic params
    const { text } = await (0, ai_1.generateText)({
        model: openrouter(model),
        prompt,
    });
    return text;
}
async function openrouter_embed(model, text) {
    const { embedding } = await (0, ai_1.embed)({
        model: openrouter.textEmbeddingModel(model),
        value: text
    });
    return embedding;
}
