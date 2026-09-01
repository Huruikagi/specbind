# Source material protocol

Source material is request context supplied to help divide work and author the
canonical artifacts. It is not another Requirements, Design, Contract, Roadmap,
or lifecycle authority.

## Capture the complete named collection

Start only from the collection the maintainer explicitly named. Enumerate every
item the provider says belongs to it before classifying any work. Do not select
promising filenames, stop after enough evidence appears, or search nearby paths
for additional likely requirements.

Every item receives a visible disposition. Included items name the work and
Specs they inform. Excluded and duplicate items name why they do not produce
work. An unreadable, unsupported, inaccessible, or unresolved item means the
collection is partial; stop before a Gate invalidation, milestone mutation, or
managed-artifact write. A partial view can hide the boundary that changes every
other disposition.

Provider metadata such as a path, revision, digest, timestamp, label, or state is
provenance and routing evidence. It does not decide Spec ownership, approval, or
semantic truth on its own.

## Preserve provenance where the work lives

The Roadmap explains the complete collection and its decomposition across work
items. Each Spec-backed Brief names the exact subset relevant to that Spec and
why it matters. A shared item is named in every relevant Brief, while the
Roadmap explains the relationship between those Specs. Direct items still have
no Brief.

Keep locators precise enough for the next phase to read the same item. Do not
replace a source reference with a loose summary that hides where the statement
came from. Do not copy the entire source into every Brief; preserve the request
and relevance, not a second uncontrolled edition of the document.

## Promote meaning into canonical artifacts

Requirements and Design read the declared source items as context. A local
Source Item is read again from its exact declared locator; a remote Source Item
uses the Discovery-captured context recorded in the approved Brief and is not
silently re-queried. Every
behavioral obligation accepted from them is restated in complete current
Requirements. Every technical conclusion needed after the milestone is restated
in complete current Design or Contract. A link, citation, source quotation, or
Brief statement is not a substitute for that self-contained meaning.

If a source item conflicts with current approved authority, stop at the owning
phase boundary. Design does not override approved Requirements from a source
document. Contract Review does not accept a missing seam because an external
document describes it. Tasks and implementation consume the canonical
Requirements, Design, and Contract rather than reconstructing decisions from
source material.

Source items and Briefs are not fingerprinted Gate inputs. A later source change
does not silently reinterpret an approval. It returns through an explicit
Discovery request and the ordinary confirmed scope and rewind flow.

## Keep acquisition read-only

Permission to read a source grants no authority to edit, move, convert, comment
on, close, relabel, synchronize, or otherwise mutate it. Provider-specific
instructions may narrow what can be read; they cannot weaken complete coverage,
visible disposition, canonical promotion, or the normal user-confirmation
boundary.
