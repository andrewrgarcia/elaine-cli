<!-- LOGO -->
<p align="center">
  <!-- TODO: Replace with Elaine logo -->
  <img width="100" alt="elaine" src="https://github.com/user-attachments/assets/1d4059ad-5d4d-4695-930b-bedc4fc149f4" />
</p>

<h1 align="center">Elaine</h1>

<p align="center">
  <a href="https://crates.io/crates/elaine-cli"><img src="https://img.shields.io/crates/v/elaine-cli.svg" /></a>
  <a href="https://github.com/andrewrgarcia/elaine-cli"><img src="https://img.shields.io/github/stars/andrewrgarcia/elaine-cli" /></a>
  <a href="#"><img src="https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-blue" /></a>
  <a href="#"><img src="https://img.shields.io/badge/status-v0.2.0-green" /></a>
</p>

<p align="center">
  <strong>An opinionated, local-first reference manager for TeX users.</strong><br/>
  Add references fast. Organize by project. Print clean, deterministic BibTeX.
</p>

<p align="center">
  <img width="700" alt="elaine_demo" src="https://github.com/user-attachments/assets/af9c5ac1-ca70-4615-8af9-52a8acff9f2d" />
</p>

---

## Elaine — CLI Reference Management for Researchers

Elaine is a lightweight command-line reference manager designed for researchers, engineers, and writers who work directly with **LaTeX / BibTeX** and want **clarity, determinism, and local ownership**.

Elaine does not try to be a PDF library, a cloud sync tool, or a GUI replacement for Zotero.  
It focuses on **one thing**: managing references cleanly and compiling reliable `.bib` files.

---

## Why Elaine exists

Most reference managers:

- Store PDFs you don’t need
- Depend on cloud sync
- Hide metadata behind GUIs
- Produce noisy or unstable BibTeX

Elaine takes a different approach:

- References are **atomic YAML files**
- Projects are **explicit collections**
- BibTeX output is **deterministic**
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

A reference can belong to **multiple projects** without duplication,
and can be emitted once when compiling a multi-project or global bibliography.

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

#### 1. BibTeX via stdin

```bash
eln add < references.bib
```

Elaine will:

* parse metadata
* store reference atoms
* attach them to the active project
* prompt before overwriting existing IDs

---

#### 2. Manual add (fast)

```bash
eln add "Title" "Author One and Author Two" 2024
```

Only **title** and **author(s)** are required.
Year is optional.

---

#### 3. Interactive mode

```bash
eln add -i
```

Guided, prompt-based entry for all supported metadata fields
(press Enter to skip optional fields).

---

### Manage projects

```bash
eln pro <project-name>
```

Creates or switches the active project.

```bash
eln pro
```

Lists all projects and highlights the active one.

---

### View status

```bash
eln status
```

Shows all projects and reference counts.

```bash
eln status -v
```

Verbose mode: includes reference IDs per project.

---

### Remove references

```bash
eln rm <ref-id>
```

Removes a reference from the active project.

If the reference is unused globally, Elaine will ask whether to delete
the reference file as well.

---

### Print bibliography

#### 1. Active project

```bash
eln printed
```

Generates a deterministic BibTeX file for the active project:

```
<project>_references.bib
```

The same content is also printed to stdout.

---

#### 2. Multiple projects (set union)

```bash
eln printed projectA projectB
```

Generates a single BibTeX file containing the **union** of references
across the specified projects:

```
projectA+projectB_references.bib
```

If the same reference appears in multiple projects, it is emitted **once**.

---

#### 3. Global bibliography

```bash
eln printed --all
```

Generates a global bibliography containing **all references across all projects**:

```
global_references.bib
```

This file is always named explicitly to avoid overwriting curated
project-level bibliographies.


---

## Example Workflow

```bash
eln init
eln pro crystal_growth_review

eln add < references.bib
eln add "No Free Lunch Theorems" "Wolpert, D.H. and Macready, W.G." 1997
eln add -i

eln status
eln printed

# Multi-project bibliography
eln printed crystal_growth_review background

# Global bibliography (all projects)
eln printed --all

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
* **Explicit scope** — global and multi-project actions are always opt-in

---

## Status

Elaine is currently **v0.2.0**.

The core lifecycle is complete:

* add (BibTeX / manual / interactive)
* edit (overwrite with confirmation)
* remove (project-safe)
* project scoping
* deterministic BibTeX emission

The on-disk schema may evolve before v1.0.0, but no accidental breakage is expected.

---

## Roadmap

Planned improvements include:

* `eln edit <ref-id>` (interactive editing)
* `eln ls` (list refs in active project)
* `eln find <query>`
* Validation / linting (`eln check`)
* Optional editor integration (`$EDITOR`)
* Schema versioning and migrations

---

## License

MIT License
