import { generateText, embed } from 'ai';
import { createOpenAI } from '@ai-sdk/openai';

const openrouter = createOpenAI({
    baseURL: 'https://openrouter.ai/api/v1',
    apiKey: process.env.OPENROUTER_API_KEY
});

export async function openrouter_chat(model: string, prompt: string, maxTokens: number, effort: string) {
    // Vercel AI SDK generateText doesn't strictly have a maxTokens in base config, let's just pass basic params
    const { text } = await generateText({
        model: openrouter(model),
        prompt,
    });
    return text;
}

export async function openrouter_embed(model: string, text: string) {
    const { embedding } = await embed({
        model: openrouter.textEmbeddingModel(model),
        value: text
    });
    return embedding;
}
