# Error description

**Problem**
Wrong DOM node is changed/removed

**Steps To Reproduce**
Steps to reproduce the behavior:
1. Clone https://github.com/cundd/yew-list-problem
2. Run `cargo web start`
3. Click "Add Song 1 to list!"
4. Click "Add Song 2 to list!"
5. Click "Add Song 3 to list!"
6. Click "Add Song 4 to list!"
7. Click "Remove Song 1 from list!"
8. Last node is removed from list

**Expected behavior**
"Song 1" is removed from setlist

**Screenshots**
![Bad state](/error-screenshot.png "Bad state")

**Environment:**
 - Yew version `master`
 - Rust version 1.40.0
 - OS: macos
 - Browser chrome, safari, firefox

**Questionnaire**
<!-- Developing Yew is a community effort! -->
<!-- If you feel up to the challenge, please check one of the boxes below: -->
- [ ] I'm interested in fixing this myself but don't know where to start
- [ ] I would like to fix and I have a solution
- [ ] I don't have time to fix this right now, but maybe later

Possibly related to #604
