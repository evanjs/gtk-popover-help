## Issue: Context menu actions seem to accumulate in terms of items selected before launching the context menu

## Procedure
To reproduce this behavior:

1. Launch application
2. Left click on the listbox items a few times
3. Right click on whatever item is selected
4. Click any of the buttons in the resulting context menu

Result: check the console.  You should see a message (e.g. "one" if you clicked "One") that prints out x times, where x is the number of items you clicked/focused on in step 2
