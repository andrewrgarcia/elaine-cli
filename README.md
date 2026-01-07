
<!-- LOGO -->
<p align="center">
  <!-- TODO: Replace with Elaine logo -->
  <img width="150" alt="elaine" src="https://github.com/user-attachments/assets/1d4059ad-5d4d-4695-930b-bedc4fc149f4" />
</p>


<h1 align="center">Elaine</h1>

<p align="center">
  <a href="https://crates.io/crates/elaine-cli"><img src="https://img.shields.io/crates/v/elaine-cli.svg" /></a>
  <a href="https://github.com/andrewrgarcia/elaine-cli"><img src="https://img.shields.io/github/stars/andrewrgarcia/elaine-cli" /></a>
  <a href="#"><img src="https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-blue" /></a>
  <a href="#"><img src="https://img.shields.io/badge/status-v0.1.0-green" /></a>
</p>

<p align="center">
  <strong>An opinionated, local-first reference manager for TeX users.</strong><br/>
  Add BibTeX fast. Organize by project. Print clean, deterministic bibliographies.
</p>

---

## Elaine — CLI Reference Management for Researchers

Elaine is a lightweight command-line reference manager designed for researchers, engineers, and writers who work directly with **LaTeX / BibTeX** and want **clarity, determinism, and local ownership**.

Elaine does not try to be a PDF library, a cloud sync tool, or a GUI replacement for Zotero.  
It focuses on **one thing**: managing references cleanly and printing reliable `.bib` files.

---

## Why Elaine exists

Most reference managers:

- Store PDFs you don’t need
- Depend on cloud sync
- Hide metadata behind GUIs
- Produce noisy, unstable BibTeX

Elaine takes a different approach:

- References are **atomic YAML files**
- Projects are **explicit collections of references**
- BibTeX output is **deterministic and minimal**
- Everything is **local, transparent, and versionable**

---

## Core Concepts

### Reference atoms

Each reference is stored as a single YAML file:

```

.elaine/refs/<reference-id>.yaml

```

This makes references:
- editable
- diffable
- reusable across projects

---

### Projects

Projects are named collections of references:

```

.elaine/projects/<project>.yaml

````

A reference can belong to **multiple projects** without duplication.

---

### Opinionated ingestion

Elaine intentionally enforces a **simple BibTeX grammar**.

**Requirement:**
> Each BibTeX entry must end with a closing brace `}`.

This constraint ensures deterministic parsing and avoids silent metadata corruption.

If parsing fails, Elaine tells you exactly why.

---

## Installation

### From crates.io

```bash
cargo install elaine-cli
````

### From source

```bash
cargo install --path . --force
```

---

## Project Structure

```
.elaine/
 ├── index.yaml              # active project pointer
 ├── projects/               # project definitions
 │    └── <project>.yaml
 └── refs/                   # reference atoms
      └── <ref-id>.yaml
```

All files are plain YAML.

---

## Core Commands

### Initialize

```bash
eln init
```

Creates the `.elaine/` directory structure.

---

### Add references

```bash
eln add
```

Paste one or more BibTeX entries via stdin.

Elaine will:

* parse metadata
* store reference atoms
* attach them to the active project
* prompt before overwriting existing IDs

---

### Manage projects

```bash
eln pro <project-name>
```

Create or switch the active project.

---

### Print bibliography

```bash
eln printed
```

Generates a deterministic BibTeX file:

```
<project>_references.bib
```

And prints the same content to the CLI.

Example output:

```text
🖨️  Printed 14 references → crystal_growth_review_references.bib
```

---

## Example Workflow

```bash
# Initialize
eln init

# Create / switch project
eln pro crystal_growth_review

# Add references
eln add < references.bib

# Print bibliography
eln printed

# Use in LaTeX
\\bibliography{crystal_growth_review_references}
```

---

## Design Principles

Elaine is built around a few non-negotiables:

* **Local-first** — no cloud, no accounts
* **Deterministic output** — same input, same `.bib`
* **Opinionated parsing** — explicit rules, no silent failure
* **Minimal surface area** — fewer commands, fewer flags
* **Researcher-friendly** — works with Git, LaTeX, and editors

---

## Status

Elaine is currently **v0.1.0**.

The core workflow is stable:

* ingestion
* storage
* project scoping
* BibTeX emission

The on-disk schema may evolve before v1.0.0, but no accidental breakage is expected.

---

## Roadmap

Planned improvements include:

* Optional `--stdout` / `-o` flags for `printed`
* Reference inspection commands (`eln show <id>`)
* Linting / validation (`eln check`)
* Schema versioning and migrations
* Optional integration helpers (Overleaf, CI)

---

## License

MIT License
