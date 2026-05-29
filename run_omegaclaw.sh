#!/bin/bash
# OmegaClaw IRC Bot Launcher
# Usage: ./run_omegaclaw.sh [OPTIONS]
#
# Options:
#   --repl              Run in CLI REPL mode (no IRC, reads from stdin/stdout)
#   --server <host>    IRC server (default: irc.quakenet.org)
#   --port <port>      IRC port (default: 6667)
#   --channel <ch>     Channel to join (default: ##metta)
#   --nick <nick>      Bot nickname (default: omegaclaw)
#   --provider <prov>  LLM provider (default: Ollama)
#   --model <model>    Model name (default: from env)
#
# Environment variables:
#   OLLAMA_API_KEY          Required for Ollama (any non-empty value)
#   OLLAMA_MODEL            Ollama model name
#   LLM_SERVER_LOCAL_URL    Ollama server URL (default: http://localhost:11434)
#   ANTHROPIC_API_KEY       For Anthropic provider
#   OPENAI_API_KEY          For OpenAI provider
#   ASIC_API_KEY            For ASICloud provider
#   ASIONE_API_KEY          For ASIOne provider
#   OPENROUTER_API_KEY      For OpenRouter provider
#   OMEGACLAW_AUTH_SECRET   IRC auth secret (optional)

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PETTA="$SCRIPT_DIR/target/release/petta"

# Defaults
REPL_MODE=false
SERVER="irc.quakenet.org"
PORT=6667
CHANNEL="##metta"
NICK="omegaclaw"
PROVIDER="Ollama"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --help) echo "Usage: $0 [--repl] [--server <host>] [--port <port>] [--channel <ch>] [--nick <nick>] [--provider <prov>] [--model <model>]"; exit 0 ;;
        --repl) REPL_MODE=true; shift ;;
        --server) SERVER="$2"; shift 2 ;;
        --port) PORT="$2"; shift 2 ;;
        --channel) CHANNEL="$2"; shift 2 ;;
        --nick) NICK="$2"; shift 2 ;;
        --provider) PROVIDER="$2"; shift 2 ;;
        --model) MODEL="$2"; shift 2 ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

# Check for compiled binary
if [[ ! -f "$PETTA" ]]; then
    echo "Building PeTTa..."
    cd "$SCRIPT_DIR"
    cargo build --release --features "repl,websocket"
fi

# Validate required env
if [[ "$PROVIDER" == "Ollama" ]]; then
    if [[ -z "$OLLAMA_API_KEY" ]]; then
        export OLLAMA_API_KEY="not-needed"
    fi
    if [[ -z "$OLLAMA_MODEL" && -z "$MODEL" ]]; then
        echo "Warning: OLLAMA_MODEL not set, using qwen3.5:9b"
        export OLLAMA_MODEL="${OLLAMA_MODEL:-qwen3.5:9b}"
    fi
fi

if [[ "$REPL_MODE" == true ]]; then
    echo "Starting OmegaClaw in REPL mode..."
    echo "  Provider: $PROVIDER"
    echo "  Type your messages at the prompt. Press Ctrl+C to exit."
    echo ""

    exec "$PETTA" run_omegaclaw.metta \
        provider="$PROVIDER" \
        ${MODEL:+model="$MODEL"}
else
    echo "Starting OmegaClaw..."
    echo "  Server:   $SERVER:$PORT"
    echo "  Channel:  $CHANNEL"
    echo "  Nick:     $NICK"
    echo "  Provider: $PROVIDER"
    echo ""

    exec "$PETTA" run_omegaclaw.metta \
        provider="$PROVIDER" \
        IRC_server="$SERVER" \
        IRC_port="$PORT" \
        IRC_channel="$CHANNEL" \
        IRC_user="$NICK" \
        ${MODEL:+model="$MODEL"}
fi