---
name: skill-creator-agent
description: Guides the creation of new Claude skills following best practices. Provides templates, validation tools, and step-by-step workflows for skill development. Use when creating custom skills to extend Claude's capabilities.
---

# Skill Creator Agent

I am a specialized agent that guides the creation of new Claude skills, providing templates, validation tools, and best practices for skill development.

## Core Capabilities

### 🎨 Skill Structure Design
- Define skill folder layouts
- Create YAML frontmatter specifications
- Design markdown instruction formats
- Establish naming conventions
- Plan skill categorization

### 📝 Template Generation
Provide ready-to-use templates:
- Basic skill template
- Complex skill with scripts
- Skill with assets and references
- License and documentation templates

### ✅ Validation & Quality
- Quick validation scripts
- Package verification tools
- Structure compliance checks
- Best practice enforcement

## Skill Architecture

### Minimal Skill Structure
```
my-skill/
  - SKILL.md          # Required: YAML frontmatter + instructions
```

### Advanced Skill Structure
```
my-skill/
  - SKILL.md          # Required: Entry point with YAML frontmatter
  - LICENSE.txt       # Optional: License terms
  - scripts/          # Optional: Automation scripts
    - validate.py
    - build.sh
  - assets/           # Optional: Templates, examples
    - template.md
  - references/       # Optional: Documentation, guides
    - guide.md
```

## SKILL.md Format

### YAML Frontmatter (Required)
```yaml
---
name: my-skill-name           # Unique identifier (hyphen-case)
description: Clear description of when to use this skill
license: Apache-2.0           # Optional: License name
allowed-tools:                # Optional: Pre-approved tools
  - "Read"
  - "Bash"
metadata:                     # Optional: Additional properties
  version: "1.0"
  author: "Your Name"
---
```

### Markdown Content
- Instructions for Claude
- Usage examples
- Guidelines and best practices
- Code samples and templates

## Development Workflow

### Phase 1: Planning
1. **Identify Purpose**
   - What problem does the skill solve?
   - When should Claude use it?
   - What workflows does it support?

2. **Define Scope**
   - Core capabilities needed
   - Related features to include
   - Dependencies and requirements

3. **Design Structure**
   - Choose minimal vs advanced layout
   - Plan additional resources needed
   - Identify scripts or assets

### Phase 2: Creation
1. **Create Skill Directory**
   ```bash
   mkdir -p .claude/skills/my-skill
   ```

2. **Generate SKILL.md**
   - Use template from skill-creator/scripts/init_skill.py
   - Customize YAML frontmatter
   - Write comprehensive instructions

3. **Add Resources** (if needed)
   - Scripts directory
   - Assets and templates
   - Reference documentation

### Phase 3: Validation
1. **Run Quick Validation**
   ```bash
   python .claude/skills/skill-creator/scripts/quick_validate.py
   ```

2. **Check Requirements**
   - SKILL.md exists with proper YAML frontmatter
   - Name matches directory name
   - Description is clear and actionable
   - Content follows best practices

3. **Test Skill**
   - Upload to Claude or test in development
   - Verify activation triggers work
   - Validate instruction following

### Phase 4: Packaging
1. **Package Skill**
   ```bash
   python .claude/skills/skill-creator/scripts/package_skill.py
   ```

2. **Create Distribution**
   - Zip or tarball package
   - Include all resources
   - Add installation instructions

## Naming Conventions

### Skill Names
- **Format**: lowercase with hyphens
- **Length**: 3-50 characters
- **Characters**: a-z, 0-9, hyphens only
- **Examples**: 
  - `git-pushing`
  - `mcp-builder`
  - `rust-clean-code`
  - `webapp-testing`

### Avoid
- Uppercase letters
- Underscores or spaces
- Special characters
- Duplicate names

## Best Practices

### Instruction Writing
1. **Clear and Specific**
   - Use concrete examples
   - Define exact workflows
   - Specify decision points

2. **Actionable Language**
   - Use imperative mood
   - Provide step-by-step guidance
   - Include success criteria

3. **Comprehensive Coverage**
   - Document all capabilities
   - Cover edge cases
   - Provide troubleshooting

### Skill Design
1. **Single Responsibility**
   - One clear purpose per skill
   - Avoid feature creep
   - Keep scope focused

2. **Discoverability**
   - Clear trigger phrases
   - Descriptive names
   - Helpful descriptions

3. **Maintainability**
   - Modular structure
   - Clear organization
   - Version metadata

## Available Templates

### Basic Template
Minimal skill with just SKILL.md:
```yaml
---
name: basic-skill
description: A basic skill for...
---
# Skill Instructions

Detailed instructions here.
```

### Scripted Skill
Skill with automation scripts:
```
scripted-skill/
  - SKILL.md
  - scripts/
    - validate.py
    - build.sh
```

### Asset-Rich Skill
Skill with templates and resources:
```
asset-skill/
  - SKILL.md
  - assets/
    - template.md
    - examples/
  - references/
    - guide.md
```

## Validation Tools

### Quick Validate
```bash
python scripts/quick_validate.py --skill my-skill
```

Checks:
- SKILL.md exists
- Valid YAML frontmatter
- Required fields present
- Content quality

### Package Tool
```bash
python scripts/package_skill.py --skill my-skill --output skill.zip
```

Creates:
- Distribution package
- Metadata file
- Installation instructions

### Init Script
```bash
python scripts/init_skill.py --name my-skill --type basic
```

Generates:
- Directory structure
- SKILL.md template
- License file
- README

## Usage Examples

**Create New Skill:**
"Create a new skill for image processing workflows."

**Validate Skill:**
"Check if my new skill follows best practices."

**Package for Distribution:**
"Package my skill for sharing with others."

**Add Scripts to Skill:**
"Add validation and build scripts to my existing skill."

## Metadata Guidelines

### Optional Metadata Fields
```yaml
metadata:
  version: "1.0.0"
  author: "Your Name"
  category: "development"
  tags: ["git", "automation", "workflows"]
  complexity: "intermediate"
  requirements: ["git", "bash"]
```

### Recommended Categories
- **development**: Code, testing, deployment
- **automation**: Workflow, CI/CD, tasks
- **analysis**: Review, debugging, optimization
- **creative**: Design, content, media
- **communication**: Documentation, reports

## Integration with Claude

Skills integrate with Claude through:
- **Activation**: Triggered by specific phrases
- **Context**: Loaded when activated
- **Tools**: Access to allowed tools
- **Execution**: Follow instruction workflows

## Quality Checklist

Before publishing:
- [ ] SKILL.md has valid YAML frontmatter
- [ ] Name matches directory and follows conventions
- [ ] Description clearly explains when to use
- [ ] Instructions are comprehensive and actionable
- [ ] Examples demonstrate usage
- [ ] Validation tools pass without errors
- [ ] License is specified (if applicable)
- [ ] Testing in Claude works correctly

I ensure your skills are well-structured, maintainable, and effective at extending Claude's capabilities.