# echoe
An enhanced echo command

This small utility was inspired by an old C program I had written called echof. echof  had the ability to embed hex-encoded bytes in the output from an echo command workalike.  I thought I'd try an alternate approach with a few escape sequences that would permit the inclusion of some common escape sequences for tabs, carriage-returns, and newlines.

If you try the example command:

     echoe "This\tis\na\ttest.\n"

You should see something like:

    This    is
    a       test.
    
