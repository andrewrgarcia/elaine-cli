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
  <a href="#"><img src="https://img.shields.io/badge/status-v0.5.0-green" /></a>
</p>

<p align="center">
  <strong>An opinionated, local-first, CLI reference manager for LaTeX / BibTeX users.</strong><br/>
  Deterministic metadata. Explicit libraries. Local documents, on your terms.
</p>

<p align="center">
  <img width="700" alt="elaine_demo" src="https://github.com/user-attachments/assets/af9c5ac1-ca70-4615-8af9-52a8acff9f2d" />
</p>

---

## Elaine — CLI Reference Management for Researchers

Elaine is a lightweight command-line reference manager designed for researchers, engineers, and writers who:

* write LaTeX directly
* want deterministic BibTeX
* want full local ownership
* want references to behave like code and data

Elaine does not try to be a PDF library, a cloud sync tool, or a PDF hoarder.
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
- Libraries are **explicit collections**
- BibTeX output is **deterministic**
- Everything is **local, transparent, and versionable**

---

## Core ideas

### Reference atoms

Each reference is a **single YAML file**:

```
.elaine/refs/<reference-id>.yaml
```

References are:

* atomic
* editable
* diffable
* reusable across libraries

Each reference has:

* a **semantic ID** (human-readable, derived)
* an **opaque SID** (UUID v4, stable and collision-free)

---

### Libraries

Libraries are explicit collections of references:

```
.elaine/libraries/<library>.yaml
```

A reference can belong to **multiple libraries** without duplication.

Libraries also have opaque SIDs.

---

### Selectors (IDs & SIDs)

Anywhere Elaine expects a reference or library, you may use:

* full ID
* full SID
* **unique prefix** of either

Examples:

```bash
eln edit rush1988
eln open 55b3ed28
eln pin 9c2128b9 crystal
eln lib ef67
eln lib crystal
eln lib --delete d084
```

Ambiguous prefixes are rejected explicitly.

---

## Attachments (PDFs, local artifacts)

Elaine supports **linking local documents** (e.g. PDFs) to references.

Attachments are:

* filesystem paths (absolute or relative)
* never copied, moved, or synced
* optional and explicit

### Attach a document

```bash
eln attach <ref-selector> /path/to/paper.pdf
```

### Open an attachment

```bash
eln open <ref-selector>
```

Opens the **first attachment** using the system default viewer.

### Detach attachments

```bash
eln detach <ref-selector>        # remove first attachment
eln detach <ref-selector> 2      # remove attachment at index
eln detach <ref-selector> --all  # remove all attachments
```

In verbose status output, references with attachments are marked:

```
📄
```

---

## Installation

### From crates.io

```bash
cargo install elaine-cli
```

### From source

```bash
cargo install --path . --force
```

---

## Project layout

```
.elaine/
 ├── index.yaml              # active library pointer
 ├── libraries/
 │    └── <library>.yaml
 └── refs/
      └── <ref-id>.yaml
```

Everything is plain text (YAML).

---

## Core commands

### Initialize

```bash
eln init
```

---

### Add references

#### BibTeX (stdin)

```bash
eln add < references.bib
```

Elaine parses, validates, and stores references atomically.

#### Manual

```bash
eln add "The Satanic Verses" "Rushdie, Salman" 1988
```

#### Interactive

```bash
eln add -i
```

---

### Edit references

```bash
eln edit <ref-selector>
```

Interactive editing with safe ID reconciliation.

---

### Libraries

```bash
eln lib <library>
eln lib
eln lib --delete <library-selector>
```

Deleting a library **never deletes references**.

### Rename libraries

Libraries can be renamed safely without losing references:

```bash
eln lib --rename new_name
```

Renaming:

* updates the library file
* preserves the opaque SID
* keeps all references intact
* updates the active library pointer automatically

---


### Status

```bash
eln status
eln status -v
eln status -vv
eln status --sort year
```

Status provides a hierarchical view of all libraries and their references.

Verbosity levels:

* `eln status`

  * libraries, reference counts, active marker

* `eln status -v`

  * reference IDs
  * short SIDs
  * attachment indicators (`📄`)

* `eln status -vv`

  * title
  * first author
  * year
  * attachments

Sorting:

```bash
eln status --sort id
eln status --sort title
eln status --sort author
eln status --sort year
```

Sorting affects **reference order within each library**.

---

### Pin / unpin

```bash
eln pin <ref> [library]
eln unpin <ref> [library]
```

Unpinned references become **orphaned**, never auto-deleted.

---

### Remove references

```bash
eln rm <ref>
```

Elaine prompts before deleting globally unused references.

---

### Search (external lookup)

```bash
eln search <ref>
```

Search hierarchy:

1. DOI
2. Stored URL
3. Semantic Scholar
4. General web search

Results are **links**, not imports.

---

### Print bibliography

#### 1. Active library

```bash
eln printed
```

Generates a deterministic BibTeX file for the active library:

```
<library>_references.bib
```

The same content is also printed to stdout.

---

#### 2. Multiple libraries (set union)

```bash
eln printed libraryA libraryB
```

Generates a single BibTeX file containing the **union** of references
across the specified libraries:

```
libraryA+libraryB_references.bib
```

If the same reference appears in multiple libraries, it is emitted **once**.

---

#### 3. Global bibliography

```bash
eln printed --all
```

Generates a global bibliography containing **all references across all libraries**:

```
global_references.bib
```

This file is always named explicitly to avoid overwriting curated
library-level bibliographies.


---

## Purge (LaTeX-aware BibTeX minimization)

Elaine can generate minimized `.bib` files based on what your LaTeX project actually cites.

```bash
eln purge <path>
```

Example:

```bash
eln purge .
```

Elaine will:

* Scan all `.tex` files under the given path
* Detect all `\cite{}` keys
* Analyze every `.bib` file in the directory
* Generate new `_purged.bib` files containing **only cited entries**

Original `.bib` files are never modified.

Example output:

```
references.bib → references_purged.bib (kept 8, removed 24)
global_references.bib → global_references_purged.bib (kept 0, removed 0)
```

### Design guarantees

* Non-destructive
* Strict per-file filtering
* Idempotent (running twice produces identical results)
* YAML-agnostic (works without `.elaine`)

This allows Elaine to operate cleanly in:

* Pure `.bib` workflows
* Hybrid `.bib` + `.elaine` projects
* Non-Elaine LaTeX repositories

---

## Example Workflow

```bash
eln init
eln lib machine_learning

eln add < references.bib
eln add "No Free Lunch Theorems" "Wolpert, D.H. and Macready, W.G." 1997
eln add -i

eln status
eln printed

# Multi-library bibliography
eln printed machine_learning background

# Global bibliography (all libraries)
eln printed --all
```

---

## Design principles
Elaine is built around a few non-negotiables:

* **Local-first** — no cloud, no accounts
* **Deterministic output** — same input, same `.bib`
* **Opinionated parsing** — explicit rules, no silent failure
* **Minimal surface area** — fewer commands, fewer flags
* **Researcher-friendly** — works with Git, LaTeX, and editors
* **Explicit scope** — global and multi-library actions are always opt-in
* **No hidden state** — orphaned references are surfaced explicitly

---

## Roadmap

* Attachment metadata (page count, checksum)
* `$EDITOR` integration
* Validation / linting (`eln check`)
* Reference listing / filtering
* Optional metadata enrichment

---

## License

MIT License
