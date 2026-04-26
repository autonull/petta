!(import! &self lib_llm)
!(import! &self lib_mm2)

; test mork / mm2 space overrides
!(＋ (A B C))
!(? $x (A B C))
!(－ (A B C))

; test llm methods mapping (the js backend doesn't run with keys during tests so we just confirm they don't break with non-existence errors)
!(useGPTEmbedding "test embedding text")
!(useOpenRouter "openai/gpt-4o" "Say hello!")
