"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.openrouter_chat = openrouter_chat;
exports.openrouter_embed = openrouter_embed;
const ai_1 = require("ai");
const openai_1 = require("@ai-sdk/openai");
async function openrouter_chat(model, prompt, maxTokens, effort) {
    const { text } = await (0, ai_1.generateText)({
        model: (0, openai_1.openai)(model, {
            baseURL: 'https://openrouter.ai/api/v1',
            apiKey: process.env.OPENROUTER_API_KEY
        }),
        prompt,
        maxTokens,
    });
    return text;
}
async function openrouter_embed(model, text) {
    const { embedding } = await (0, ai_1.embed)({
        model: openai_1.openai.embedding(model, {
            baseURL: 'https://openrouter.ai/api/v1',
            apiKey: process.env.OPENROUTER_API_KEY
        }),
        value: text
    });
    return embedding;
}
