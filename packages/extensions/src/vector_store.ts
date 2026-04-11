export class VectorStore {
    private vectors: { id: string; vector: number[] }[] = [];

    add(id: string, vector: number[]) {
        this.vectors.push({ id, vector });
    }

    search(query: number[], k: number = 5) {
        return this.vectors
            .map(v => ({
                id: v.id,
                similarity: this.cosineSimilarity(query, v.vector)
            }))
            .sort((a, b) => b.similarity - a.similarity)
            .slice(0, k);
    }

    private cosineSimilarity(a: number[], b: number[]): number {
        let dotProduct = 0;
        let normA = 0;
        let normB = 0;
        for (let i = 0; i < a.length; i++) {
            dotProduct += a[i] * b[i];
            normA += a[i] * a[i];
            normB += b[i] * b[i];
        }
        return dotProduct / (Math.sqrt(normA) * Math.sqrt(normB));
    }
}
