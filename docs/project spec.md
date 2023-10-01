# Vannevar

An attempt to build the simplest possible, text-based, *Memex* implementation.

The memex is an imaginary machine introduced by Vannevar Bush in his 1945
essay *As We May Think*. In the paper, Bush lays out a set of revolutionary
ideas, which can be summarized as such:

- technology has evolved first to replace manual labor, and partially mental
  labor, but only for specific tasks (mechanical calculators)
- the storage and retrieval of knwoledge hasn't changed in centuries:
  hierarchical or alphabetical encyclopedias are still the main vector of
  compiled human knowledge
- the memex, a desk-sized machine based on coded microfilms, may represent
  knowledge in a more organic way: the storage system will allow more
  organic links between the files, analogously to how a human mind operates,
  instead of forcing a hierarchical structure
- in addition to interwebbed links between documents, linear trails of related
  documents may be created

This essay is regarded as the first instance of the concept of hypertext.

A very similar idea is used in the *zettelkasten* process, where atomic notes
are linked together, both with direct links and backlinks, but the concept of
*trails* is a specificity of the memex hardly found elsewhere.

# Features

The idea is to keep it as basic as possible.

I'm drawing heavy inspiration from Conrad Barski's ZEK
(https://github.com/drcode/zek).

The ideas I'm copying are:

- keeping the project as a CLI application
- having a journal system
- Soviet-like simplicity

The ideas I'm changing are:

- the editing system: I don't use a line editor for anything in my day-to-day
  computer usage, and I'd rather have a vi-like editor than getting familiar
  with a line editor just for the sake of using ZEK

The feature I'm adding are:

- a trails system

# Specs

Storage data structures are plain text files. Links are represented with the
name of the linked file between square brackets:

```
[linked file]
```

The journal entries and the trails are stored in subfolders so that they won't
mix with the main notes. This ensures that the notes will be easy to open
through other editors and the journal entries and trails won't cause any
issue.

## Editor

Editing mode:

- basic vi-like commands

Viewing mode:

- a shell prompt is always available, to control the software

When switching from editing mode to viewing mode, all the non-existing pages
referenced in new links will be created and added to the daily journal entry.

## Journal

Journal entries are stored in a journal subfolder.

The file name represents the day.

The first few lines are an optional, hand-written description of the day.

The remainder of the file stores the pages that have been added during the
current day and the list is updated automatically.

This is the standard format:

```
This is the (optional) description of the journal entry for the day. The date
is contained in the file name, so this starts right from the description.
Below is a line with three dashes, which separates the description from the
list of pages added in the day.
---
[first note added today]
[second note added today]
[last note added today]
```

## Trails

Trails are stored in a trail subfolder.

The trail stores a flat ordered list of page names, with optional comments on
the reason why the pages are connected.

Here is an example of the format:

```
Trail description.
---
[page 1]
(optional comment about page 1)
->
[page 2]
()
->
[page 3]
(optional comment about page 3)
->
[page 4]
()
```

The trail will be scanned until EOF, split at `->` signs and parsed with
precompiled regex.

Comments are inserted between parentheses, which are still present, but empty,
in case of a missing comment.

# MVC Pattern

## Model

The model contains structs to represent:
- currently loaded data:
  - the currently opened note / journal page / trail
  - auxiliary data (while opening a trailed note, the trail itself)
  - the current date (for timestamping journal pages)
  - lists of the available links
- lists of available data on disk
  - notes
  - journal pages
  - trails

## Controller

The controller displays data from the model, keeps a list of available notes
and valid links.

The controller also contains the message enums to communicate with the view.

## View

The view will be a CLI.

# Things yet to fix


Upcoming refactors:

- link dupliction control
- lib -> model

Other features to implement:

- file deletion
- seeing old journal pages

Bugs:

- text editor doesn't display newlines

# Backlinks

Here's how to implement backlinks:

- get a list of all pages
- scan each file
- search for the file name between square brackets