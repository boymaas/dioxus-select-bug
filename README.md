# Input hydration select bug

While experimenting with the Deoxys platform, I have discovered that
select boxes and forms do not hydrate well. 

After discussing this issue with @ealmloff, a member of the core, we have
decided to isolate this case in a minimal app that showcases the problem
with option select boxes. 

As you can see, when you refresh the page, hydration does not happen correctly
based on the server generated value set on the select field.

The expected behavior is to render option2, but it is currently displaying
option1 instead, disregarding the assigned value.

