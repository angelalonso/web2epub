# web2epub
Script to put together one (or more) ebook(s) directly from a list of websites.

It allows you to clean up the HTML a bit by selecting which DIVs to include, and which ones to remove.  
It also checks if anything changed since last time you created the EPUB file.   
This comes handy when creating an EPUB off a, for instance, News site.

It REQUIRES [Calibre](https://calibre-ebook.com/).

## Motivation
- Calibre itself cannot download from a URL.
- Calibre builds a huge epub from an html unless you tune and tweak for some time. In my example it was a difference of 6 to 260.
- We'll still use Calibre's auto check and correct.
- Other solutions like "Save as ebook" addon for firefox does not work from the command line.

# ATTENTION!
This script has not been properly tested!
Use at your own risk!

I'd totally love to hear from you if my script bricked your ebook reader, though!

ALSO: the binary has been generated on a 64bit linux machine. If your machine cannot run it, have rust installed and try:
```cargo build --release && cp target/release/web2epub .```

# TL;DR
- Install Calibre
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
```./web2epub```
- Your epub docs will have been already generated and you will be presented with calibre. Press <F7> and "Try to correct all fixable errors automatically" until no more show up. Exit. Save
- Copy over the file on ebooks/ to your ebook reader

## Features
- Keep a list of URLs to get content from on YAML format
  - They are grouped by epub document to enable several docs
- Define what contents to "extract" (e.g.: get everything under div tagged as "main")
- Check if the docs have changed (maintain a local copy and compare)
  -  Update only if the docs changed

## NOT YET WORKING
All of the following is yet to be implemented:

- Download and adapt src for images.
- Build a Table of Contents that makes sense.
- Make the program independent from Calibre.
