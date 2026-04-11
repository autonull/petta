;Import TS helpers for OpenRouter
; We assume the TS host exposes these via js-call under 'llm' namespace

;Function to prompt GPT via OpenAI API:
(= (useGPT $prompt)
   (useGPT gpt-5.2 1000000 medium $prompt))
(= (useGPT $model $max_tokens $effort $prompt)
   (js-call (llm.use_gpt $model $prompt $max_tokens $effort)))

;Function to prompt a model via OpenRouter API:
(= (useOpenRouter $model $prompt)
   (useOpenRouter $model 100000 medium $prompt))
(= (useOpenRouter $model $max_tokens $effort $prompt)
   (js-call (llm.use_openrouter $model $prompt $max_tokens $effort)))

(= (sread-safe $w)
   (case (catch (sread $w))
         (((Error $A $B) (empty))
          ($z $z))))

(= (useGPTEmbedding $text)
   (js-call (llm.use_gpt_embedding $text)))

(= (useOpenRouterEmbedding $model $text)
   (js-call (llm.use_openrouter_embedding $model $text)))
