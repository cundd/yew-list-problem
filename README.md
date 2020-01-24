# Wrong DOM node is changed

## Reproduce

1. Click "Add Song 1 to list!"
2. Click "Add Song 2 to list!"
3. Click "Add Song 3 to list!"
4. Click "Add Song 4 to list!"
5. Click "Remove Song 1 from list!"

## Expect

"Song 1" is removed from setlist

## Actual

Last node is removed from list 

![Bad state](/error-screenshot.png "Bad state")

