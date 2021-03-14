# web2epub
Script to put together one (or more) ebook(s) from a list of websites.

It allows you to clean up the HTML by selecting which DIVs to include, and which ones to remove.

# ATTENTION!
This script has not been properly tested!
Use at your own risk!

I'd totally love to hear from you if my script bricked your ebook reader, though!

# TL;DR
- Open the site(s) you want in your ebook with an element inspector(e.g.: element picker on Firefox)
  - Play around a bit to find out the divs you want to capture, and whethere they are defined by class or id (so far nothing else supported)
  - If you feel lost see [tutorials like this](https://www.youtube.com/watch?v=F7fUtZh6APw)
- Copy config.yaml.template to config.yaml.
- Modify the config.yaml. For each book you want to create you'll need to have:
  - '- title:' -> This serves as the title, name of the ebook file...
  - '  items:'
  - '  - url:' -> the URL of the site you want to download
  - '    title:' -> A title that will be included in the ebook for this site
  - '    divs_in:' -> List of divs from the site that you want to add 
  - '    - class:' -> IF the div you want is deifned by a class, define it like this...
  - '    - id:' ->      ... and if the div is defined by an id, do it like this instead.
  - '    divs_out: -> List of divs INSIDE the divs you added that you may want to remove. Similar mechanic as divs_in.
- Run:
```cargo run```
- Your epub docs will have been already generated

## Features
- Keep a list of URLs to get content from on YAML format
  - They are grouped by epub document to enable several docs
- Define what contents to "extract" (e.g.: get everything under div tagged as "main")

## NOT YET WORKING
All of the following is yet to be implemented:

- Check if the docs have changed (maintain a local copy and compare)
  -  Autoupdate if the docs changed
- Download and adapt src for images
