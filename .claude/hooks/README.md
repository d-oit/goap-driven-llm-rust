# Claude Code Hooks

This directory contains Claude Code hooks that enhance development workflows based on the GOAP codebase patterns and Rust best practices.

## Hook Structure

Each hook is a Python script that:
- Receives JSON input from Claude Code
- Analyzes the context
- Performs validation or enhancement
- Returns exit codes for control flow:
  - `0`: Success, continue execution
  - `1`: Warning, show stderr but continue
  - `2`: Error, block execution

## Available Hooks

### Pre-Tool-Use Hooks
- `rust-test-validator.py`: Validates test requirements for Rust code
- `clippy-check.py`: Enforces clippy linting standards
- `rustfmt-check.py`: Validates code formatting
- `benchmark-validator.py`: Ensures benchmark targets are met

### Post-Tool-Use Hooks
- `coverage-analyzer.py`: Analyzes test coverage metrics
- `documentation-checker.py`: Validates documentation completeness
- `agent-integration-check.py`: Checks agent/skill integration

### Session Hooks
- `session-init.py`: Initializes session with project context
- `context-manager.py`: Manages context and memory usage

## Configuration

Hooks are configured in `.claude/settings.json` with:
```json
{
  "hooks": {
    "PreToolUse": [...],
    "PostToolUse": [...],
    "SessionStart": [...]
  }
}
```

## Usage

Each hook can be run directly for testing:
```bash
python hooks/<hook-name>.py < input.json
```

Or configured to run automatically via Claude Code settings.