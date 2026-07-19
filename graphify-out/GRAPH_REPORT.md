# Graph Report - .  (2026-07-19)

## Corpus Check
- Corpus is ~17,102 words - fits in a single context window. You may not need a graph.

## Summary
- 195 nodes · 271 edges · 43 communities (21 shown, 22 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 13 edges (avg confidence: 0.9)
- Token cost: 5,068 input · 0 output

## Community Hubs (Navigation)
- [[_COMMUNITY_Build & Target Config|Build & Target Config]]
- [[_COMMUNITY_Config Resolution|Config Resolution]]
- [[_COMMUNITY_Architecture Docs|Architecture Docs]]
- [[_COMMUNITY_CLI Commands & Compiler|CLI Commands & Compiler]]
- [[_COMMUNITY_Module References|Module References]]
- [[_COMMUNITY_Graphify Query & Export|Graphify Query & Export]]
- [[_COMMUNITY_CLI Dispatch|CLI Dispatch]]
- [[_COMMUNITY_Project Templates|Project Templates]]
- [[_COMMUNITY_Post-Edit Hook|Post-Edit Hook]]
- [[_COMMUNITY_Build Command|Build Command]]
- [[_COMMUNITY_Debug Command|Debug Command]]
- [[_COMMUNITY_Disasm Command|Disasm Command]]
- [[_COMMUNITY_Run Command|Run Command]]
- [[_COMMUNITY_Sections Command|Sections Command]]
- [[_COMMUNITY_Symbols Command|Symbols Command]]
- [[_COMMUNITY_Lint & Format Hooks|Lint & Format Hooks]]
- [[_COMMUNITY_Changelog & Commits|Changelog & Commits]]
- [[_COMMUNITY_Graphify Incremental|Graphify Incremental]]
- [[_COMMUNITY_Clean Command|Clean Command]]
- [[_COMMUNITY_New Command|New Command]]
- [[_COMMUNITY_Watch Command|Watch Command]]
- [[_COMMUNITY_Main Entry|Main Entry]]
- [[_COMMUNITY_Changelog Script|Changelog Script]]
- [[_COMMUNITY_Embedded Targets|Embedded Targets]]
- [[_COMMUNITY_FalkorDB Export|FalkorDB Export]]
- [[_COMMUNITY_Neo4j Export|Neo4j Export]]
- [[_COMMUNITY_Wiki Export|Wiki Export]]
- [[_COMMUNITY_Extraction Subagent|Extraction Subagent]]
- [[_COMMUNITY_Cross-Repo Merge|Cross-Repo Merge]]
- [[_COMMUNITY_GitHub Clone|GitHub Clone]]
- [[_COMMUNITY_Claude Install Hook|Claude Install Hook]]
- [[_COMMUNITY_rv Tool Identity|rv Tool Identity]]
- [[_COMMUNITY_Crate Config|Crate Config]]
- [[_COMMUNITY_Disasm Usage|Disasm Usage]]
- [[_COMMUNITY_Watch Usage|Watch Usage]]
- [[_COMMUNITY_Plugin System Roadmap|Plugin System Roadmap]]
- [[_COMMUNITY_v0.1 Foundation|v0.1 Foundation]]
- [[_COMMUNITY_v0.2 Mixed Language|v0.2 Mixed Language]]
- [[_COMMUNITY_v0.3 Inspection|v0.3 Inspection]]

## God Nodes (most connected - your core abstractions)
1. `Config` - 28 edges
2. `run()` - 8 edges
3. `run_command()` - 8 edges
4. `Qemu` - 8 edges
5. `Command` - 7 edges
6. `link()` - 7 edges
7. `Graphify` - 7 edges
8. `compile_asm()` - 6 edges
9. `compile_c()` - 6 edges
10. `Build` - 6 edges

## Surprising Connections (you probably didn't know these)
- `rv.toml Configuration` --semantically_similar_to--> `Configuration System (rv.toml)`  [INFERRED] [semantically similar]
  README.md → ARCHITECTURE.md
- `Project Philosophy` --semantically_similar_to--> `Design Philosophy`  [INFERRED] [semantically similar]
  CONTRIBUTING.md → ARCHITECTURE.md
- `Conventional Commits` --conceptually_related_to--> `git-cliff`  [INFERRED]
  CONTRIBUTING.md → .claude/HOOKS.md
- `rv CLI` --implements--> `Clap (CLI Framework)`  [EXTRACTED]
  ARCHITECTURE.md → CLAUDE.md
- `rv (RISC-V Assembly/C CLI)` --references--> `rv CLI`  [EXTRACTED]
  README.md → ARCHITECTURE.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **rv CLI Control Flow** — architecture_main_rs, architecture_cli_mod_rs, architecture_config_mod, architecture_commands_mod, architecture_compiler_gcc, architecture_qemu_run, architecture_gdb_debug, architecture_templates_mod [EXTRACTED 1.00]
- **rv Subcommands** — readme_rv_new, readme_rv_build, readme_rv_run, readme_rv_debug, readme_rv_disasm, readme_rv_watch [EXTRACTED 1.00]
- **rv Target Configurations** — readme_bare_metal_target, readme_linux_user_mode_target, readme_esp32_c6_target [EXTRACTED 1.00]
- **Graphify Pipeline Components** — _claude_skills_graphify_skill_graphify, _claude_skills_graphify_skill_knowledge_graph, _claude_skills_graphify_skill_community_detection, _claude_skills_graphify_skill_god_nodes, _claude_skills_graphify_references_query_bfs_traversal, _claude_skills_graphify_references_query_query_expansion [EXTRACTED 1.00]
- **Graphify Export Formats** — _claude_skills_graphify_references_exports_wiki_export, _claude_skills_graphify_references_exports_neo4j_export, _claude_skills_graphify_references_exports_falkordb_export, _claude_skills_graphify_references_exports_mcp_server [EXTRACTED 1.00]
- **Claude Code Hooks** — _claude_hooks_post_edit_hook, _claude_hooks_post_commit_changelog_hook, _claude_hooks_cargo_fmt, _claude_hooks_cargo_clippy, _claude_hooks_git_cliff [EXTRACTED 1.00]
- **rv Roadmap Milestones** — roadmap_v0_1_foundation, roadmap_v0_2_mixed_language, roadmap_v0_3_inspection_debugging, roadmap_v0_4_bare_metal, roadmap_v0_5_embedded_targets [EXTRACTED 1.00]

## Communities (43 total, 22 thin omitted)

### Community 0 - "Build & Target Config"
Cohesion: 0.14
Nodes (27): Default, Build, default_abi(), default_arch(), default_build_dir(), default_cc(), default_gdb(), default_link_driver() (+19 more)

### Community 1 - "Config Resolution"
Cohesion: 0.22
Nodes (10): Compile, Config, PathBuf, Result, Path, Result, run(), Path (+2 more)

### Community 2 - "Architecture Docs"
Cohesion: 0.12
Nodes (16): Build Pipeline, Configuration System (rv.toml), Design Philosophy, Link Drivers, rv CLI, Target Abstraction, Clap (CLI Framework), Serde (+8 more)

### Community 3 - "CLI Commands & Compiler"
Cohesion: 0.29
Nodes (15): Command, Option, String, compile_asm(), compile_c(), eprint_command(), link(), obj_path() (+7 more)

### Community 4 - "Module References"
Cohesion: 0.13
Nodes (15): cli/mod.rs (Clap CLI), commands/ Module, compiler/gcc.rs, config/mod.rs, gdb/debug.rs, main.rs Entry Point, qemu/run.rs, templates/mod.rs (+7 more)

### Community 5 - "Graphify Query & Export"
Cohesion: 0.18
Nodes (11): Graphify Add (URL Ingest), MCP Server, BFS Traversal, DFS Traversal, Constrained Query Expansion, Whisper Transcription, Community Detection, God Nodes (+3 more)

### Community 6 - "CLI Dispatch"
Cohesion: 0.40
Nodes (3): Cli, Result, Self

### Community 7 - "Project Templates"
Cohesion: 0.50
Nodes (3): String, rv_toml(), starter_asm()

### Community 9 - "Build Command"
Cohesion: 0.67
Nodes (3): Option, Result, run()

### Community 10 - "Debug Command"
Cohesion: 0.67
Nodes (3): Option, Result, run()

### Community 11 - "Disasm Command"
Cohesion: 0.50
Nodes (3): Option, Result, run()

### Community 12 - "Run Command"
Cohesion: 0.67
Nodes (3): Option, Result, run()

### Community 13 - "Sections Command"
Cohesion: 0.50
Nodes (3): Option, Result, run()

### Community 14 - "Symbols Command"
Cohesion: 0.50
Nodes (3): Option, Result, run()

### Community 15 - "Lint & Format Hooks"
Cohesion: 0.67
Nodes (3): cargo clippy, cargo fmt, Post-Edit Hook

### Community 16 - "Changelog & Commits"
Cohesion: 0.67
Nodes (3): git-cliff, Post-Commit Changelog Hook, Conventional Commits

### Community 17 - "Graphify Incremental"
Cohesion: 0.67
Nodes (3): Graphify Watch (Auto-Rebuild), Graphify Post-Commit Hook, Incremental Update

## Knowledge Gaps
- **47 isolated node(s):** `post-commit-changelog.sh script`, `Graphify Skill`, `Post-Commit Changelog Hook`, `cargo fmt`, `cargo clippy` (+42 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **22 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Config` connect `Config Resolution` to `Build & Target Config`, `CLI Commands & Compiler`?**
  _High betweenness centrality (0.106) - this node is a cross-community bridge._
- **Why does `Command` connect `CLI Commands & Compiler` to `CLI Dispatch`?**
  _High betweenness centrality (0.031) - this node is a cross-community bridge._
- **What connects `post-commit-changelog.sh script`, `Graphify Skill`, `Post-Commit Changelog Hook` to the rest of the system?**
  _48 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Build & Target Config` be split into smaller, more focused modules?**
  _Cohesion score 0.1408199643493761 - nodes in this community are weakly interconnected._
- **Should `Architecture Docs` be split into smaller, more focused modules?**
  _Cohesion score 0.125 - nodes in this community are weakly interconnected._
- **Should `Module References` be split into smaller, more focused modules?**
  _Cohesion score 0.13333333333333333 - nodes in this community are weakly interconnected._