# Vannevar

An attempt to build the simplest possible, text-based, *Memex* implementation.

The memex is an imaginary machine introduced by Vannevar Bush in his 1945
essay *As We May Think*.

The concept is very similar to that of *zettelkasten*, but with the added
feature of *trails*, linear collections of connected notes, useful for making
drafts for longer texts by trailing multiple atomic notes.

## Usage

## Known issues

- On some operating systems, journals and trails might fail to be saved to
  disk unless their `journal` and `trails` subfolders already exist. If this
  is the case, create the subfolders manually.
- The text editing mode doesn't support newlines. When you press return, it's
  displayed as a space. The newlines will be displayed correctly in the view
  mode.
- Trails aren't fully implemented yet.
- Some options in the main menu don't actually lead to any page, the software
  just hangs.
