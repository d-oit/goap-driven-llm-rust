#!/usr/bin/env python3
"""
SessionStart hook for agent integration.

Initializes session with:
- Available agents from .claude/agents/
- Agent activation patterns
- Project context from AGENTS.md
- Skill references from .claude/skills/

Provides contextual agent recommendations.
"""

import json
import sys
from pathlib import Path

def load_agents():
    """Load available agents from .claude/agents/ directory."""
    agents_dir = Path('.claude/agents')
    agents = {}
    
    if agents_dir.exists():
        for agent_file in agents_dir.glob('*-agent.md'):
            agent_name = agent_file.stem.replace('-agent', '')
            
            try:
                content = agent_file.read_text()
                
                # Extract name and description from frontmatter
                name_match = content.split('name:')[1].split('\n')[0].strip()
                desc_match = content.split('description:')[1].split('\n')[0].strip()
                
                # Extract triggers
                triggers = []
                if 'trigger:' in content:
                    trigger_section = content.split('trigger:')[1].split('---')[0]
                    triggers = [
                        line.strip().strip('"').strip("'").strip('-')
                        for line in trigger_section.split('\n')
                        if line.strip() and not line.strip().startswith('#')
                    ]
                
                agents[agent_name] = {
                    'name': name_match,
                    'description': desc_match,
                    'triggers': triggers,
                    'file': str(agent_file)
                }
            except Exception as e:
                print(f"Warning: Failed to load agent {agent_file}: {e}", file=sys.stderr)
    
    return agents

def load_project_context():
    """Load project context from documentation files."""
    context = {}
    
    # Load AGENTS.md
    agents_md = Path('AGENTS.md')
    if agents_md.exists():
        context['agents_md'] = agents_md.read_text()      
    
    # Load Cargo.toml for dependencies
    cargo_toml = Path('Cargo.toml')
    if cargo_toml.exists():
        context['cargo_toml'] = cargo_toml.read_text()
    
    return context

def generate_agent_summary(agents):
    """Generate a summary of available agents."""
    summary = "\n🤖 Available Claude Code Agents:\n\n"
    
    for agent_name, agent_data in sorted(agents.items()):
        summary += f"**{agent_data['name']}**\n"
        summary += f"  {agent_data['description']}\n"
        
        if agent_data['triggers']:
            summary += f"  Triggers: {', '.join(agent_data['triggers'][:3])}"
            if len(agent_data['triggers']) > 3:
                summary += f" (and {len(agent_data['triggers']) - 3} more)"
            summary += "\n"
        
        summary += "\n"
    
    return summary

def main():
    """Main hook execution."""
    print("🚀 Initializing Claude Code with GOAP Project Agents...", file=sys.stderr)
    
    # Load agents
    agents = load_agents()
    
    if not agents:
        print("⚠ No agents found in .claude/agents/", file=sys.stderr)
        sys.exit(0)
    
    # Load project context
    context = load_project_context()
    
    # Generate and print agent summary
    summary = generate_agent_summary(agents)
    print(summary, file=sys.stderr)
    
    # Print project-specific guidance
    if 'cargo_toml' in context:
        print("📦 Detected Rust project with dependencies:", file=sys.stderr)
        
        if 'tokio' in context['cargo_toml']:
            print("  ✓ Async runtime: tokio", file=sys.stderr)
        if 'serde' in context['cargo_toml']:
            print("  ✓ Serialization: serde", file=sys.stderr)
        if 'tracing' in context['cargo_toml']:
            print("  ✓ Logging: tracing", file=sys.stderr)
        
        print("\n💡 Use rust-clean-code-agent for Rust-specific guidance", file=sys.stderr)
    
    # Print agent recommendations
    print("\n🎯 Quick Start Recommendations:", file=sys.stderr)
    print("  • Say 'commit these changes' → activates git-pushing-agent", file=sys.stderr)
    print("  • Say 'test my webapp' → activates webapp-testing-agent", file=sys.stderr)
    print("  • Say 'verify before completion' → activates verification-agent", file=sys.stderr)
    
    print("\n✅ Agent integration initialized successfully", file=sys.stderr)
    
    sys.exit(0)

if __name__ == '__main__':
    main()